\# What Command



\# Updated Entscheidungen \& Ergänzungen



\## Android App — wie einbinden?



Da Tauri jetzt Android unterstützt (Tauri v2), bleibt der Stack konsistent:



```

┌─────────────────────────────────────────────────────────┐

│                  Shared Core                             │

├──────────┬──────────┬──────────────┬────────────────────┤

│ Desktop  │  CLI     │   Android    │  (iOS optional)    │

│ Tauri+   │  Binary  │   Tauri v2   │  Tauri v2          │

│ React    │          │   + React    │                    │

└──────────┴──────────┴──────────────┴────────────────────┘

```



\*\*Tauri v2 auf Android:\*\*

\- Gleiche React-UI, nur angepasstes Layout (Mobile-First für Android)

\- Native Android-Intents für "in Terminal öffnen" (Termux-Integration!)

\- Verteilung über F-Droid (Open Source!) + GitHub Releases APK



\*\*Termux-Integration\*\* wäre ein Killer-Feature:

```bash

\# In der Android-App: "Im Terminal ausführen"

→ öffnet Termux und führt Command direkt aus

```



\---



\## Cloud AI Provider: opencode ✅



\*\*opencode\*\* (das neue Anthropic/Open-Source CLI-AI-Tool) als primärer Provider:



```toml

\# config.toml

\[ai]

provider = "opencode"   # default

model = "claude-sonnet" # via opencode

fallback = "ollama"     # lokal wenn offline



\# Weitere supported providers:

\# "openai", "anthropic", "gemini", "ollama", "opencode"

```



\*\*Warum opencode als Default?\*\*

\- Open Source → passt zur App-Philosophie

\- Unterstützt viele Modelle hinter einem Interface

\- Kein Vendor Lock-in

\- Community wächst schnell



Provider-Abstraktion im Core:



```rust

trait AiProvider {

&#x20;   async fn generate\_command(\&self, prompt: \&str) -> Result<CommandSuggestion>;

&#x20;   async fn explain\_command(\&self, cmd: \&str) -> Result<String>;

}



// Implementierungen: OpenCodeProvider, OllamaProvider, OpenAIProvider

```



\---



\## Command DB: GitHub Docs als Source ✅



Geniale Idee — statt alles selbst zu kuratieren, \*\*offizielle Docs als Single Source of Truth:\*\*



```

Command DB Pipeline:

┌─────────────────┐    ┌──────────────┐    ┌─────────────┐

│  GitHub Docs    │───▶│  Parser /    │───▶│  SQLite DB  │

│  (tldr-pages,   │    │  Transformer │    │  (lokal,    │

│  cheat.sh,      │    │  (Bun/Rust)  │    │   offline)  │

│  man pages)     │    └──────────────┘    └─────────────┘

└─────────────────┘

```



\*\*Konkrete Sources (alle Open Source auf GitHub):\*\*



| Source | Inhalt | Lizenz |

|---|---|---|

| `tldr-pages/tldr` | 2000+ vereinfachte man pages | CC BY 4.0 |

| `cheat/cheatsheets` | Community cheatsheets | MIT |

| `nickel/cheat.sh` | Aggregator | MIT |

| `donnemartin/eg` | Praktische Beispiele | Apache 2.0 |



\*\*Update-Mechanismus:\*\*

```bash

clh update          # Pullt neueste Commands von GitHub

clh update --force  # Kompletter Rebuild der DB

```



DB-Schema:

```sql

CREATE TABLE commands (

&#x20; id TEXT PRIMARY KEY,

&#x20; command TEXT NOT NULL,

&#x20; description TEXT,

&#x20; category TEXT,

&#x20; platform TEXT,         -- 'linux,macos,windows'

&#x20; danger\_level INTEGER,  -- 0-3

&#x20; source TEXT,           -- 'tldr', 'cheat', 'user'

&#x20; examples JSON,

&#x20; tags TEXT,

&#x20; updated\_at TIMESTAMP

);

```



\---



\## Runtime: Bun vs Rust vs .NET — Empfehlung



| | \*\*Bun\*\* | \*\*Rust\*\* | \*\*.NET / C#\*\* |

|---|---|---|---|

| \*\*CLI Performance\*\* | ⚡ Sehr schnell | ⚡⚡ Schnellste | 🟡 Gut |

| \*\*Native Binaries\*\* | ✅ `bun build --compile` | ✅ | ✅ AOT |

| \*\*Android\*\* | ❌ Kein Tauri-Support | ✅ Nativ | 🟡 MAUI |

| \*\*Cross-Platform\*\* | ✅ | ✅ | ✅ |

| \*\*Dev Speed\*\* | ⚡⚡ Sehr schnell | 🟡 Steile Kurve | ⚡ Schnell |

| \*\*Open Source Eco\*\* | ✅ | ✅ | 🟡 (MS) |

