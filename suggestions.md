# What Command — Feature Suggestions

> **Context**: Cross-platform command helper with offline SQLite DB (tldr-pages), AI-assisted search (OpenCode Zen, Kilo Gateway, local GGUF), Tauri v2 Desktop + Android. Stack: Rust crates (`wc-core`, `wc-ai`, `wc-cli`), Bun, Vue 3 + TypeScript, Halo design system.

---

## 🎯 High Impact — Core UX Improvements

### 1. Command Palette / Global Search (Cmd/Ctrl+K)

- **What**: A global fuzzy-search overlay accessible from anywhere in the app (and globally via system hotkey on desktop).
- **Why**: Power users expect instant access; reduces navigation friction.
- **Implementation**:
  - Tauri global shortcut plugin → invoke search overlay
  - Fuse.js or custom Rust-side FTS5 ranking for instant results
  - Show: commands, frameworks, recent, favorites, AI suggestions
  - Keyboard navigation (↑/↓, Enter to copy/playground/explain)

### 2. Parameterized Command Templates

- **What**: Commands with `{{placeholders}}` that prompt for values before copy/simulation.
- **Why**: Many commands need user-specific paths, names, IDs (e.g., `docker run -v {{path}}:/data ...`).
- **Implementation**:
  - Extend `Command` model: `params: Param[]` where `Param = { key, label, type: 'string'|'path'|'choice', default?, choices? }`
  - UI: Modal form when template detected; prefill from recent values
  - Playground: Show resolved command with values substituted
  - DB: Store param schema in `commands` table (JSON column)

### 3. Smart History & Frequency Tracking

- **What**: Track every copy/simulation/explain; surface "Recently Used", "Most Used", "Trending This Week".
- **Why**: Reduces search time for repetitive workflows.
- **Implementation**:
  - New table: `command_usage (command_id, action_type, timestamp, platform?)`
  - Tauri command: `record_usage(command_id, action)`
  - Views: "Recents" tab in Browse, "Top Commands" in Research
  - Privacy: Local-only, optional opt-out in settings

### 4. Curated Command Packs (Collections)

- **What**: Shareable, versioned bundles of commands for specific domains (e.g., "Kubernetes Debugging", "React Native Dev", "AWS CLI Essentials").
- **Why**: Onboards new users faster; enables community contributions.
- **Implementation**:
  - New tables: `packs`, `pack_commands`
  - Built-in packs bundled in seed DB
  - Import from URL/GitHub/Gist (JSON manifest)
  - Export favorites as a pack
  - Mobile: Swipe-to-install pack from Research view

### 5. Enhanced AI Chat — Multi-turn & Context

- **What**: Conversational flow where AI remembers previous commands, suggests follow-ups, chains commands.
- **Why**: Real workflows are iterative ("now tail the logs", "filter for errors").
- **Implementation**:
  - Extend `AiContext` with `history: ChatMessage[]`, `currentDirectory?`, `recentCommands?`
  - Provider prompts: Include last 3-5 exchanges + relevant DB commands
  - UI: Threaded view, "Continue" chips under suggestions
  - Local LLM: Smaller context window management

---

## 🔧 Developer Productivity

### 6. Shell Integration & Completions

- **What**: Generate shell completions (bash/zsh/fish/nushell) for `wc` CLI; "Open in Terminal" action from app.
- **Why**: Seamless handoff from GUI to real shell.
- **Implementation**:
  - `wc completions <shell>` → stdout
  - Desktop: `tauri-plugin-shell` → `open` with `wt`, `gnome-terminal`, `Terminal.app`, `konsole`, etc.
  - Android: Intent to Termux (`com.termux.RUN_COMMAND`) — see Plan.md
  - Copy as `alias gc='git commit -m'` for quick alias creation

### 7. Command Chaining / Pipeline Builder

- **What**: Visual builder for pipelines (`cmd1 | cmd2 | cmd3`) with live preview of each stage.
- **Why**: Complex one-liners are hard to construct and remember.
- **Implementation**:
  - New view: "Pipeline" tab
  - Drag-drop commands into sequence
  - Simulate each stage output (mock or AI-predicted)
  - Export as single command or script

