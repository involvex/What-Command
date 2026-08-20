# What Command — Implementation Plan

> **Goal**: Prioritized feature roadmap by difficulty (effort) and user/business needs. Each phase delivers shippable value.

---

## 📊 Prioritization Matrix

| Feature                         | Difficulty | User Need   | Business Value         | Phase        |
| ------------------------------- | ---------- | ----------- | ---------------------- | ------------ |
| **1. Command Palette (Cmd+K)**  | Medium     | ⭐⭐⭐ High | Core UX differentiator | P0 (✅ Done) |
| **2. Parameterized Templates**  | Medium     | ⭐⭐ High   | Power user retention   | P0 (✅ Done) |
| **3. Smart History/Usage**      | Low        | ⭐⭐ High   | Daily workflow speed   | P0 (✅ Done) |
| **4. Command Packs**            | Medium     | ⭐⭐ Medium | Community growth       | P1           |
| **5. Enhanced AI Chat**         | High       | ⭐⭐⭐ High | AI differentiation     | P1           |
| **8. Custom Commands**          | Low        | ⭐⭐ Medium | Knowledge capture      | P1           |
| **13. Themes/Accessibility**    | Medium     | ⭐⭐ Medium | Inclusivity            | P1           |
| **14. Onboarding**              | Low        | ⭐⭐ High   | Activation/retention   | P1           |
| **15. Command Detail View**     | Low        | ⭐⭐ Medium | Power user depth       | P1           |
| **10. Gesture Navigation**      | Medium     | ⭐⭐ High   | Mobile UX parity       | P2           |
| **12. Android Widget**          | High       | ⭐⭐ Medium | Mobile engagement      | P2           |
| **17. Structured AI Explain**   | Medium     | ⭐⭐ Medium | AI quality             | P3           |
| **23. DB Optimizations**        | Low        | ⭐ Medium   | Perf foundation        | P3           |
| **19. Additional Data Sources** | Medium     | ⭐⭐ Medium | Content breadth        | P4           |
| **27-29. Distribution**         | High       | ⭐⭐ Medium | Growth                 | P4           |

---

## 🚀 Phase 0: Foundation (Week 1-2) — _Complete_

- ✅ CLI completions (`wc completions <shell>`)
- ✅ Full build script (`bun run build` = prebuild + desktop + android + cli)
- ✅ Package.json scripts for CLI workflow

---

## 🎯 Phase 1: Core UX & Performance (Week 3-6) — **P0**

### 1.1 Command Palette / Global Search (Cmd/Ctrl+K)

**Effort**: Medium | **Files**: 8-10 | **Risk**: Low

**Implementation**:

- **Rust** (`wc-core`): Add `search_all(query, limit)` → returns unified results (commands, frameworks, recent, favorites)
- **Tauri**: `tauri-plugin-global-shortcut` → register `CmdOrCtrl+K` → emit event to frontend
- **Frontend**: New `CommandPalette.vue` (portal-mounted overlay)
  - Fuse.js for client-side fuzzy ranking (instant)
  - Keyboard nav: ↑/↓, Enter (copy), Shift+Enter (playground), Ctrl+Enter (explain)
  - Sections: Commands | Frameworks | Recent | Favorites | AI Suggestions
- **Settings**: Toggle global shortcut, customize hotkey

**Acceptance**: <100ms open latency, 60fps scroll, works offline

---

### 1.2 Parameterized Command Templates

**Effort**: Medium | **Files**: 6-8 | **Risk**: Low

**Implementation**:

- **DB Migration**: Add `params JSON` column to `commands` table
- **Model** (`wc-core::models::Command`): Add `params: Option<Vec<Param>>`
  ```rust
  struct Param { key: String, label: String, type: ParamType, default: Option<String>, choices: Option<Vec<String>> }
  enum ParamType { String, Path, Choice, Number }
  ```
