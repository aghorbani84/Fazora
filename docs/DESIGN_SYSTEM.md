# FAZORA Design System

Source of truth for the FAZORA visual identity. Implementation lives in
`src/design/tokens.css` (CSS custom properties) and `src/design/tokens.rs`
(Rust-side constants for canvas drawing and layout math).

---

## 1. Brand identity

**Tagline:** Sound. Motion. Precision.

### Logo

- **Monogram:** geometric "F" built on a 64×64 grid, square stroke caps,
  miter joins. Used standalone for application icons, compact navigation and
  brand marks. See `src/components/branding/fazora_logo.rs` and
  `brand_mark.rs`.
- **Wordmark:** FAZORA in Manrope SemiBold, tracking -0.02em. Used in
  onboarding, application headers and branding locations.

Never invent a different logo in individual screens.

---

## 2. Color tokens

| Token | CSS variable | Value | Usage |
|---|---|---|---|
| Obsidian | `--fazora-obsidian` | `#0B0D10` | Application foundation (background) |
| Carbon | `--fazora-carbon` | `#15191F` | Panels, navigation |
| Gold | `--fazora-gold` | `#D8B56A` | Focus, selected states, active controls |
| Ivory | `--fazora-ivory` | `#F4F1E9` | Primary typography |
| Steel | `--fazora-steel` | `#89919D` | Secondary information |
| Teal | `--fazora-teal` | `#78CFC1` | Playback progress, positive status |

### Semantic tokens

```css
--background: var(--fazora-obsidian);
--surface-primary: var(--fazora-carbon);
--surface-secondary: #1C2129;
--surface-tertiary: #242A33;

--text-primary: var(--fazora-ivory);
--text-secondary: #AEB4BC;
--text-muted: var(--fazora-steel);

--accent: var(--fazora-gold);
--accent-hover: #E7C986;
--accent-active: #C9A15A;
--accent-muted: rgba(216, 181, 106, 0.14);
--accent-text: var(--fazora-obsidian);

--success: var(--fazora-teal);
--warning: #E4B978;
--danger: #E47C7C;

--border: rgba(244, 241, 233, 0.09);
--divider: rgba(244, 241, 233, 0.07);
--focus-ring: var(--fazora-gold);
```

**Color philosophy:** gold is a distinctive accent — never paint every button
gold. Avoid gradients, neon colors, glowing borders and unnecessary effects.

---

## 3. Typography

| Purpose | Typeface | CSS variable |
|---|---|---|
| Brand and headings | Manrope | `--font-brand` |
| Main interface | Manrope | `--font-ui` |
| Persian / Arabic interface | Vazirmatn | `--font-persian` |
| Technical information | IBM Plex Mono | `--font-mono` |

Scale: `--text-xs` (0.75rem) through `--text-5xl` (3rem). Headings use
`--font-brand`, weight 700, letter-spacing -0.02em, line-height 1.35. Body
uses `--font-ui`, line-height 1.6. Numbers use `tabular-nums` where aligned
(durations, timestamps).

Multilingual support: `body[dir="ltr"]` → UI font; `body[dir="rtl"]` →
Vazirmatn. RTL mirrors navigation and spacing, but media controls, timeline
direction, artwork composition and technical elements keep appropriate
interaction conventions.

---

## 4. Spacing

| Token | Value |
|---|---|
| `--space-1` | 4px |
| `--space-2` | 8px |
| `--space-3` | 12px |
| `--space-4` | 16px |
| `--space-5` | 20px |
| `--space-6` | 24px |
| `--space-8` | 32px |
| `--space-10` | 40px |
| `--space-12` | 48px |

---

## 5. Radii

| Token | Value |
|---|---|
| `--radius-small` | 6px |
| `--radius-medium` | 10px |
| `--radius-large` | 16px |
| `--radius-xl` | 22px |

---

## 6. Motion

| Token | Value | Use |
|---|---|---|
| `--transition-fast` | 120ms | Small interactions |
| `--transition-base` | 180ms | Panels, menus |
| `--transition-slow` | 240ms | Page transitions |

Easing: `cubic-bezier(0.16, 1, 0.3, 1)` (ease-out). Respect
`prefers-reduced-motion: reduce` — all transitions collapse to 0.01ms.

Animation is for state communication, never decoration: opacity, subtle
scale, smooth panel movement, waveform progression, artwork crossfades.

---

## 7. Layout metrics

| Token | Value |
|---|---|
| `--sidebar-width` | 260px |
| `--sidebar-compact-width` | 72px |
| `--topbar-height` | 56px |
| `--player-dock-height` | 88px |
| `--secondary-panel-width` | 360px |

Z-index scale: dropdown 100, modal 200, tooltip 300, toast 400.

---

## 8. Component specifications

### Button (`fz-button`)
Height 36px, padding 0–16px, radius 10px, weight 600, Manrope. Variants:
primary (gold fill, obsidian text), surface (carbon-ish fill, border),
ghost (transparent), danger (transparent, danger border/text). Icon buttons
are 36×36 squares.

### Card (`fz-card`)
Surface-secondary fill, 1px border, radius 16px, subtle hover border lift.
No drop shadows by default.

### Input (`fz-input`)
Height 40px, surface-tertiary fill, border, radius 10px, gold focus ring.

### Toggle (`fz-toggle`)
44×24px track, 20px thumb, radius 9999px, 180ms thumb transition.

### Slider (`fz-slider`)
4px track, accent fill, 14px thumb with obsidian border. Used for timeline,
volume, equalizer bands.

### Badge (`fz-badge`)
20px height, radius 9999px, weight 700, text-xs. Default or accent (gold)
variant.

---

## 9. Icon system

Inline SVG symbols in `assets/icons.svg`, referenced by `<use href="#icon-…">`
from icon components. Icons are currentColor, 24×24 logical grid, 1.5px
stroke, round caps/joins. Monochrome — never colored except the active state
which uses the gold accent.

---

## 10. States

- `disabled`: opacity 0.45, cursor not-allowed.
- `active/selected`: gold accent fill or border.
- `hover`: surface-tertiary → rgba(ivory, 0.1), border brightens to 0.16 alpha.
- `focus-visible`: 2px gold outline, 2px offset.
- `loading`: inline spinner (`fz-spinner`), never block the whole page unless
  a full-screen load is genuinely required.