### 8. Custom Command Authoring

- **What**: Users add private commands (team aliases, project-specific scripts) with same metadata as DB commands.
- **Why**: Personal/organizational knowledge capture.
- **Implementation**:
  - `source = 'user'` in commands table
  - CRUD UI in More → "My Commands"
  - Markdown description support
  - Sync via export/import (JSON/TOML)

### 9. Project-aware Context

- **What**: Detect current project (git root, package.json, Cargo.toml) and filter/sort commands by relevance.
- **Why**: `docker` commands irrelevant in a pure Rust project.
- **Implementation**:
  - Tauri: `tauri-plugin-fs` → walk up from cwd for markers
  - Pass `projectType: 'rust'|'node'|'python'|'go'|...` to `AiContext`
  - Boost ranking for matching frameworks

---

## 📱 Mobile-First & Android App Polish

### 10. Gesture Navigation & Fluid Transitions

- **What**: Swipe between tabs, pull-to-refresh, long-press context menus, haptic feedback.
- **Why**: Mobile UX expectations differ from desktop.
- **Implementation**:
  - Full-featured bottom tabs (`Browse`, `Playground`, `Research`, `AI Chat`, `More`) with swipe gestures and haptic feedback.
  - `@vueuse/gesture` or native touch events for edge-swipe back and pull-to-refresh.
  - Bottom sheet for command details (instead of full card expand) to maintain mobile ergonomic flow.

### 11. On-Device GGUF Model Selector & Quantization Toggles

- **What**: Seamless management of local GGUF models (`--features local-llm`) with download status, VRAM estimation, and offline fallback.
- **Why**: Empowers offline privacy and zero-cost on-device AI inference on capable Android hardware.
- **Implementation**:
  - `wc-ai` local engine integration with progress streaming via Tauri events.
  - UI modal in Settings / AI Chat to pick quantizations (Q4_K_M, Q5_K_M, Q8_0).
  - Automatic fallback to remote gateways or stub summaries when device memory is constrained.

### 12. Simulated Terminal Enhancements

- **What**: Richer feedback in the educational playground (`wc-core` simulation), highlighting blocked destructive commands and one-tap copy-to-clipboard handoff.
- **Why**: Keeps the Android playground safe while teaching complex command syntax interactively.
- **Implementation**:
  - Enhanced `simulate_command` in `wc-core` returning structured ANSI-colored mock output, risk analysis, and suggested flags.
  - Dedicated "Copy to Clipboard" primary action for executing real commands outside the sandbox.

### 13. Offline-First with Background Sync

- **What**: DB updates download in background; queue AI requests when offline; sync favorites/history when online.
- **Why**: Mobile connectivity is unreliable.
- **Implementation**:
  - WorkManager (Android) / Background Tasks for DB updater
  - Outbox pattern for AI requests: store locally, retry with exponential backoff
  - Conflict resolution for favorites (last-write-wins + merge)

### 14. Widget & Quick Actions

- **What**: Android home screen widget with top 3 favorites; "Ask AI" quick action from launcher.
- **Why**: Zero-tap access to most-used commands.
- **Implementation**:
  - Glance widget (Jetpack Compose) reading from shared preferences
  - App shortcuts for "Search", "AI Chat", "Playground"

---

## 🎨 UI/UX Polish

### 15. Themes & Accessibility

- **What**: Light mode, high contrast, reduced motion, custom accent colors.
- **Why**: Inclusivity; user preference.
- **Implementation**:
  - Halo design tokens already support CSS variables → add `[data-theme="light"]` overrides
  - `prefers-color-scheme`, `prefers-reduced-media`, `prefers-contrast` media queries
  - Settings: Theme selector, accent picker (indigo/cyan/amber/magenta)

### 16. Onboarding & Interactive Tutorial

- **What**: First-run flow: pick AI provider, import GGUF, try playground, learn shortcuts.
- **Why**: Reduces drop-off; teaches power features.
- **Implementation**:
  - `driver.js` or custom stepper component
  - Persistent "Tips" panel (dismissible)
  - Keyboard shortcut cheat sheet (Cmd+/)

### 17. Command Detail Drill-down

