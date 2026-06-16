\---

version: alpha

name: Halo

description: A modern, minimal, and scalable dark design system that treats the interface as deep-space substrate and information as the source of light.

theme: dark

colors:

&#x20; background: "#0A0B0F"

&#x20; surface: "#14151C"

&#x20; elevated: "#1E2029"

&#x20; border: "#2A2D38"

&#x20; border-strong: "#3A3D4A"

&#x20; on-surface: "#F2F4F8"

&#x20; on-surface-muted: "#9AA0AE"

&#x20; on-surface-faint: "#5C6170"

&#x20; primary: "#5B6BFF"

&#x20; primary-hover: "#7886FF"

&#x20; primary-pressed: "#4A59E6"

&#x20; secondary: "#3DD7E5"

&#x20; tertiary: "#F5D547"

&#x20; success: "#2BE08C"

&#x20; warning: "#F5D547"

&#x20; info: "#3DD7E5"

&#x20; error: "#FF3A5C"

&#x20; focus: "rgba(91, 107, 255, 0.35)"

typography:

&#x20; display:

&#x20;   fontFamily: "Inter, -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif"

&#x20;   fontSize: "4.5rem"

&#x20;   fontWeight: 600

&#x20;   letterSpacing: "-0.03em"

&#x20;   lineHeight: 1.04

&#x20; headline-lg:

&#x20;   fontFamily: "Inter, sans-serif"

&#x20;   fontSize: "2.25rem"

&#x20;   fontWeight: 600

&#x20;   letterSpacing: "-0.02em"

&#x20;   lineHeight: 1.12

&#x20; headline-md:

&#x20;   fontFamily: "Inter, sans-serif"

&#x20;   fontSize: "1.5rem"

&#x20;   fontWeight: 600

&#x20;   letterSpacing: "-0.015em"

&#x20;   lineHeight: 1.2

&#x20; title-md:

&#x20;   fontFamily: "Inter, sans-serif"

&#x20;   fontSize: "1.125rem"

&#x20;   fontWeight: 600

&#x20;   letterSpacing: "-0.01em"

&#x20;   lineHeight: 1.3

&#x20; body-md:

&#x20;   fontFamily: "Inter, sans-serif"

&#x20;   fontSize: "0.9375rem"

&#x20;   fontWeight: 400

&#x20;   letterSpacing: "-0.005em"

&#x20;   lineHeight: 1.55

&#x20; body-sm:

&#x20;   fontFamily: "Inter, sans-serif"

&#x20;   fontSize: "0.8125rem"

&#x20;   fontWeight: 400

&#x20;   letterSpacing: "0"

&#x20;   lineHeight: 1.5

&#x20; label-sm:

&#x20;   fontFamily: "Inter, sans-serif"

&#x20;   fontSize: "0.75rem"

&#x20;   fontWeight: 500

&#x20;   letterSpacing: "0.08em"

&#x20;   lineHeight: 1.2

&#x20;   textTransform: "uppercase"

&#x20; mono-sm:

&#x20;   fontFamily: "'JetBrains Mono', ui-monospace, monospace"

&#x20;   fontSize: "0.8125rem"

&#x20;   fontWeight: 500

&#x20;   letterSpacing: "0"

&#x20;   lineHeight: 1.4

&#x20; metric:

&#x20;   fontFamily: "'JetBrains Mono', ui-monospace, monospace"

&#x20;   fontSize: "2.5rem"

&#x20;   fontWeight: 600

&#x20;   letterSpacing: "-0.02em"

&#x20;   lineHeight: 1

rounded:

&#x20; none: "0px"

&#x20; sm: "6px"

&#x20; md: "10px"

&#x20; lg: "16px"

&#x20; xl: "24px"

&#x20; full: "999px"

spacing:

&#x20; xs: "4px"

&#x20; sm: "8px"

&#x20; md: "16px"

&#x20; lg: "24px"

