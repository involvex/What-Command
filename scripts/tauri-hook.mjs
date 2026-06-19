import { spawnSync } from "node:child_process";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const task = process.argv[2] ?? "build";
const root = join(dirname(fileURLToPath(import.meta.url)), "..");
const desktopDir = join(root, "apps/desktop");

const result = spawnSync("bun", ["run", task], {
  cwd: desktopDir,
  stdio: "inherit",
  shell: true,
});

process.exit(result.status ?? 1);