- **What**: Tap command → full screen with: description, platforms, danger, examples, related, AI explain, user notes.
- **Why**: Current card is truncated; power users want depth.
- **Implementation**:
  - New route: `/command/:id`
  - Tabs: Overview | Examples | Related | Notes
  - Markdown rendering for rich descriptions
  - "Open in browser" for tldr upstream page

---

## 🧠 AI & Intelligence

### 18. Local Hybrid Search (FTS5 + Vector Embeddings)

- **What**: Combine SQLite FTS5 keyword search with lightweight ONNX/candle vector embeddings for semantic command discovery without cloud dependency.
- **Why**: Keyword search misses intent ("how to find big files" ≠ `find -size`).
- **Implementation**:
  - `wc-core`: Add embedding storage and scoring logic.
  - `wc-ai`: `LocalEmbeddingProvider` using fastembed-rust / candle.
  - Search: Hybrid ranking balancing BM25 text match with cosine similarity.

### 19. Context-Aware Multi-Turn Prompt Chaining

- **What**: Conversational AI chat retaining recent command context, directory/project type detection (`package.json`, `Cargo.toml`), and structured follow-up suggestions.
- **Why**: Real workflows are iterative and project-specific.
- **Implementation**:
  - Extend `AiContext` with conversation history, workspace metadata, and active project markers.
  - Provider prompts injecting relevant command snippets automatically.
  - UI chips for quick follow-up actions ("explain this flag", "add to pipeline").

### 20. Natural Language → Pipeline

- **What**: "Find all .log files older than 7 days, compress them, upload to S3" → multi-step pipeline.
- **Why**: Complex tasks need composition.
- **Implementation**:
  - Specialized prompt / fine-tuned model
  - Output: Array of `CommandSuggestion` with `dependsOn` links
  - Playground: "Run Pipeline" button with step-by-step simulation

---

## 📊 Data & Extensibility

### 21. Additional Data Sources

- **What**: Ingest `cheat.sh`, `eg`, `navi`, man pages, GitHub READMEs, custom YAML.
- **Why**: tldr covers ~2000 commands; ecosystem has 10x more.
- **Implementation**:
  - `scripts/db-updater`: Modular source adapters (each source = TypeScript class)
  - Normalize to common schema before insert
  - Source attribution preserved (`source` column)
  - CI: Weekly GitHub Action for each source

### 22. Plugin / Extension API

- **What**: Third-party command sources, AI providers, UI panels via WASM or dynamic import.
- **Why**: Community-driven growth without core bloat.
- **Implementation**:
  - Tauri: `tauri-plugin-shell` sidecar or `tauri-plugin-wasm` (future)
  - Manifest: `what-command-plugin.json` with `commands`, `providers`, `views`
  - Security: Capability-based permissions (read DB, network, fs)

### 23. Telemetry (Opt-in, Anonymous)

- **What**: Aggregate usage: top searches, AI provider success rates, simulation coverage.
- **Why**: Data-driven prioritization.
- **Implementation**:
  - `tauri-plugin-opentelemetry` or custom minimal beacon
  - No PII; hashed command IDs only
  - User toggle in Settings → "Help improve What Command"

---

## ⚡ Performance & Reliability

### 24. Virtualized Lists & Incremental Search

- **What**: Render only visible commands; debounced search with loading skeleton.
- **Why**: 5000+ commands → jank on low-end devices.
- **Implementation**:
  - `@tanstack/vue-virtual` or custom
  - Rust: `search_commands` returns `totalCount` + `page` for pagination
  - Mobile: Critical for 60fps scrolling

### 25. DB Optimizations

- **What**: FTS5 triggers for auto-rebuild; partial indexes; WAL mode; connection pooling.
- **Why**: Faster searches, safer concurrent access.
- **Implementation**:
  - `PRAGMA journal_mode=WAL; PRAGMA busy_timeout=5000;`
  - FTS5 `triggers` on `commands` insert/update/delete
  - Index on `(category, danger_level)` for Research view

### 26. Error Boundaries & Graceful Degradation

- **What**: UI error boundaries; AI fallback chain; offline banner; corrupted DB recovery.
- **Why**: Production resilience.
- **Implementation**:
  - Vue `onErrorCaptured` + global error modal
  - AI Router: `opencode_zen → kilo_gateway → local_llm → stub`
  - DB: `integrity_check` on startup; auto-repair from seed

