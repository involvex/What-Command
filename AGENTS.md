## Learned User Preferences

- Android ships as signed APK only for now; no Play Store release yet
- Mobile app must be full-featured with bottom tabs (Browse, Playground, Research, AI Chat, More), not a lite shell
- Playground uses an in-app simulated terminal; no Termux integration and no real shell execution
- AI default is remote agents (OpenCode Zen, Kilo Gateway); local offline via small GGUF models (e.g. Gemma), not Ollama-first
- Command playground "Try" runs educational simulation only; destructive commands are blocked and copy-to-clipboard is the handoff for real use

## Learned Workspace Facts

- Monorepo stack: Bun workspaces + Rust Cargo workspace + Tauri v2 (desktop + Android) + Vue 3 TypeScript frontend
- Tauri app at `apps/desktop`; bundle identifier `com.involvex.whatcommand`; Rust crates `wc-core`, `wc-ai`, `wc-cli`
- Windows Tauri desktop requires MSVC (`stable-x86_64-pc-windows-msvc` in `rust-toolchain.toml`); GNU/MinGW hits linker export limits
- Tauri `beforeDevCommand`/`beforeBuildCommand` use `scripts/tauri-hook.mjs` with `shell: true` so Windows can resolve `bun`
- Android builds: run `scripts/patch-android-abi.mjs` before dev/build/run; blank `archList`/`abiList` in gradle.properties breaks Gradle JNI merge (patch uses universal fallback + RustPlugin blank fix); use `arm64Debug`/`assembleArm64Debug`, not `assembleDebug` or `armDebug`
- Android scripts: `bun android:dev`, `bun android:run`, `bun android:build`, `bun android:apk` (all run ABI patch first; APK uses `--target aarch64`)
- UI uses Halo design system (`packages/halo`); custom CSS tokens, no Tailwind
- Command DB is SQLite (`data/commands.db`); playground simulation lives in `wc-core` via `simulate_command`
- Root `bun check` runs format, lint:fix, and typecheck; `prebuild` hook runs check before builds