| \*\*Binary Size\*\* | \~10MB | \~2-5MB | \~20MB+ |

| \*\*Tauri Integration\*\* | ✅ (Frontend) | ✅ (Backend+Frontend) | ❌ |



\### \*\*Empfehlung: Hybrid-Ansatz\*\*



```

┌─────────────────────────────────────────────┐

│  Rust                                        │

│  ├── clh-core  (Command DB, Parser, Logic)   │

│  ├── clh-cli   (Binary, Completions)         │

│  └── Tauri Backend (Desktop + Android)       │

│                                              │

│  Bun / TypeScript                            │

│  ├── React Frontend (Desktop + Android UI)   │

│  ├── DB Update Scripts (GitHub Scraper)      │

│  └── CI/CD Tooling                           │

└─────────────────────────────────────────────┘

```



\*\*Warum nicht .NET?\*\*

\- Tauri-Integration ist Rust-first

\- Größere Binaries

\- Weniger in der Open-Source CLI-Welt verbreitet



\*\*Warum nicht Bun allein?\*\*

\- Kein nativer Tauri-Android-Support als Backend

\- Rust gibt echte plattformübergreifende CLI-Binaries ohne Runtime



\---



\## Revidierte Architektur (komplett)



```

clh/  (Open Source, MIT License)

├── crates/

│   ├── clh-core/        # Rust: DB, AI-Abstraction, Parser

│   ├── clh-cli/         # Rust: CLI Binary + Completions

│   └── clh-ai/          # Rust: opencode/ollama/openai clients

├── apps/

│   ├── desktop/         # Tauri v2: macOS/Linux/Windows

│   └── mobile/          # Tauri v2: Android (+ iOS später)

│       └── src/

│           ├── views/

│           │   ├── CommandBrowser.tsx

│           │   ├── AiChat.tsx

│           │   └── Favorites.tsx

│           └── components/

├── scripts/

│   ├── db-updater/      # Bun: GitHub Docs → SQLite

│   └── completions/     # Shell completion scripts

├── data/

│   └── commands.db      # Initiale SQLite DB (gebundelt)

└── .github/

&#x20;   └── workflows/

&#x20;       ├── build.yml    # Tauri Build für alle Plattformen

&#x20;       └── db-update.yml # Wöchentlicher DB-Update via GitHub Action

```



\---



\## GitHub Actions: Automatischer DB-Update



```yaml

\# .github/workflows/db-update.yml

name: Update Command DB

on:

&#x20; schedule:

&#x20;   - cron: '0 2 \* \* 1'  # Jeden Montag

&#x20; workflow\_dispatch:



jobs:

&#x20; update:

&#x20;   steps:

&#x20;     - uses: actions/checkout@v4

&#x20;     - uses: oven-sh/setup-bun@v2

&#x20;     - run: bun run scripts/db-updater/index.ts

&#x20;     - run: |

&#x20;         git commit -am "chore: update command db"

&#x20;         git push

```



\---



\## Open Source Setup



\- \*\*Lizenz:\*\* MIT

\- \*\*GitHub:\*\* `github.com/yourname/clh`

\- \*\*Releases:\*\* GitHub Releases mit signierten Binaries

\- \*\*F-Droid:\*\* Android APK über F-Droid Repo

\- \*\*Homebrew Tap:\*\* `brew install yourname/tap/clh`

\- \*\*Crates.io:\*\* `cargo install clh`

\- \*\*Contributing:\*\* YAML-Dateien für neue Commands → niedrige Hürde für PRs



\---



\## Nächste konkrete Schritte



1\. \*\*Repo aufsetzen\*\* — Cargo Workspace + Bun Monorepo

2\. \*\*clh-core\*\* — SQLite-Integration mit `rusqlite`, tldr-pages importieren

3\. \*\*clh-cli MVP\*\* — `clh search`, `clh ask`, `clh explain`

4\. \*\*opencode Client\*\* — HTTP-Client in Rust gegen opencode API

5\. \*\*Tauri Desktop\*\* — React-UI mit Command-Browser

6\. \*\*Tauri Android\*\* — Mobile-Layout + Termux-Intent



Gute Wahl! Vue passt sehr gut zu Tauri, und kein Tailwind — dann gehen wir Richtung echtes Component-basiertes Styling. Lass mich den Stack konkretisieren.



\---



\## Finaler Stack



```

clh/

├── Rust          → Core, CLI, Tauri Backend

├── Bun           → Package Manager, Build, Scripts

├── TypeScript    → Überall im Frontend

├── Vue 3         → UI Framework (Composition API)

└── CSS / Design  → Kein Tailwind — eigenes Design System

```



\---



\## Frontend: Vue 3 + Styling



\*\*Kein Tailwind\*\* — stattdessen eine dieser Optionen:



| Option | Vibe | Aufwand |

|---|---|---|

| \*\*UnoCSS\*\* (atomic, aber konfigurierbar) | Flexibel, kein Tailwind-Zwang | 🟡 |

| \*\*PrimeVue\*\* | Fertige UI-Components, professionell | 🟢 leicht |

| \*\*Vuetify 3\*\* | Material Design, sehr komplett | 🟢 leicht |

| \*\*Radix Vue + eigenes CSS\*\* | Maximum Control, clean | 🔴 mehr Arbeit |

| \*\*CSS Modules + Design Tokens\*\* | Puristisch, wartbar | 🟡 |



\*\*Empfehlung für CLH:\*\* `Radix Vue` (accessible primitives) + \*\*CSS Variables / CSS Modules\*\* — gibt dir ein sauberes, eigenständiges Look \& Feel, das sich nicht nach "noch einer Tailwind-App" anfühlt.



\---



\## Theme-Konzept



Ein dunkles Terminal-nahes Design mit modernem Twist:



```css

/\* design-tokens.css \*/

:root {

&#x20; /\* Core Palette \*/

&#x20; --clh-bg-base:        #0f1117;

&#x20; --clh-bg-surface:     #1a1d27;

&#x20; --clh-bg-elevated:    #22263a;

&#x20; --clh-bg-hover:       #2a2f45;



&#x20; /\* Accent — ein klares Cyan/Mint \*/

&#x20; --clh-accent:         #00d4aa;

&#x20; --clh-accent-dim:     #00d4aa33;

&#x20; --clh-accent-hover:   #00efc0;



&#x20; /\* Text \*/

&#x20; --clh-text-primary:   #e8eaf0;

&#x20; --clh-text-secondary: #8b90a7;

&#x20; --clh-text-muted:     #4a5068;

&#x20; --clh-text-code:      #a8daff;



&#x20; /\* Danger / Warning \*/

&#x20; --clh-danger:         #ff5f6d;

&#x20; --clh-warning:        #ffb347;

&#x20; --clh-success:        #69db7c;



&#x20; /\* Typography \*/

&#x20; --clh-font-ui:        'Inter', system-ui, sans-serif;

&#x20; --clh-font-mono:      'JetBrains Mono', 'Fira Code', monospace;



&#x20; /\* Spacing \& Radius \*/

&#x20; --clh-radius-sm:      6px;

&#x20; --clh-radius-md:      10px;

&#x20; --clh-radius-lg:      16px;



&#x20; /\* Transitions \*/

&#x20; --clh-transition:     150ms ease;

}

```



\---



\## Projekt-Struktur



```

clh/

├── apps/

│   └── desktop/                  # Tauri v2 App

│       ├── src/                  # Vue Frontend

│       │   ├── main.ts

│       │   ├── App.vue

│       │   ├── assets/

│       │   │   └── styles/

│       │   │       ├── tokens.css      # Design Tokens

│       │   │       ├── base.css        # Reset + Globals

│       │   │       └── typography.css

│       │   ├── components/

│       │   │   ├── CommandCard.vue

│       │   │   ├── SearchBar.vue

│       │   │   ├── CategorySidebar.vue

│       │   │   ├── AiPanel.vue

│       │   │   └── ui/             # Atomic UI Components

│       │   │       ├── Button.vue

│       │   │       ├── Badge.vue

│       │   │       ├── CodeBlock.vue

│       │   │       └── Kbd.vue     # Keyboard shortcut display

│       │   ├── views/

│       │   │   ├── HomeView.vue

│       │   │   ├── BrowserView.vue

│       │   │   ├── AiView.vue

│       │   │   └── FavoritesView.vue

│       │   ├── stores/             # Pinia

│       │   │   ├── commands.ts

│       │   │   ├── ai.ts

│       │   │   └── settings.ts

│       │   └── composables/

│       │       ├── useSearch.ts

│       │       ├── useTauri.ts     # invoke() wrapper

│       │       └── useAi.ts

│       ├── src-tauri/

│       │   ├── src/

│       │   │   ├── main.rs

│       │   │   ├── commands.rs     # Tauri Commands

│       │   │   └── db.rs

│       │   └── Cargo.toml

│       ├── index.html

│       ├── vite.config.ts

│       └── package.json

│

├── crates/

│   ├── clh-core/                 # Rust: DB + Logic

│   ├── clh-cli/                  # Rust: CLI Binary

│   └── clh-ai/                   # Rust: AI Clients

│

└── scripts/

&#x20;   └── db-updater/               # Bun: tldr → SQLite

&#x20;       ├── index.ts

&#x20;       └── package.json

```



\---



\## Package.json / Bun Setup