- **DB Updater** (`scripts/db-updater`): Detect `{{placeholder}}` in tldr examples → auto-generate param schema
- **UI**: `ParameterModal.vue` — dynamic form based on param types (text, file picker, select)
- **Playground**: Show resolved command with substituted values
- **Copy**: "Copy with values" button in CommandCard

**Acceptance**: Template commands prompt for values; resolved command works in playground

---

### 1.3 Smart History & Frequency Tracking

**Effort**: Low | **Files**: 4-5 | **Risk**: Very Low

**Implementation**:

- **DB**: New table `command_usage (id, command_id, action, timestamp, platform)`
- **Tauri Commands**: `record_usage(command_id, action)`, `get_recent(limit)`, `get_top(limit, window)`
- **Frontend**:
  - "Recents" tab in Browse (last 10 used)
  - "Top Commands" in Research (most copied this week)
  - Settings: "Track usage" toggle (default on, local-only)

**Acceptance**: History persists across restarts; privacy-respecting

---

### 1.4 Virtualized Lists & Incremental Search

**Effort**: Medium | **Files**: 4-6 | **Risk**: Low

**Implementation**:

- **Frontend**: `@tanstack/vue-virtual` in BrowseView & ResearchView
- **Rust**: Pagination support — `search_commands(query, limit, offset)` → returns `{ items, total }`
- **Debounce**: 150ms debounce on search input
- **Skeleton**: Loading placeholders during fetch

**Acceptance**: 60fps scroll with 5000+ commands; no jank on low-end Android

---

### 1.5 Error Boundaries & Graceful Degradation

**Effort**: Low | **Files**: 3-5 | **Risk**: Low

**Implementation**:

- **Vue**: Global `onErrorCaptured` → toast + error modal with "Report" button
- **AI Router**: Explicit fallback chain `opencode_zen → kilo_gateway → local_llm → stub`
- **DB**: Startup `PRAGMA integrity_check`; auto-repair from seed on corruption
- **Offline Banner**: Network status listener → dismissible banner

**Acceptance**: No white screens; AI degrades gracefully; DB self-heals

---

## 🎯 Phase 2: Power Features & Polish (Week 7-12) — **P1**

### 2.1 Command Packs (Collections)

**Effort**: Medium | **Files**: 8-10 | **Risk**: Medium

**Implementation**:

- **DB**: Tables `packs`, `pack_commands` (many-to-many)
- **Seed**: Built-in packs (Kubernetes, Docker, Git, AWS, React Native)
- **UI**: ResearchView → "Packs" tab; swipe-to-install on mobile
- **Import/Export**: JSON manifest → share via Gist/URL
- **CLI**: `wc pack install <url>`, `wc pack export favorites`

---

### 2.2 Enhanced AI Chat — Multi-turn Context

**Effort**: High | **Files**: 10-15 | **Risk**: Medium

**Implementation**:

- **Context**: Extend `AiContext` with `history`, `cwd`, `recent_commands`
- **Providers**: Update prompts to include last 3-5 exchanges + relevant DB commands
- **UI**: Threaded conversation view; "Continue" chips for follow-ups
- **Local LLM**: Context window management (summarize older turns)

---

### 2.3 Custom Command Authoring

**Effort**: Low | **Files**: 5-7 | **Risk**: Low

**Implementation**:

- **DB**: `source = 'user'` filter; CRUD in `wc-core`
- **UI**: MoreView → "My Commands" (add/edit/delete)
- **Markdown**: `marked` for rich descriptions
- **Sync**: Export/import JSON/TOML

---

### 2.4 Themes & Accessibility

**Effort**: Medium | **Files**: 6-8 | **Risk**: Low

**Implementation**:

- **Halo Tokens**: Add `[data-theme="light"]` overrides in `packages/halo/index.css`
- **Media Queries**: `prefers-color-scheme`, `prefers-reduced-motion`, `prefers-contrast`
- **Settings**: Theme selector (Dark/Light/System), Accent picker (4 colors)
- **Focus**: Visible focus rings everywhere

---

