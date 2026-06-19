import { existsSync, mkdirSync, readdirSync, readFileSync, rmSync } from "fs";
import { join } from "path";
import { $ } from "bun";

const TLDR_REPO = "https://github.com/tldr-pages/tldr.git";
const TLDR_BRANCH = "main";

export async function fetchTldrPages(cacheDir: string): Promise<string> {
  mkdirSync(cacheDir, { recursive: true });
  const pagesDir = join(cacheDir, "pages");

  if (existsSync(join(cacheDir, ".git"))) {
    await $`git -C ${cacheDir} fetch origin ${TLDR_BRANCH} --depth 1`.quiet();
    await $`git -C ${cacheDir} reset --hard origin/${TLDR_BRANCH}`.quiet();
    return pagesDir;
  }

  rmSync(cacheDir, { recursive: true, force: true });
  await $`git clone --depth 1 --branch ${TLDR_BRANCH} --filter=blob:none --sparse ${TLDR_REPO} ${cacheDir}`.quiet();
  await $`git -C ${cacheDir} sparse-checkout set pages`.quiet();
  return pagesDir;
}

export function collectMarkdownFiles(pagesDir: string): string[] {
  const files: string[] = [];

  function walk(dir: string) {
    if (!existsSync(dir)) {
      return;
    }
    for (const entry of readdirSync(dir, { withFileTypes: true })) {
      const full = join(dir, entry.name);
      if (entry.isDirectory()) {
        walk(full);
      } else if (entry.name.endsWith(".md")) {
        files.push(full);
      }
    }
  }

  walk(pagesDir);
  return files;
}

export function readPageFile(absPath: string, _pagesRoot: string): string {
  return readFileSync(absPath, "utf8");
}

export function relPagePath(absPath: string, pagesRoot: string): string {
  return absPath.slice(pagesRoot.length + 1).replace(/\\/g, "/");
}