```json

{

&#x20; "name": "clh-desktop",

&#x20; "scripts": {

&#x20;   "dev":        "bunx tauri dev",

&#x20;   "build":      "bunx tauri build",

&#x20;   "vue:dev":    "vite",

&#x20;   "typecheck":  "vue-tsc --noEmit",

&#x20;   "lint":       "eslint src --ext .ts,.vue"

&#x20; },

&#x20; "dependencies": {

&#x20;   "vue":               "^3.5",

&#x20;   "@tauri-apps/api":   "^2.0",

&#x20;   "pinia":             "^2.2",

&#x20;   "vue-router":        "^4.4",

&#x20;   "@radix-vue/core":   "latest"

&#x20; },

&#x20; "devDependencies": {

&#x20;   "vite":              "^6",

&#x20;   "@vitejs/plugin-vue":"^5",

&#x20;   "vue-tsc":           "latest",

&#x20;   "typescript":        "^5.5"

&#x20; }

}

```



\---



\## Beispiel: CommandCard.vue



```vue

<script setup lang="ts">

import { computed } from 'vue'

import Badge from './ui/Badge.vue'

import Kbd from './ui/Kbd.vue'



interface Props {

&#x20; command: string

&#x20; description: string

&#x20; category: string

&#x20; dangerLevel?: 0 | 1 | 2 | 3

&#x20; platform: string\[]

}

const props = defineProps<Props>()

const isDangerous = computed(() => props.dangerLevel >= 2)

</script>



<template>

&#x20; <article class="command-card" :class="{ 'command-card--danger': isDangerous }">

&#x20;   <div class="command-card\_\_header">

&#x20;     <Badge :label="category" variant="category" />

&#x20;     <span v-if="isDangerous" class="command-card\_\_danger-tag">⚠ destructive</span>

&#x20;   </div>



&#x20;   <pre class="command-card\_\_code"><code>{{ command }}</code></pre>

&#x20;   <p class="command-card\_\_desc">{{ description }}</p>



&#x20;   <footer class="command-card\_\_actions">

&#x20;     <button class="btn btn--ghost" @click="$emit('copy')">Copy</button>

&#x20;     <button class="btn btn--ghost" @click="$emit('explain')">Explain</button>

&#x20;     <button class="btn btn--icon" @click="$emit('favorite')">⭐</button>

&#x20;   </footer>

&#x20; </article>

</template>



<style scoped>

.command-card {

&#x20; background:    var(--clh-bg-surface);

&#x20; border:        1px solid var(--clh-bg-elevated);

&#x20; border-radius: var(--clh-radius-md);

&#x20; padding:       1rem 1.25rem;

&#x20; transition:    border-color var(--clh-transition);



&#x20; \&:hover {

&#x20;   border-color: var(--clh-accent-dim);

&#x20; }

}

.command-card--danger {

&#x20; border-color: var(--clh-warning);

}

.command-card\_\_code {

&#x20; font-family:   var(--clh-font-mono);

&#x20; color:         var(--clh-text-code);

&#x20; font-size:     0.9rem;

&#x20; background:    var(--clh-bg-base);

&#x20; padding:       0.6rem 0.9rem;

&#x20; border-radius: var(--clh-radius-sm);

&#x20; overflow-x:    auto;

}

.command-card\_\_desc {

&#x20; color:         var(--clh-text-secondary);

&#x20; font-size:     0.875rem;

&#x20; margin-top:    0.5rem;

}

.command-card\_\_actions {

&#x20; display:       flex;

&#x20; gap:           0.5rem;

&#x20; margin-top:    0.75rem;

}

</style>

```



\---



\## Zusammenfassung



```

✅ Tauri v2        → Native Desktop + Android

✅ Bun             → Package Manager + Build + Scripts  

✅ TypeScript       → Frontend + Scripts

✅ Vue 3            → Composition API + <script setup>

✅ Pinia            → State Management

✅ Radix Vue        → Accessible Primitives

✅ CSS Variables    → Eigenes Design System, kein Tailwind

✅ JetBrains Mono   → Code-Font passend zum CLI-Vibe

```



\---



\- Rust + Bun Hybrid-Stack

\- Tauri v2 für Desktop \& Android

\- opencode als AI-Provider

\- tldr-pages/GitHub Docs als Command-DB

\- Komplett Open Source (MIT)



etup Commands — exakt so ausführen

\# 1. Projekt erstellen

bun create tauri-app



\# Im Wizard auswählen:

\# → Project name: (dein Name, siehe unten)

\# → Identifier: com.yourname.clh

\# → Language: TypeScript / JavaScript

\# → Package Manager: bun

\# → UI Template: Vue

\# → UI Flavor: TypeScript

\# 2. Dependencies installieren \& testen

cd <projektname>

bun install

\# 3. Android Prerequisites (einmalig)

\# Android Studio + SDK muss installiert sein

\# Dann:

bun tauri android init