### 2.5 Onboarding & Interactive Tutorial

**Effort**: Low | **Files**: 4-6 | **Risk**: Low

**Implementation**:

- **Flow**: First-run modal → Provider picker → GGUF import → Playground demo → Shortcuts cheat sheet
- **Component**: `OnboardingStepper.vue` (driver.js or custom)
- **Persistence**: `localStorage` flag; "Show tips" toggle in Settings

---

### 2.6 Command Detail Drill-down

**Effort**: Low | **Files**: 4-5 | **Risk**: Low

**Implementation**:

- **Route**: `/command/:id` (Vue Router)
- **View**: Tabs — Overview | Examples | Related | Notes
- **Actions**: Copy, Explain, Playground, Open tldr page
- **Notes**: User markdown notes per command (localStorage)

---

## 🎯 Phase 3: Mobile Excellence (Week 13-18) — **P2**

### 3.1 Gesture Navigation & Touch Optimizations

**Effort**: Medium | **Files**: 5-8 | **Risk**: Medium

**Implementation**:

- **Tabs**: Swipe between Browse/Playground/Research/AI Chat/More
- **Cards**: Long-press → context menu (Copy, Explain, Playground, Favorite)
- **Pull-to-refresh**: Browse & Research views
- **Bottom Sheets**: Command detail (instead of full-screen on mobile)
- **Haptics**: `tauri-plugin-haptics` for key actions

---

### 3.2 Android Widget (Glance)

**Effort**: High | **Files**: 15-20 | **Risk**: High

**Implementation**:

- **Glance AppWidget**: Top 3 favorites + "Ask AI" / "Search" quick actions
- **SharedPreferences**: Sync favorites from app DB → widget
- **Deep Links**: `whatcommand://command/<id>`, `whatcommand://ai`, `whatcommand://search?q=...`
- **WorkManager**: Refresh widget every 30min
- **Config Activity**: User selects which favorites to show

---

### 3.3 Offline-First with Background Sync

**Effort**: Medium | **Files**: 6-10 | **Risk**: Medium

**Implementation**:

- **DB Updater**: WorkManager job (weekly) → download seed → notify app
- **AI Outbox**: Queue requests when offline → retry with backoff when online
- **Conflict Resolution**: Last-write-wins + merge for favorites/history

---

## 🎯 Phase 4: AI Intelligence (Week 19-26) — **P3**

### 4.1 Structured AI Explanation

**Effort**: Medium | **Files**: 5-8 | **Risk**: Low

**Implementation**:

- **Schema**: Structured output (What it does | Flags | Pitfalls | Alternatives | Examples)
- **Providers**: Update all providers to return structured JSON
- **UI**: Accordion sections; copyable flag table; danger callouts

---

### 4.2 DB Optimizations

**Effort**: Low | **Files**: 2-3 | **Risk**: Very Low

**Implementation**:

- **Pragmas**: `WAL`, `busy_timeout=5000`, `mmap_size`
- **FTS5 Triggers**: Auto-rebuild on insert/update/delete
- **Indexes**: `(category, danger_level)`, `(source, updated_at)`

---

### 4.3 Local RAG (Optional/Experimental)

**Effort**: High | **Files**: 10-15 | **Risk**: High

**Implementation**:

- **Embeddings**: `fastembed-rust` or ONNX (all-MiniLM-L6-v2)
- **Storage**: Sidecar `.vec` file or `embedding BLOB` column
- **Search**: Hybrid BM25 + cosine similarity
- **AI Context**: Top-K relevant commands injected into prompt

---

## 🎯 Phase 5: Ecosystem & Distribution (Week 27+) — **P4**

### 5.1 Additional Data Sources

**Effort**: Medium | **Files**: 8-12 | **Risk**: Low

**Implementation**:

- **Modular Adapters**: `scripts/db-updater/sources/` — each source = TS class
- **Sources**: `cheat.sh`, `eg`, `navi`, man pages, custom YAML
- **CI**: Weekly GitHub Action per source

