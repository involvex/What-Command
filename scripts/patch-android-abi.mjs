import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { spawnSync } from "node:child_process";
import { join, dirname } from "node:path";
import { fileURLToPath } from "node:url";

const root = join(dirname(fileURLToPath(import.meta.url)), "..");
const gradleProps = join(root, "apps/desktop/src-tauri/gen/android/gradle.properties");
const rustPlugin = join(
  root,
  "apps/desktop/src-tauri/gen/android/buildSrc/src/main/java/com/involvex/whatcommand/kotlin/RustPlugin.kt",
);

const UNIVERSAL_PROFILE = {
  archList: "arm64,arm,x86,x86_64",
  abiList: "arm64-v8a,armeabi-v7a,x86,x86_64",
  targetList: "aarch64,armv7,i686,x86_64",
  variant: "universalDebug",
};

function adbProp(name) {
  const result = spawnSync("adb", ["shell", "getprop", name], {
    encoding: "utf8",
  });
  return result.status === 0 ? result.stdout.trim() : "";
}

function pickProfile(primaryAbi, abiList) {
  const normalized = primaryAbi.toLowerCase();
  if (normalized.includes("arm64") || normalized === "aarch64") {
    return {
      archList: "arm64",
      abiList: "arm64-v8a",
      targetList: "aarch64",
      variant: "arm64Debug",
    };
  }
  if (normalized.includes("armeabi") || normalized === "armeabi-v7a") {
    return {
      archList: "arm",
      abiList: "armeabi-v7a",
      targetList: "armv7",
      variant: "armDebug",
    };
  }
  if (normalized.includes("x86_64")) {
    return {
      archList: "x86_64",
      abiList: "x86_64",
      targetList: "x86_64",
      variant: "x86_64Debug",
    };
  }
  if (normalized.includes("x86") || normalized.includes("i686")) {
    return {
      archList: "x86",
      abiList: "x86",
      targetList: "i686",
      variant: "x86Debug",
    };
  }
  if (abiList.length > 0) {
    return {
      archList: abiList.join(","),
      abiList: abiList.join(","),
      targetList: UNIVERSAL_PROFILE.targetList,
      variant: "universalDebug",
    };
  }
  return UNIVERSAL_PROFILE;
}

function patchRustPlugin() {
  if (!existsSync(rustPlugin)) {
    return;
  }

  let content = readFileSync(rustPlugin, "utf8");
  const replacements = [
    [
      "(findProperty(\"abiList\") as? String)?.split(',')",
      "(findProperty(\"abiList\") as? String)?.takeIf { it.isNotBlank() }?.split(',')",
    ],
    [
      "(findProperty(\"archList\") as? String)?.split(',')",
      "(findProperty(\"archList\") as? String)?.takeIf { it.isNotBlank() }?.split(',')",
    ],
    [
      "(findProperty(\"targetList\") as? String)?.split(',')",
      "(findProperty(\"targetList\") as? String)?.takeIf { it.isNotBlank() }?.split(',')",
    ],
  ];

  let changed = false;
  for (const [from, to] of replacements) {
    if (content.includes(from) && !content.includes(to)) {
      content = content.replace(from, to);
      changed = true;
    }
  }

  if (changed) {
    writeFileSync(rustPlugin, content);
    console.log("Patched RustPlugin.kt to ignore blank gradle.properties ABI keys");
  }
}

if (!existsSync(gradleProps)) {
  console.error(
    "Android project not found. Run: bun run --filter desktop tauri android init",
  );
  process.exit(1);
}

const primaryAbi = adbProp("ro.product.cpu.abi");
const abiListRaw = adbProp("ro.product.cpu.abilist");

const deviceAbis = abiListRaw
  ? abiListRaw
      .split(",")
      .map((s) => s.trim())
      .filter(Boolean)
  : primaryAbi
    ? [primaryAbi]
    : [];

const profile = pickProfile(primaryAbi || deviceAbis[0] || "", deviceAbis);

const keys = {
  archList: profile.archList,
  abiList: profile.abiList,
  targetList: profile.targetList,
};
const marker = "# wc-android-abi (auto — do not edit by hand)";
let content = readFileSync(gradleProps, "utf8");
content = content
  .split("\n")
  .filter((line) => !line.startsWith("archList="))
  .filter((line) => !line.startsWith("abiList="))
  .filter((line) => !line.startsWith("targetList="))
  .filter((line) => !line.startsWith(marker))
  .join("\n")
  .trimEnd();

const block = [
  "",
  marker,
  `archList=${keys.archList}`,
  `abiList=${keys.abiList}`,
  `targetList=${keys.targetList}`,
  "",
].join("\n");

writeFileSync(gradleProps, `${content}${block}`);
patchRustPlugin();

console.log(
  `Android ABI profile: ${keys.abiList} (use ${profile.variant} in Android Studio if prompted)`,
);
