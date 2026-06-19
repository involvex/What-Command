import { existsSync } from "node:fs";
import { spawnSync } from "node:child_process";
import { join } from "node:path";

const pkg = "com.involvex.whatcommand";

function sh(args) {
  const result = spawnSync("adb", args, { encoding: "utf8" });
  return {
    ok: result.status === 0,
    out: (result.stdout ?? "").trim(),
    err: (result.stderr ?? "").trim(),
  };
}

const abi = sh(["shell", "getprop", "ro.product.cpu.abi"]).out;
const abilist = sh(["shell", "getprop", "ro.product.cpu.abilist"]).out;
const sdk = sh(["shell", "getprop", "ro.build.version.sdk"]).out;
const pageSize = sh(["shell", "getconf", "PAGESIZE"]).out;
const installed = sh(["shell", "pm", "path", pkg]);
const dumpsys = sh(["shell", "dumpsys", "package", pkg]);
const primaryCpuAbi = dumpsys.out.match(/primaryCpuAbi=([^\s]+)/)?.[1] ?? "unknown";

const localAppData = process.env.LOCALAPPDATA ?? "";
const ndkDir = localAppData ? join(localAppData, "Android", "Sdk", "ndk") : "";
let ndkVersions = [];
if (ndkDir && existsSync(ndkDir)) {
  ndkVersions = spawnSync("cmd", ["/c", "dir", "/b", ndkDir], {
    encoding: "utf8",
  })
    .stdout.split(/\r?\n/)
    .map((s) => s.trim())
    .filter(Boolean);
}

console.log("Android compatibility check");
console.log(`  Phone ABI: ${abi || "none"} (${abilist || "adb offline"})`);
console.log(`  Phone SDK: ${sdk || "?"}  page size: ${pageSize || "?"}`);
console.log(`  Installed app: ${installed.ok ? primaryCpuAbi : "not installed"}`);
console.log(
  `  NDK versions: ${ndkVersions.length ? ndkVersions.join(", ") : "none found"}`,
);