&#x20; xl: "40px"

&#x20; xxl: "64px"

&#x20; gutter: "24px"

border:

&#x20; width: "1px"

&#x20; width-thick: "1.5px"

elevation:

&#x20; sm: "0 1px 0 rgba(255,255,255,0.02) inset, 0 1px 2px rgba(0,0,0,0.4)"

&#x20; md: "0 8px 24px rgba(0,0,0,0.45), 0 1px 0 rgba(255,255,255,0.03) inset"

&#x20; lg: "0 24px 60px rgba(0,0,0,0.55), 0 1px 0 rgba(255,255,255,0.04) inset"

&#x20; focus: "0 0 0 3px rgba(91, 107, 255, 0.35)"

motion:

&#x20; duration-fast: "120ms"

&#x20; duration-base: "150ms"

&#x20; duration-slow: "240ms"

&#x20; easing-standard: "cubic-bezier(0.2, 0.6, 0.2, 1)"

components:

&#x20; button-primary:

&#x20;   backgroundColor: "{colors.primary}"

&#x20;   textColor: "#FFFFFF"

&#x20;   rounded: "{rounded.md}"

&#x20;   height: "40px"

&#x20;   padding: "0px 18px"

&#x20;   typography: "{typography.body-sm}"

&#x20; button-primary-hover:

&#x20;   backgroundColor: "{colors.primary-hover}"

&#x20;   textColor: "#FFFFFF"

&#x20; button-primary-pressed:

&#x20;   backgroundColor: "{colors.primary-pressed}"

&#x20;   textColor: "#FFFFFF"

&#x20; button-secondary:

&#x20;   backgroundColor: "{colors.surface}"

&#x20;   textColor: "{colors.on-surface}"

&#x20;   rounded: "{rounded.md}"

&#x20;   height: "40px"

&#x20;   padding: "0px 18px"

&#x20;   border: "1px solid {colors.border-strong}"

&#x20; button-tertiary:

&#x20;   backgroundColor: "transparent"

&#x20;   textColor: "{colors.on-surface-muted}"

&#x20;   rounded: "{rounded.md}"

&#x20;   height: "40px"

&#x20;   padding: "0px 18px"

&#x20; button-danger:

&#x20;   backgroundColor: "{colors.error}"

&#x20;   textColor: "#FFFFFF"

&#x20;   rounded: "{rounded.md}"

&#x20;   height: "40px"

&#x20;   padding: "0px 18px"

&#x20; input-field:

&#x20;   backgroundColor: "{colors.surface}"

&#x20;   textColor: "{colors.on-surface}"

&#x20;   rounded: "{rounded.md}"

&#x20;   height: "40px"

&#x20;   padding: "0px 14px"

&#x20;   border: "1px solid {colors.border}"

&#x20; input-field-focus:

&#x20;   backgroundColor: "{colors.surface}"

&#x20;   border: "1px solid {colors.primary}"

&#x20;   shadow: "{elevation.focus}"

&#x20; input-field-error:

&#x20;   border: "1px solid {colors.error}"

&#x20; card:

&#x20;   backgroundColor: "{colors.surface}"

&#x20;   textColor: "{colors.on-surface}"

&#x20;   rounded: "{rounded.lg}"

&#x20;   padding: "24px"

&#x20;   border: "1px solid {colors.border}"

&#x20; card-elevated:

&#x20;   backgroundColor: "{colors.elevated}"

&#x20;   rounded: "{rounded.lg}"

&#x20;   padding: "24px"

&#x20;   border: "1px solid {colors.border-strong}"

&#x20;   shadow: "{elevation.md}"

&#x20; checkbox:

&#x20;   backgroundColor: "{colors.surface}"

&#x20;   rounded: "6px"

&#x20;   size: "18px"

&#x20;   border: "1px solid {colors.border-strong}"

&#x20; checkbox-checked:

&#x20;   backgroundColor: "{colors.primary}"

&#x20;   textColor: "#FFFFFF"

&#x20;   border: "1px solid {colors.primary}"

&#x20; radio:

&#x20;   backgroundColor: "{colors.surface}"

&#x20;   rounded: "{rounded.full}"

&#x20;   size: "18px"

&#x20;   border: "1px solid {colors.border-strong}"

&#x20; radio-checked:

&#x20;   backgroundColor: "{colors.primary}"

&#x20;   textColor: "#FFFFFF"

&#x20;   border: "1px solid {colors.primary}"

&#x20; switch-track:

&#x20;   backgroundColor: "{colors.elevated}"

&#x20;   rounded: "{rounded.full}"

&#x20;   width: "36px"

&#x20;   height: "20px"

&#x20;   border: "1px solid {colors.border-strong}"

&#x20; switch-track-on:

&#x20;   backgroundColor: "{colors.primary}"

&#x20;   border: "1px solid {colors.primary}"

&#x20; tabs-container:

&#x20;   backgroundColor: "{colors.surface}"

&#x20;   rounded: "{rounded.full}"

&#x20;   padding: "4px"

&#x20;   border: "1px solid {colors.border}"

&#x20; tabs-active:

&#x20;   backgroundColor: "{colors.elevated}"

&#x20;   textColor: "{colors.on-surface}"

&#x20;   rounded: "{rounded.full}"

&#x20;   height: "32px"

&#x20;   padding: "0px 14px"

&#x20;   border: "1px solid {colors.primary}"

&#x20; tabs-inactive:

&#x20;   backgroundColor: "transparent"

&#x20;   textColor: "{colors.on-surface-muted}"

&#x20;   rounded: "{rounded.full}"

&#x20;   height: "32px"

&#x20;   padding: "0px 14px"

&#x20; chip:

&#x20;   backgroundColor: "rgba(91, 107, 255, 0.12)"

&#x20;   textColor: "{colors.primary-hover}"

&#x20;   rounded: "{rounded.full}"

&#x20;   height: "24px"

&#x20;   padding: "0px 10px"

&#x20;   typography: "{typography.mono-sm}"

&#x20; stat-tile:

&#x20;   backgroundColor: "{colors.surface}"

&#x20;   textColor: "{colors.on-surface}"

&#x20;   rounded: "{rounded.lg}"

&#x20;   padding: "20px"

&#x20;   border: "1px solid {colors.border}"

&#x20; stat-tile-accent:

&#x20;   backgroundColor: "{colors.primary}"

&#x20;   height: "2px"

\---



\## Overview



Halo is a dark, architectural design system built around the idea that the interface should fade and the data should glow. Surfaces stack in three quiet charcoal tiers, hairline 1px borders draw the geometry, and a single electric indigo carries every action. Saturated signal colors — lime, amber, cyan, magenta — are reserved for status, trends, and points of focus, so the screen reads as a calm dashboard with bright, intentional flares of information.



The system is framework-agnostic. It is delivered as plain CSS custom properties and reusable class selectors, paired with semantic HTML elements (`<button>`, `<input>`, `<label>`, `<section>`, `<article>`). It is designed for product surfaces — dashboards, console screens, settings panels, marketing pages — where density and clarity matter more than ornament.



\## Colors



Halo's palette is intentionally tight. The neutral spine is a single near-black canvas split into three surface tiers, separated by hairline borders rather than shadows. Color is then introduced as light: one brand indigo for actions and focus, and four signal hues that each map to a single, predictable meaning.



| Token | Hex | Role |

| --- | --- | --- |

| `background` | `#0A0B0F` | Page canvas, viewport, and outer layout |

| `surface` | `#14151C` | Cards, panels, inputs, and component fills |

| `elevated` | `#1E2029` | Modals, popovers, active tabs, hovered controls |

| `border` | `#2A2D38` | Hairline dividers and default component outlines |

