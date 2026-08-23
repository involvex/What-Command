import { existsSync, readdirSync } from "node:fs";
import { join, dirname } from "node:path";
import { fileURLToPath } from "node:url";
import { execFileSync, spawnSync } from "node:child_process";

const root = join(dirname(fileURLToPath(import.meta.url)), "..");

const VSWHERE =
  "C:\\Program Files (x86)\\Microsoft Visual Studio\\Installer\\vswhere.exe";

function compareVersions(a, b) {
  const pa = a.split(".").map(Number);
  const pb = b.split(".").map(Number);
  for (let i = 0; i < Math.max(pa.length, pb.length); i += 1) {
    const da = pa[i] ?? 0;
    const db = pb[i] ?? 0;
    if (da !== db) {
      return da - db;
    }
  }
  return 0;
}

function newestDir(base, marker) {
  if (!base || !existsSync(base)) {
    return null;
  }
  const entries = readdirSync(base, { withFileTypes: true })
    .filter((entry) => entry.isDirectory())
    .map((entry) => entry.name)
    .filter((name) => !marker || existsSync(join(base, name, marker)))
    .sort(compareVersions)
    .reverse();
  return entries[0] ? join(base, entries[0]) : null;
}

function vsInstallPath() {
  if (!existsSync(VSWHERE)) {
    return null;
  }
  const out = execFileSync(
    VSWHERE,
    [
      "-latest",
      "-products",
      "*",
      "-requires",
      "Microsoft.VisualStudio.Component.VC.Tools.x86.x64",
      "-property",
      "installationPath",
    ],
    { encoding: "utf8" },
  ).trim();
  return out || null;
}

function windowsSdkRoot() {
  const result = spawnSync(
    "reg",
    [
      "query",
      "HKLM\\SOFTWARE\\Microsoft\\Windows Kits\\Installed Roots",
      "/v",
      "KitsRoot10",
    ],
    { encoding: "utf8" },
  );
  const match = result.stdout?.match(/KitsRoot10\s+REG_SZ\s+(.*)/);
  return match ? match[1].trim() : null;
}

export function hostMsvcEnv() {
  if (process.platform !== "win32") {
    return null;
  }
  if (process.env.INCLUDE && process.env.LIB) {
    return null;
  }

  const vs = vsInstallPath();
  const tools = newestDir(join(vs ?? "", "VC", "Tools", "MSVC"), "bin");
  const sdkRoot = windowsSdkRoot();
  const sdkIncRoot = newestDir(join(sdkRoot ?? "", "Include"), "ucrt");
  const sdkLibRoot = newestDir(join(sdkRoot ?? "", "Lib"), "ucrt");
  if (!tools || !sdkIncRoot || !sdkLibRoot) {
    return null;
  }

  const binDir = join(tools, "bin", "HostX64", "x64");
  const inc = [
    join(tools, "include"),
    join(sdkIncRoot, "ucrt"),
    join(sdkIncRoot, "um"),
    join(sdkIncRoot, "shared"),
    join(sdkIncRoot, "winrt"),
    join(sdkIncRoot, "cppwinrt"),
  ];
  const lib = [
    join(tools, "lib", "x64"),
    join(sdkLibRoot, "ucrt", "x64"),
    join(sdkLibRoot, "um", "x64"),
  ];

  const toolsetVersion = tools.split(/[\\/]/).pop() ?? "";
  const minor = toolsetVersion.split(".")[1] ?? "39";
  const compatFlag = `-fms-compatibility-version=19.${minor}`;

  const clangResourceInclude = resolveClangBuiltinIncludes();
  const bindgenParts = [];
  if (clangResourceInclude) {
    bindgenParts.push(`-resource-dir ${posixPath(join(clangResourceInclude, ".."))}`);
    bindgenParts.push(`-isystem "${posixPath(clangResourceInclude)}"`);
  }
  for (const dir of [inc[0], inc[1], inc[2], inc[3]]) {
    bindgenParts.push(`-isystem "${posixPath(dir)}"`);
  }
  bindgenParts.push(compatFlag);
  const existingBindgenArgs = process.env.BINDGEN_EXTRA_CLANG_ARGS ?? "";
  const bindgenArgs = existingBindgenArgs.includes("-resource-dir")
    ? existingBindgenArgs
    : `${existingBindgenArgs} ${bindgenParts.join(" ")}`.trim();

  return {
    PATH: `${binDir};${process.env.PATH ?? ""}`,
    INCLUDE: inc.join(";"),
    LIB: lib.join(";"),
    LIBPATH: lib.join(";"),
    BINDGEN_EXTRA_CLANG_ARGS: bindgenArgs,
    OPENSSL_ROOT_DIR: join(root, ".openssl-disabled"),
  };
}

function resolveClangBuiltinIncludes() {
  const libClangDir = process.env.LIBCLANG_PATH;
  if (!libClangDir) {
    return null;
  }
  const llvmRoot = dirname(libClangDir.replace(/[\\/]+$/, ""));
  const clangDir = join(llvmRoot, "lib", "clang");
  const version = newestDir(clangDir, "include");
  return version ? join(version, "include") : null;
}

function posixPath(path) {
  return path.replace(/\\/g, "/");
}

export function applyHostMsvcEnv() {
  const env = hostMsvcEnv();
  if (!env) {
    return false;
  }
  Object.assign(process.env, env);
  return true;
}

function isDirectRun() {
  const entry = process.argv[1] ?? "";
  return entry.replace(/\\/g, "/").endsWith("host-msvc-env.mjs");
}

if (isDirectRun()) {
  const applied = applyHostMsvcEnv();
  console.log(applied ? "MSVC env configured" : "MSVC env skipped");

  const [cmd, ...args] = process.argv.slice(2);
  if (!cmd) {
    process.exit(0);
  }

  const result = spawnSync(cmd, args, {
    cwd: root,
    env: process.env,
    stdio: "inherit",
    shell: true,
  });
  process.exit(result.status ?? 1);
}
