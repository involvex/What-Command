#!/usr/bin/env bun
/**
 * Builds data/commands.db from tldr-pages/tldr (GitHub) with demo fallbacks.
 */
import { Database } from "bun:sqlite";
import { mkdirSync } from "fs";
import { join } from "path";
import {
  collectMarkdownFiles,
  fetchTldrPages,
  readPageFile,
  relPagePath,
} from "./fetch-tldr";
import { dangerLevel, inferFramework, parseTldrPage } from "./parse-tldr";

const root = join(import.meta.dir, "../..");
const outPath = join(root, "data/commands.db");
const cacheDir = join(root, ".cache/tldr-pages");
const pagesRoot = join(cacheDir, "pages");

mkdirSync(join(root, "data"), { recursive: true });

const db = new Database(outPath);
db.run("PRAGMA foreign_keys = ON");

db.run(`CREATE TABLE IF NOT EXISTS commands (
  id TEXT PRIMARY KEY,
  command TEXT NOT NULL,
  description TEXT,
  category TEXT,
  platform TEXT,
  danger_level INTEGER DEFAULT 0,
  source TEXT,
  updated_at TEXT
)`);

db.run(`CREATE VIRTUAL TABLE IF NOT EXISTS commands_fts USING fts5(
  command, description, content='commands', content_rowid='rowid'
)`);

db.run(`CREATE TABLE IF NOT EXISTS frameworks (
  id TEXT PRIMARY KEY,
  name TEXT NOT NULL,
  description TEXT,
  icon TEXT
)`);

db.run(`CREATE TABLE IF NOT EXISTS command_frameworks (
  command_id TEXT,
  framework_id TEXT,
  PRIMARY KEY (command_id, framework_id)
)`);

db.run(`CREATE TABLE IF NOT EXISTS playground_sessions (
  id TEXT PRIMARY KEY,
  command TEXT NOT NULL,
  transcript TEXT,
  updated_at TEXT
)`);

db.run("DELETE FROM command_frameworks");
db.run("DELETE FROM commands");
db.run("DELETE FROM frameworks");

const insertCmd = db.prepare(
  `INSERT OR REPLACE INTO commands
   (id, command, description, category, platform, danger_level, source, updated_at)
   VALUES (?, ?, ?, ?, ?, ?, ?, datetime('now'))`,
);

const insertFw = db.prepare(
  `INSERT OR IGNORE INTO frameworks (id, name, description, icon) VALUES (?, ?, ?, ?)`,
);

const link = db.prepare(
  `INSERT OR IGNORE INTO command_frameworks (command_id, framework_id) VALUES (?, ?)`,
);

let pageCount = 0;
let commandCount = 0;

try {
  console.log("Fetching tldr-pages/tldr from GitHub…");
  await fetchTldrPages(cacheDir);
  const files = collectMarkdownFiles(pagesRoot);
  console.log(`Parsing ${files.length} tldr pages…`);

  const insertMany = db.transaction(() => {
    for (const absPath of files) {
      const rel = relPagePath(absPath, pagesRoot);
      const content = readPageFile(absPath, pagesRoot);
      const parsed = parseTldrPage(content, rel);
      if (!parsed) {
        continue;
      }
      pageCount += 1;

      const fw = inferFramework(parsed.examples[0]?.command ?? "", parsed.category);
      insertFw.run(fw.id, fw.name, fw.description, fw.icon);

      parsed.examples.forEach((ex, idx) => {
        const cmdId =
          parsed.examples.length === 1 ? parsed.id : `${parsed.id}-${idx + 1}`;
        const danger = dangerLevel(ex.command);
        insertCmd.run(
          cmdId,
          ex.command,
          ex.description,
          parsed.category,
          JSON.stringify(parsed.platform),
          danger,
          "tldr",
        );
        link.run(cmdId, fw.id);
        commandCount += 1;
      });
    }
  });

  insertMany();
} catch (err) {
  console.warn("tldr fetch failed, using demo seed:", err);
  seedDemos();
}

function seedDemos() {
  const demos = [
    {
      id: "git-status",
      command: "git status",
      description: "Show working tree status",
      category: "git",
      platform: '["common"]',
      danger: 0,
      fw: {
        id: "git",
        name: "Git",
        description: "Version control",
        icon: "git-branch",
      },
    },
    {
      id: "docker-ps",
      command: "docker ps",
      description: "List running containers",
      category: "docker",
      platform: '["linux","osx"]',
      danger: 0,
      fw: {
        id: "docker",
        name: "Docker",
        description: "Containers",
        icon: "container",
      },
    },
    {
      id: "find-large",
      command: "find . -size +100M",
      description: "Find files larger than 100MB",
      category: "files",
      platform: '["linux","osx"]',
      danger: 0,
      fw: { id: "bash", name: "Bash", description: "Shell", icon: "terminal" },
    },
    {
      id: "npm-install",
      command: "npm install",
      description: "Install package dependencies",
      category: "npm",
      platform: '["common"]',
      danger: 0,
      fw: { id: "npm", name: "npm", description: "Node packages", icon: "package" },
    },
    {
      id: "kubectl-get-pods",
      command: "kubectl get pods",
      description: "List pods in current namespace",
      category: "kubernetes",
      platform: '["common"]',
      danger: 0,
      fw: {
        id: "kubernetes",
        name: "Kubernetes",
        description: "Orchestration",
        icon: "layers",
      },
    },
    {
      id: "rm-rf-root",
      command: "rm -rf /",
      description: "Never run — removes entire filesystem",
      category: "destructive",
      platform: '["linux"]',
      danger: 3,
      fw: { id: "bash", name: "Bash", description: "Shell", icon: "terminal" },
    },
  ];

  for (const d of demos) {
    insertCmd.run(
      d.id,
      d.command,
      d.description,
      d.category,
      d.platform,
      d.danger,
      "tldr",
    );
    insertFw.run(d.fw.id, d.fw.name, d.fw.description, d.fw.icon);
    link.run(d.id, d.fw.id);
    commandCount += 1;
  }
  pageCount = demos.length;
}

try {
  db.run("INSERT INTO commands_fts(commands_fts) VALUES('rebuild')");
} catch {
  // FTS rebuild optional when external-content triggers are absent
}

db.run("PRAGMA optimize");
db.run("VACUUM");

const total: number = db.query("SELECT COUNT(*) AS c FROM commands").get()!.c as number;

console.log(
  `Wrote ${outPath} — ${total} commands from ${pageCount} tldr pages (${commandCount} examples inserted)`,
);
