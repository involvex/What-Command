# What Command

Cross-platform command helper: offline SQLite command DB (tldr-pages), AI-assisted search (OpenCode Zen, Kilo Gateway, local GGUF via llama.cpp), Tauri v2 desktop + Android.

## Stack

- **Rust** — `wc-core`, `wc-ai`, `wc-cli`, Tauri backend
- **Bun** — workspace tooling, `scripts/db-updater`
- **Vue 3 + TypeScript** — Tauri frontend with Halo design system
- **Identifier** — `com.involvex.whatcommand`

## Prerequisites

- [Bun](https://bun.sh)
- Rust stable — **pinned to MSVC on Windows** via `rust-toolchain.toml` (required for Tauri desktop; MinGW/GNU hits `export ordinal too large`)
- For Android: Android Studio / SDK, then `bun tauri android init` in `apps/desktop`

## Commands

```bash
bun install
bun run db:update          # build data/commands.db
bun run dev                # Tauri desktop dev
bun run android:dev        # Android dev (after android init)
bun run android:apk        # debug/release APK with local-llm (on-device GGUF)
bun run dev:local          # desktop dev with local-llm
bun run typecheck
bun run lint
cargo build -p wc-cli      # CLI binary `wc`
cargo run -p wc-cli -- search git
```

## CLI (`wc`)

```bash
wc search "docker"
wc ask "find large files"
wc explain "kubectl get pods"
wc update
```

For local development, `bun link` exposes `wc` globally in dev mode (runs via `cargo run`, so source changes are picked up automatically). Remove it with `bun unlink`.

### Configuration

`wc settings` manages `~/.config/what-command/config.toml` (AI providers, API keys, models):

```bash
wc settings                  # same as 'wc settings list'
wc settings list              # summarize settings (secrets masked)
wc settings list --json       # JSON output (secrets masked; add --raw to include them)
wc settings show ai_provider  # print one value (--raw to reveal a secret)
wc settings set ai_provider local_llm      # aliases: opencode|kilo|local|openai
wc settings set opencode_api_key sk-...    # secrets are masked in output
wc settings set ai_model null             # empty/"null" clears the key
wc settings edit            # open the config file in $EDITOR
wc settings reset           # restore defaults
wc settings env             # show env vars (OPENCODE_API_KEY, KILO_API_KEY, ...) that override config
wc config path              # print config.toml path
wc config dir               # print config directory
```

Env vars `OPENCODE_API_KEY`, `KILO_API_KEY`, `LOCAL_GGUF_PATH`, `OPENAI_COMPAT_BASE_URL`, and `OPENAI_COMPAT_API_KEY` take precedence over `config.toml`.

Set `OPENCODE_API_KEY` or `KILO_API_KEY` (see `.env.example`) for live AI. For on-device GGUF inference, pick a `.gguf` file in Settings (copied to app storage on Android) and build with the `local-llm` feature — included in `bun run android:apk` and `bun run dev:local`. Host `cargo build --features local-llm` does **not** affect the Android APK. Use `bun run android:apk` (sets `ANDROID_NDK`, bindgen sysroot, and `local-llm`) or wrap manual builds with `node scripts/android-ndk-env.mjs …`.

## Project layout

```
apps/desktop/     Tauri + Vue app
crates/wc-core/   SQLite DB, simulator, settings
crates/wc-ai/     AI provider adapters
crates/wc-cli/    CLI binary
packages/halo/    Design system CSS
scripts/db-updater/
data/commands.db  Bundled seed database
```

## Mobile app

Five tabs: Browse, Playground (simulated terminal), Research, AI Chat, More. APK-only distribution via GitHub Releases (no Play Store in current scope).
