#!/usr/bin/env bun
import { spawnSync } from "node:child_process";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const root = join(dirname(fileURLToPath(import.meta.url)), "..");

const result = spawnSync(
  "cargo",
  ["run", "-p", "wc-cli", "--", ...process.argv.slice(2)],
  { cwd: root, stdio: "inherit" },
);

if (result.error) {
  console.error(
    `Failed to run cargo. Is Rust installed and on PATH? (${result.error.message})`,
  );
  process.exit(1);
}

// cargo run was killed by a signal -> exit 128+signal (matches POSIX shells).
process.exit(result.status ?? (result.signal ? 128 + result.signal : 1));