| `border-strong` | `#3A3D4A` | Inputs, secondary buttons, emphasised dividers |

| `on-surface` | `#F2F4F8` | Headlines, body, and primary foreground |

| `on-surface-muted` | `#9AA0AE` | Secondary text, labels, inactive controls |

| `on-surface-faint` | `#5C6170` | Helper text, placeholders, captions |

| `primary` | `#5B6BFF` | Primary action, focus ring base, brand accent |

| `success` | `#2BE08C` | Positive metrics, confirmation, "up" trends |

| `warning` | `#F5D547` | Caution, attention, editorial highlight |

| `info` | `#3DD7E5` | Information, data accent, decorative |

| `error` | `#FF3A5C` | Destructive actions, "down" trends, critical alerts |



Accessibility: primary text on `background` meets WCAG AA at all body sizes. Use `on-surface-muted` for supporting text only; never for primary copy below 14px. Signal colors are always paired with an icon or label — never used as the only signal.



\## Typography



Halo uses a modern grotesque typographic system with a tabular monospace companion for numbers, hex tokens, and dense data. Inter is the primary face for UI, headlines, and body; JetBrains Mono carries every metric, code sample, and token value so numerics align cleanly in tables and stat tiles.



| Level | Family | Size / Weight / Tracking |

| --- | --- | --- |

| `display` | Inter | clamp(2.75rem, 6vw, 4.5rem) / 600 / -0.03em |

| `headline-lg` | Inter | 2.25rem / 600 / -0.02em |

| `headline-md` | Inter | 1.5rem / 600 / -0.015em |

| `title-md` | Inter | 1.125rem / 600 / -0.01em |

| `body-md` | Inter | 0.9375rem / 400 / -0.005em |

| `body-sm` | Inter | 0.8125rem / 400 / 0 |

| `label-sm` | Inter | 0.75rem / 500 / 0.08em UPPERCASE |

| `mono-sm` | JetBrains Mono | 0.8125rem / 500 / 0 |

| `metric` | JetBrains Mono | 2.5rem / 600 / -0.02em |



Display sizes carry tight negative tracking and are used sparingly, typically once per page for hero headlines. Eyebrow labels use `label-sm` with wide tracking to act as quiet section dividers. Numerics always use `mono-sm` or `metric` to preserve column alignment.



\## Layout



The layout follows a 4px base spacing scale. Containers cap at 1200px with a responsive horizontal padding via `clamp(20px, 4vw, 48px)`. Internal density is medium: components allow generous breathing room, with 24px section gutters and 16px inline gutters as the default.



\- Spacing: `xs 4px`, `sm 8px`, `md 16px`, `lg 24px`, `xl 40px`, `xxl 64px`.

\- Container: `max-width: 1200px` with responsive side padding.

\- Grids: 2, 3, and 4 column responsive grids collapse to single column below 720px.

\- Section rhythm: every major section is separated by 64–80px of vertical space; cards inside a section by 24px.

\- Top navigation is a 64px bar with a hairline bottom border; the brand mark sits on the left and a control cluster sits on the right.



\## Elevation \& Depth



Depth in Halo is created by stacking material, not by blurring shadows. Three surface tiers (background → surface → elevated) communicate hierarchy, and 1px borders carry the geometry that shadow would carry in a light system. A single ambient shadow is reserved for floating elevated surfaces such as modals and popovers; focus and hover use saturated color, not blur.



| Token | Value | Use |

| --- | --- | --- |

| `elevation.sm` | inset + 1px shadow | Pressed buttons, subtle inner depth |

| `elevation.md` | 0 8px 24px black 45% | Elevated cards, popovers |

| `elevation.lg` | 0 24px 60px black 55% | Modals, command palettes, drawers |

| `elevation.focus` | 0 0 0 3px indigo 35% | All focus rings |



