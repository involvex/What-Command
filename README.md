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
bun run android:apk        # release APK
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

Set `OPENCODE_API_KEY` or `KILO_API_KEY` (see `.env.example`) for live AI. For on-device GGUF inference, place a model in `~/.config/what-command/models/` and build with `cargo build --features local-llm`.

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