---

### 5.2 Package Manager Distribution

**Effort**: High | **Files**: 5-8 | **Risk**: Medium

**Implementation**:

- **Homebrew**: Formula via `tauri-action` artifacts
- **Scoop/Chocolatey/AUR/Nix**: Community PRs
- **crates.io**: `cargo publish` for `wc-cli`
- **GitHub Releases**: Auto-generated changelogs

---

### 5.3 F-Droid / IzzyOnDroid

**Effort**: High | **Files**: 3-5 | **Risk**: Medium

**Implementation**:

- **Reproducible Builds**: Pinned deps, no timestamps, `SOURCE_DATE_EPOCH`
- **Metadata**: `fdroidserver` YAML
- **Signing**: Dedicated F-Droid key

---

### 5.4 Auto-update (Desktop)

**Effort**: Medium | **Files**: 4-6 | **Risk**: Low

**Implementation**:

- **Tauri Updater**: `tauri-plugin-updater` + GitHub Releases
- **Channels**: Stable/Beta/Nightly
- **Settings**: Channel selector, "Check now", auto-download toggle

---

## 💡 Quick Wins (Anytime, Parallelizable)

| Task                                  | Effort | Owner    |
| ------------------------------------- | ------ | -------- |
| Copy as `alias` in CommandCard        | 1hr    | Frontend |
| Keyboard Shortcut Cheat Sheet (Cmd+/) | 2hr    | Frontend |
| Recent Commands in Browse header      | 1hr    | Frontend |
| Danger Level Tooltip                  | 1hr    | Frontend |
| Search Result Count                   | 30min  | Frontend |
| Copy Command + Explanation            | 1hr    | Frontend |
| Theme Toggle in Header                | 2hr    | Frontend |
| Reduce Motion Toggle                  | 1hr    | Frontend |

---

## 🛠 Technical Guidelines

### Rust-First Architecture

- All heavy logic (search, parsing, simulation, AI routing) in `wc-core`/`wc-ai`
- CLI, Desktop, Android share identical core — no platform divergence

### Capability-Based Security

- `tauri.conf.json` capabilities: minimal permissions per feature
- New features → new capability file in `src-tauri/capabilities/`

### Offline-by-Default

- Every feature works without network
- AI is enhancement; stub fallback always available

### Design System Compliance

- Extend `packages/halo` tokens — never hardcode colors/spacing
- Use semantic tokens: `var(--color-primary)`, `var(--space-4)`

### Quality Gates

- `bun run check` (format, lint:fix, typecheck) must pass pre-commit
- New Tauri commands → unit tests in `wc-core`
- E2E tests for critical flows (Playwright)

---

## 📅 Timeline Summary

| Phase | Focus           | Weeks | Key Deliverables                                                       |
| ----- | --------------- | ----- | ---------------------------------------------------------------------- |
| 0     | Foundation      | 1-2   | ✅ Done                                                                |
| 1     | Core UX/Perf    | 3-6   | Palette, Templates, History, Virtualization, Error Handling            |
| 2     | Power Features  | 7-12  | Packs, Multi-turn AI, Custom Commands, Themes, Onboarding, Detail View |
| 3     | Mobile          | 13-18 | Gestures, Widget, Offline Sync                                         |
| 4     | AI Intelligence | 19-26 | Structured Explain, DB Opt, Local RAG                                  |
| 5     | Ecosystem       | 27+   | Data Sources, Package Managers, F-Droid, Auto-update                   |

---

## ❓ Open Decisions

1. **Local RAG**: Invest now (P3) or defer until user demand? Requires ~50MB model bundle.
2. **Widget Scope**: Start with favorites only (simpler) or include AI quick action?
3. **Onboarding**: Full stepper vs. lightweight tooltip tour?
4. **Command Packs**: Built-in only v1, or enable community sharing from start?

---

_Plan generated: 2026-08-04 | Based on suggestions.md analysis_