Focus is the only state allowed to glow. Hover lifts a control by one tier (e.g. `surface → elevated`) rather than adding a shadow.



\## Shapes



The shape language is soft, architectural rectangle. Corners are never sharp; ornament is achieved with hairline borders and color, never chrome or gradients.



\- `none` 0px — used only for full-bleed dividers.

\- `sm` 6px — checkboxes and small inline pills.

\- `md` 10px — buttons, inputs, selects, segmented controls.

\- `lg` 16px — cards, stat tiles, panels.

\- `xl` 24px — hero sections, large feature surfaces.

\- `full` 999px — tabs container, switch tracks, dot badges, signal chips.



Border weight is uniformly `1px`. A `1.5px` thick variant is reserved for emphasized strokes on hero cards or callout banners.



\## Components



\### Button

Three variants and three sizes share one anatomy: 10px radius, single-line label, optional leading or trailing icon. Primary is a solid indigo fill with a faint top inset highlight. Secondary is a surface tile with a 1px strong border. Tertiary is text-only with a surface hover tint. Danger swaps primary indigo for the magenta error token. Sizes are 32px (sm), 40px (default), and 48px (lg).



\### Input and form field

Inputs sit on the `surface` tier with a 1px `border`. A `label-sm` uppercase label sits above with widened tracking. Placeholder uses `on-surface-faint`. Focus state switches the border to `primary` and adds the standard 3px focus ring. Optional leading icon and helper text are supported. Error state replaces the border with `error` and the focus ring with a soft `error` glow. Selects and textareas inherit the same anatomy.



\### Card

Cards use 16px radius, 1px border, and 20–24px internal padding. Variants include base card, elevated card (uses `elevated` surface and `md` shadow), media card (top media block with hairline bottom border), and accent card (a colored 2px top hairline drawn from any signal token via `data-accent`).



\### Checkbox and radio

18px square with 6px radius (checkboxes) or full radius (radio). Default state is `surface` fill with a strong border; checked fills with `primary` and shows a white check glyph. Radio adds an inner 8px white dot. Both share the standard 3px focus ring.



\### Tabs (segmented control)

A pill-style container on the `surface` tier with 4px internal padding and full radius. Active tab is an `elevated` tile with a 1px `primary` border. Inactive tabs are transparent with `on-surface-muted` text; hover lifts to elevated.



\### Stat Tile (signature)

The system's hero pattern. A compact metric tile pairs a `label-sm` eyebrow, a large monospaced numeric value, a trend chip (lime for positive, magenta for negative) with a directional arrow glyph, and a 32px-tall inline sparkline rendered as a polyline. The tile lives on `surface`, uses 16px radius and a hairline border, and carries a 2px colored top hairline that picks up the relevant signal tone. Three sizes (`sm`, `md`, `lg`) cover dashboard, list, and hero contexts.



\### Icons

The system uses Lucide (https://lucide.dev, ISC license) as its single icon library. Icons are rendered at a 1.5–1.75px stroke weight to match the hairline border language and inherit `currentColor` so they tint with their containing component. Do not mix icon libraries or invent custom paths.



\## Do's and Don'ts



\*\*Do\*\*

\- Use the three surface tiers in order: `background` for page, `surface` for components, `elevated` for floating or active states.

\- Reserve `primary` indigo for action and focus; never use it for purely decorative fills.

\- Use `mono-sm` or `metric` for every number that needs to align in a column.

\- Pair signal colors with an icon or text label, never color alone.

\- Keep one icon library — Lucide — and one stroke weight across the entire surface.



\*\*Don't\*\*

\- Don't introduce drop shadows on flat components; depth comes from tier and border.

\- Don't combine more than one signal color per stat tile or chip.

\- Don't lower body text below 13px or set primary text in `on-surface-muted`.

\- Don't sharpen corners to 0; the system has no square components.

\- Don't gradient-fill buttons or surfaces; brand color is delivered as a single, solid tone.



