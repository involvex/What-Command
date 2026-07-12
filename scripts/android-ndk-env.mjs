import { existsSync, readdirSync } from "node:fs";
import { join, dirname } from "node:path";
import { fileURLToPath } from "node:url";
import { spawnSync } from "node:child_process";

const root = join(dirname(fileURLToPath(import.meta.url)), "..");

function ndkHostTag() {
  if (process.platform === "win32") {
    return "windows-x86_64";
  }
  if (process.platform === "darwin") {
    return process.arch === "arm64" ? "darwin-arm64" : "darwin-x86_64";
  }
  return "linux-x86_64";
}

function resolveNdkHome() {
  for (const key of ["ANDROID_NDK", "NDK_HOME"]) {
    const value = process.env[key];
    if (value && existsSync(value)) {
      return value;
    }
  }

  const sdk = process.env.ANDROID_HOME || process.env.ANDROID_SDK_ROOT;
  if (!sdk) {
    return null;
  }

  const ndkRoot = join(sdk, "ndk");
  if (!existsSync(ndkRoot)) {
    return null;
  }

  const versions = readdirSync(ndkRoot, { withFileTypes: true })
    .filter((entry) => entry.isDirectory())
    .map((entry) => entry.name)
    .filter((name) => existsSync(join(ndkRoot, name, "source.properties")))
    .sort()
    .reverse();

  return versions[0] ? join(ndkRoot, versions[0]) : null;
}

function posixPath(path) {
  return path.replace(/\\/g, "/");
}

function clangResourceDir(prebuilt) {
  const clangRoot = join(prebuilt, "lib/clang");
  if (!existsSync(clangRoot)) {
    return null;
  }
  const versions = readdirSync(clangRoot, { withFileTypes: true })
    .filter((entry) => entry.isDirectory())
    .map((entry) => entry.name)
    .sort((a, b) => Number.parseFloat(b) - Number.parseFloat(a));
  return versions[0] ? join(clangRoot, versions[0]) : null;
}

/** @param {string} ndk @param {number} api */
export function androidNdkEnv(ndk, api = 30) {
  const host = ndkHostTag();
  const prebuilt = join(ndk, "toolchains/llvm/prebuilt", host);
  const sysroot = join(prebuilt, "sysroot");
  const libDir = join(prebuilt, "lib");
  const binDir = join(prebuilt, "bin");
  const ext = process.platform === "win32" ? ".cmd" : "";
  const resourceDir = clangResourceDir(prebuilt);

  const bindgenArgs = [
    `--sysroot=${posixPath(sysroot)}`,
    `-isystem ${posixPath(join(sysroot, "usr/include"))}`,
    `-isystem ${posixPath(join(sysroot, "usr/include/c++/v1"))}`,
  ];
  if (resourceDir) {
    bindgenArgs.push(`-resource-dir=${posixPath(resourceDir)}`);
    bindgenArgs.push(`-isystem ${posixPath(join(resourceDir, "include"))}`);
  }

  const targets = {
    aarch64: `aarch64-linux-android${api}`,
    armv7: `armv7a-linux-androideabi${api}`,
    i686: `i686-linux-android${api}`,
    x86_64: `x86_64-linux-android${api}`,
  };

  const env = {
    ANDROID_NDK: ndk,
    NDK_HOME: ndk,
    LIBCLANG_PATH: libDir,
    BINDGEN_EXTRA_CLANG_ARGS: bindgenArgs.join(" "),
  };

  const llvmAr =
    process.platform === "win32"
      ? join(binDir, "llvm-ar.exe")
      : join(binDir, "llvm-ar");
  const llvmRanlib =
    process.platform === "win32"
      ? join(binDir, "llvm-ranlib.exe")
      : join(binDir, "llvm-ranlib");

  for (const [rustTarget, triple] of [
    ["aarch64_linux_android", targets.aarch64],
    ["armv7_linux_androideabi", targets.armv7],
    ["i686_linux_android", targets.i686],
    ["x86_64_linux_android", targets.x86_64],
  ]) {
    const clang = join(binDir, `${triple}-clang${ext}`);
    const clangxx = join(binDir, `${triple}-clang++${ext}`);
    env[`CC_${rustTarget}`] = clang;
    env[`CXX_${rustTarget}`] = clangxx;
    env[`AR_${rustTarget}`] = llvmAr;
    env[`RANLIB_${rustTarget}`] = llvmRanlib;
    env[`CARGO_TARGET_${rustTarget.toUpperCase()}_LINKER`] = clang;
    env[`CARGO_TARGET_${rustTarget.toUpperCase()}_AR`] = llvmAr;
  }

  return env;
}

export function applyAndroidNdkEnv(api = 30) {
  const ndk = resolveNdkHome();
  if (!ndk) {
    throw new Error(
      "Android NDK not found. Set ANDROID_NDK or NDK_HOME, or install NDK via Android SDK.",
    );
  }
  Object.assign(process.env, androidNdkEnv(ndk, api));
  return ndk;
}

function detectAndroidTriple(argv, targets) {
  const flagIndex = argv.findIndex((arg) => arg === "--target");
  if (flagIndex >= 0 && argv[flagIndex + 1]) {
    const map = {
      aarch64: targets.aarch64,
      armv7: targets.armv7,
      i686: targets.i686,
      x86_64: targets.x86_64,
    };
    return map[argv[flagIndex + 1]] ?? targets.aarch64;
  }
  return targets.aarch64;
}

function isDirectRun() {
  const entry = process.argv[1] ?? "";
  return entry.replace(/\\/g, "/").endsWith("android-ndk-env.mjs");
}

if (isDirectRun()) {
  const ndk = resolveNdkHome();
  if (!ndk) {
    console.error(
      "Android NDK not found. Set ANDROID_NDK or NDK_HOME, or install NDK via Android SDK.",
    );
    process.exit(1);
  }

  const childArgs = process.argv.slice(2);
  const env = androidNdkEnv(ndk);
  const host = ndkHostTag();
  const prebuilt = join(ndk, "toolchains/llvm/prebuilt", host);
  const binDir = join(prebuilt, "bin");
  const ext = process.platform === "win32" ? ".cmd" : "";
  const api = 30;
  const targets = {
    aarch64: `aarch64-linux-android${api}`,
    armv7: `armv7a-linux-androideabi${api}`,
    i686: `i686-linux-android${api}`,
    x86_64: `x86_64-linux-android${api}`,
  };
  const triple = detectAndroidTriple(childArgs, targets);
  env.CLANG_PATH = join(binDir, `${triple}-clang${ext}`);
  delete env.BINDGEN_EXTRA_CLANG_ARGS;
  for (const rustTarget of [
    "aarch64_linux_android",
    "armv7_linux_androideabi",
    "i686_linux_android",
    "x86_64_linux_android",
  ]) {
    env[`BINDGEN_EXTRA_CLANG_ARGS_${rustTarget}`] =
      `-D__ANDROID_API__=${api} --target=${triple}`;
  }

  Object.assign(process.env, env);
  console.log(`Android NDK env: ${ndk} (clang: ${triple})`);

  if (childArgs.length === 0) {
    process.exit(0);
  }

  const [cmd, ...args] = childArgs;
  const result = spawnSync(cmd, args, {
    cwd: root,
    env: process.env,
    stdio: "inherit",
    shell: true,
  });
  process.exit(result.status ?? 1);
}