---

## 🧪 Testing & Quality

### 27. E2E Test Suite (Playwright)

- **What**: Critical flows: search→copy, AI ask→playground, settings persist, Android APK smoke test.
- **Why**: Prevent regressions across platforms.
- **Implementation**:
  - `webapp-testing` skill → Playwright config
  - CI: `bun run test:e2e` on desktop headless; Android emulator in GitHub Actions

### 28. Property-Based Testing (Rust)

- **What**: `proptest` for simulator, parser, danger detection.
- **Why**: Edge cases in command parsing are infinite.
- **Implementation**:
  - `cargo test --proptest` in `wc-core`
  - Fuzz: arbitrary command strings → no panic, correct danger level

---

## 📦 Distribution & Growth

### 29. Package Manager Distribution

- **What**: Homebrew (`brew install what-command`), Scoop, Chocolatey, AUR, Nix, Cargo (`cargo install wc-cli`).
- **Why**: Discoverability; standard install paths.
- **Implementation**:
  - CI: `tauri-action` → artifacts → `gh release` → formula generators
  - `wc-cli` crate published to crates.io

### 30. F-Droid / IzzyOnDroid (Android)

- **What**: Reproducible builds, open-source distribution.
- **Why**: Plan.md mentions F-Droid; privacy-conscious users.
- **Implementation**:
  - `fdroidserver` metadata YAML
  - Reproducible build: pinned deps, no timestamps
  - Gradle: `signingConfig` for F-Droid key

### 31. Auto-update (Desktop)

- **What**: Tauri Updater integration; delta updates; release channels (stable/beta/nightly).
- **Why**: Seamless updates without GitHub Releases manual download.
- **Implementation**:
  - `tauri-plugin-updater` + GitHub Releases endpoint
  - `tauri.conf.json`: `updater.active = true`, `pubkey`
  - Settings: Channel selector, "Check now" button

---

## 🗂 Suggested Prioritization

| Priority              | Features               | Rationale                                      |
| --------------------- | ---------------------- | ---------------------------------------------- |
| **P0 (Now)**          | 1, 2, 3, 6, 24, 26     | Core UX, developer workflow, perf, reliability |
| **P1 (Next)**         | 4, 5, 7, 15, 16, 17    | Power features, accessibility, onboarding      |
| **P2 (Mobile)**       | 10, 11, 12, 13, 14     | Android parity, offline-first, local GGUF      |
| **P3 (Intelligence)** | 18, 19, 20             | Differentiation via hybrid RAG & AI chaining   |
| **P4 (Ecosystem)**    | 21, 22, 23, 29, 30, 31 | Growth, distribution, community                |

---

## 💡 Quick Wins (Weekend Projects)

1. **Copy as `alias`** — One-liner in `CommandCard` actions
2. **Keyboard Shortcut Cheat Sheet** — `Cmd+/` overlay with all bindings
3. **Recent Commands in Browse** — Top 5 from `command_usage` table
4. **Danger Level Tooltip** — Hover badge → "Level 2: Potentially destructive"
5. **Search Result Count** — "42 commands found" in Browse header
6. **Copy Command + Explanation** — "Copy with context" for sharing
7. **Theme Toggle in Header** — Sun/moon icon, persists to localStorage
8. **Reduce Motion Toggle** — Respects `prefers-reduced-motion`, disables transitions

---

## 📝 Notes for Implementers

- **Rust-first**: Heavy logic (search, parsing, simulation, AI routing) stays in `wc-core`/`wc-ai` for CLI + Desktop + Android parity.
- **Capability-based**: Tauri capabilities (`tauri.conf.json`) — grant minimal permissions per feature.
- **Offline-by-default**: Every feature works without network; AI is enhancement, not requirement.
- **Design System**: Extend `packages/halo` tokens; never hardcode colors/spacing.
- **Testing**: `bun run check` (format, lint, typecheck) must pass before PR; add tests for new Tauri commands.

---

_Updated from codebase analysis & roadmap expansion — 2026-08-21_
