//! CSS design tokens and component styles.
//! Injected once by the root `App` component via `<style>` elements.
//! The token block is the single source of truth for FAZORA's visual identity.

pub const TOKENS_CSS: &str = r#"
:root {
  /* --- Official brand palette --- */
  --fazora-obsidian: #0B0D10;
  --fazora-carbon: #15191F;
  --fazora-gold: #D8B56A;
  --fazora-ivory: #F4F1E9;
  --fazora-steel: #89919D;
  --fazora-teal: #78CFC1;

  /* --- Semantic tokens --- */
  --background: var(--fazora-obsidian);
  --surface-primary: var(--fazora-carbon);
  --surface-secondary: #1C2129;
  --surface-tertiary: #242A33;

  --text-primary: var(--fazora-ivory);
  --text-secondary: #AEB4BC;
  --text-muted: var(--fazora-steel);
  --text-inverse: var(--fazora-obsidian);

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

  --radius-small: 6px;
  --radius-medium: 10px;
  --radius-large: 16px;
  --radius-xl: 22px;

  --space-1: 4px;
  --space-2: 8px;
  --space-3: 12px;
  --space-4: 16px;
  --space-5: 20px;
  --space-6: 24px;
  --space-8: 32px;
  --space-10: 40px;
  --space-12: 48px;

  --font-brand: "Manrope", -apple-system, BlinkMacSystemFont, "Segoe UI",
    Roboto, "Helvetica Neue", Arial, "Noto Sans", sans-serif;
  --font-ui: "Manrope", -apple-system, BlinkMacSystemFont, "Segoe UI",
    Roboto, "Helvetica Neue", Arial, "Noto Sans", sans-serif;
  --font-persian: "Vazirmatn", -apple-system, BlinkMacSystemFont, "Segoe UI",
    Roboto, "Helvetica Neue", Arial, "Noto Sans", sans-serif;
  --font-mono: "IBM Plex Mono", ui-monospace, SFMono-Regular, Menlo, Monaco,
    Consolas, monospace;

  --text-xs: 0.75rem;
  --text-sm: 0.875rem;
  --text-base: 1rem;
  --text-lg: 1.125rem;
  --text-xl: 1.25rem;
  --text-2xl: 1.5rem;
  --text-3xl: 1.875rem;
  --text-4xl: 2.25rem;
  --text-5xl: 3rem;

  --leading-none: 1.2;
  --leading-tight: 1.35;
  --leading-normal: 1.6;

  --transition-fast: 120ms cubic-bezier(0.16, 1, 0.3, 1);
  --transition-base: 180ms cubic-bezier(0.16, 1, 0.3, 1);
  --transition-slow: 240ms cubic-bezier(0.16, 1, 0.3, 1);
  --ease-out: cubic-bezier(0.16, 1, 0.3, 1);
  --ease-in-out: cubic-bezier(0.4, 0, 0.2, 1);

  --sidebar-width: 260px;
  --sidebar-compact-width: 72px;
  --topbar-height: 56px;
  --player-dock-height: 88px;
  --secondary-panel-width: 360px;

  --z-dropdown: 100;
  --z-modal: 200;
  --z-tooltip: 300;
  --z-toast: 400;
}

/* --- Reset & base --- */
* {
  box-sizing: border-box;
  margin: 0;
  padding: 0;
}

html, body, #root {
  width: 100%;
  height: 100%;
  background: var(--background);
  color: var(--text-primary);
  font-family: var(--font-ui);
  font-size: var(--text-base);
  line-height: var(--leading-normal);
  -webkit-font-smoothing: antialiased;
  overflow: hidden;
}

body {
  font-family: var(--font-ui);
  direction: ltr;
}

body[dir="rtl"] {
  font-family: var(--font-persian);
}

/* Typography scale */
h1, h2, h3, h4, h5, h6 {
  font-family: var(--font-brand);
  font-weight: 700;
  letter-spacing: -0.02em;
  color: var(--text-primary);
  line-height: var(--leading-tight);
}

h1 { font-size: var(--text-5xl); }
h2 { font-size: var(--text-3xl); }
h3 { font-size: var(--text-2xl); }
h4 { font-size: var(--text-xl); }
h5 { font-size: var(--text-lg); }
h6 { font-size: var(--text-base); }

p { margin: 0 0 var(--space-4) 0; color: var(--text-secondary); }
a { color: var(--text-inherit, inherit); text-decoration: none; }

.fz-text-muted { color: var(--text-muted); }
.fz-text-secondary { color: var(--text-secondary); }
.fz-text-accent { color: var(--accent); }

/* --- Layout primitives --- */
.fz-app-shell {
  display: flex;
  flex-direction: column;
  width: 100%;
  height: 100%;
  background: var(--background);
}

.fz-body {
  display: flex;
  flex: 1 1 auto;
  overflow: hidden;
}

.fz-sidebar {
  width: var(--sidebar-width);
  min-width: var(--sidebar-width);
  background: var(--surface-primary);
  border-right: 1px solid var(--border);
  display: flex;
  flex-direction: column;
  z-index: 10;
}

.fz-sidebar.collapsed {
  width: var(--sidebar-compact-width);
  min-width: var(--sidebar-compact-width);
}

.fz-sidebar-overlay {
  position: fixed;
  inset: 0;
  background: rgba(11, 13, 16, 0.6);
  z-index: 50;
  opacity: 0;
  pointer-events: none;
  transition: opacity var(--transition-base);
}

.fz-sidebar-overlay.open {
  opacity: 1;
  pointer-events: auto;
}

.fz-topbar {
  height: var(--topbar-height);
  background: var(--surface-primary);
  border-bottom: 1px solid var(--border);
  display: flex;
  align-items: center;
  padding: 0 var(--space-4);
  flex-shrink: 0;
  z-index: 20;
}

.fz-content {
  flex: 1 1 auto;
  min-width: 0;
  overflow-y: auto;
  padding: var(--space-6);
  background: var(--background);
}

.fz-page {
  max-width: 1400px;
  margin: 0 auto;
  width: 100%;
}

.fz-player-dock {
  height: var(--player-dock-height);
  background: var(--surface-primary);
  border-top: 1px solid var(--border);
  flex-shrink: 0;
  z-index: 15;
}

.fz-secondary-panel {
  width: var(--secondary-panel-width);
  min-width: var(--secondary-panel-width);
  background: var(--surface-primary);
  border-left: 1px solid var(--border);
  display: flex;
  flex-direction: column;
  flex-shrink: 0;
}

/* Scrollbar */
::-webkit-scrollbar {
  width: 10px;
  height: 10px;
}
::-webkit-scrollbar-track {
  background: transparent;
}
::-webkit-scrollbar-thumb {
  background: rgba(137, 145, 157, 0.25);
  border-radius: 999px;
  border: 2px solid transparent;
  background-clip: content-box;
}
::-webkit-scrollbar-thumb:hover {
  background: rgba(137, 145, 157, 0.4);
}

/* --- Focus & accessibility --- */
:focus-visible {
  outline: 2px solid var(--accent);
  outline-offset: 2px;
}

.fz-sr-only {
  position: absolute;
  width: 1px;
  height: 1px;
  padding: 0;
  margin: -1px;
  overflow: hidden;
  clip: rect(0, 0, 0, 0);
  white-space: nowrap;
  border: 0;
}

.fz-visually-hidden:focus {
  position: static;
  width: auto;
  height: auto;
  padding: var(--space-2);
  margin: 0;
  overflow: visible;
  clip: auto;
  white-space: normal;
}

/* Reduced motion */
@media (prefers-reduced-motion: reduce) {
  *, *::before, *::after {
    animation-duration: 0.01ms !important;
    animation-iteration-count: 1 !important;
    transition-duration: 0.01ms !important;
    scroll-behavior: auto !important;
  }
}

/* RTL: mirror spacing */
body[dir="rtl"] .fz-sidebar { border-right: none; border-left: 1px solid var(--border); }

/* --- Selection --- */
::selection {
  background: var(--accent);
  color: var(--accent-text);
}
"#;

pub const COMPONENTS_CSS: &str = r#"
/* --- Button primitives --- */
.fz-button {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  height: 36px;
  padding: 0 var(--space-4);
  border: none;
  border-radius: var(--radius-medium);
  font-family: inherit;
  font-size: var(--text-sm);
  font-weight: 600;
  cursor: pointer;
  transition: background var(--transition-fast), color var(--transition-fast),
    transform var(--transition-fast), box-shadow var(--transition-fast);
  text-decoration: none;
  gap: var(--space-2);
}

.fz-button:disabled {
  opacity: 0.45;
  cursor: not-allowed;
}

.fz-button-icon {
  width: 36px;
  height: 36px;
  padding: 0;
  border-radius: var(--radius-medium);
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 0;
}

.fz-button-primary {
  background: var(--accent);
  color: var(--accent-text);
}
.fz-button-primary:not(:disabled):hover {
  background: var(--accent-hover);
}
.fz-button-primary:not(:disabled):active {
  background: var(--accent-active);
}

.fz-button-surface {
  background: var(--surface-tertiary);
  color: var(--text-primary);
  border: 1px solid var(--border);
}
.fz-button-surface:not(:disabled):hover {
  background: rgba(244, 241, 233, 0.1);
  border-color: rgba(244, 241, 233, 0.2);
}

.fz-button-ghost {
  background: transparent;
  color: var(--text-secondary);
}
.fz-button-ghost:not(:disabled):hover {
  background: rgba(244, 241, 233, 0.06);
  color: var(--text-primary);
}

.fz-button-danger {
  background: transparent;
  color: var(--danger);
  border: 1px solid rgba(228, 124, 124, 0.35);
}
.fz-button-danger:not(:disabled):hover {
  background: rgba(228, 124, 124, 0.1);
}

/* --- Badge --- */
.fz-badge {
  display: inline-flex;
  align-items: center;
  height: 20px;
  padding: 0 7px;
  border-radius: 999px;
  font-size: var(--text-xs);
  font-weight: 700;
  background: var(--surface-tertiary);
  color: var(--text-muted);
  border: 1px solid var(--border);
}
.fz-badge-accent {
  background: var(--accent-muted);
  color: var(--accent);
  border-color: rgba(216, 181, 106, 0.35);
}

/* --- Card --- */
.fz-card {
  background: var(--surface-secondary);
  border: 1px solid var(--border);
  border-radius: var(--radius-large);
  overflow: hidden;
}
.fz-card:hover {
  border-color: rgba(244, 241, 233, 0.16);
}

/* --- Inputs --- */
.fz-input {
  width: 100%;
  height: 40px;
  padding: 0 var(--space-4);
  background: var(--surface-tertiary);
  border: 1px solid var(--border);
  border-radius: var(--radius-medium);
  color: var(--text-primary);
  font: inherit;
  transition: border-color var(--transition-fast), box-shadow var(--transition-fast);
}
.fz-input:focus {
  border-color: var(--accent);
  outline: none;
}
.fz-input::placeholder {
  color: var(--text-muted);
}

/* --- Toggle --- */
.fz-toggle {
  position: relative;
  width: 44px;
  height: 24px;
  background: var(--surface-tertiary);
  border-radius: 999px;
  cursor: pointer;
}
.fz-toggle .fz-toggle-thumb {
  position: absolute;
  top: 2px; left: 2px;
  width: 20px; height: 20px;
  background: var(--text-primary);
  border-radius: 999px;
  transition: transform var(--transition-base);
}

/* --- Slider (generic, used by volume/timeline/eq) --- */
.fz-slider {
  position: relative;
  height: 4px;
  width: 100%;
  background: rgba(244, 241, 233, 0.16);
  border-radius: 999px;
  cursor: pointer;
  outline: none;
}
.fz-slider-track-fill {
  position: absolute;
  inset: 0;
  height: 100%;
  background: var(--accent);
  border-radius: 999px;
  transform-origin: left;
}
.fz-slider-thumb {
  position: absolute;
  width: 14px; height: 14px;
  background: var(--accent);
  border: 2px solid var(--surface-primary);
  border-radius: 50%;
  cursor: pointer;
}

/* --- Divider --- */
.fz-divider {
  height: 1px;
  background: var(--divider);
  border: none;
  margin: var(--space-6) 0;
}

/* --- Tooltip --- */
.fz-tooltip {
  position: relative;
  display: inline-flex;
}
.fz-tooltip .fz-tooltip-content {
  position: absolute;
  bottom: calc(100% + 8px);
  left: 50%;
  transform: translatex(-50%);
  padding: var(--space-2) var(--space-3);
  background: var(--text-primary);
  color: var(--fazora-obsidian);
  font-size: var(--text-xs);
  font-weight: 600;
  border-radius: var(--radius-small);
  white-space: nowrap;
  opacity: 0;
  pointer-events: none;
  transition: opacity var(--transition-fast), transform var(--transition-fast);
  transform: translatex(-50%) translatey(6px);
}
.fz-tooltip:hover .fz-tooltip-content {
  opacity: 1;
  transform: translatex(-50%) translatey(0);
}

/* --- Empty / loading / error states --- */
.fz-empty, .fz-loading, .fz-error {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
  text-align: center;
  padding: var(--space-12);
  color: var(--text-muted);
}
.fz-empty h2, .fz-loading h2, .fz-error h2 {
  font-size: var(--text-xl);
  color: var(--text-primary);
  margin-bottom: var(--space-2);
}
.fz-spinner {
  width: 32px; height: 32px;
  border: 2px solid var(--border);
  border-top-color: var(--accent);
  border-radius: 50%;
  animation: fz-spin 0.8s linear infinite;
}
@keyframes fz-spin { to { transform: rotate(360deg); } }

.fz-list {
  display: flex;
  flex-direction: column;
  gap: var(--space-3);
}
.fz-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
  gap: var(--space-5);
}

/* Section headers */
.fz-section {
  margin-bottom: var(--space-8);
}
.fz-section-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: var(--space-4);
}
.fz-section-title {
  font-size: var(--text-lg);
  font-weight: 700;
  letter-spacing: -0.01em;
}
.fz-section-actions {
  display: flex;
  gap: var(--space-2);
}

/* Table (track list) */
.fz-table {
  width: 100%;
  border-collapse: collapse;
}
.fz-table th {
  text-align: left;
  font-size: var(--text-xs);
  font-weight: 600;
  color: var(--text-muted);
  padding: var(--space-3) var(--space-4);
  border-bottom: 1px solid var(--border);
}
.fz-table td {
  padding: var(--space-3) var(--space-4);
  border-bottom: 1px solid var(--divider);
  font-size: var(--text-sm);
  color: var(--text-secondary);
}
.fz-table tr:hover td {
  background: rgba(244, 241, 233, 0.04);
}

/* Mobile bottom navigation */
.fz-bottom-nav {
  display: none;
  height: 72px;
  background: var(--surface-primary);
  border-top: 1px solid var(--border);
  display: flex;
  justify-content: space-around;
  align-items: center;
  flex-shrink: 0;
  z-index: 40;
}
.fz-bottom-nav-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 2px;
  color: var(--text-muted);
  font-size: var(--text-xs);
  font-weight: 600;
  min-width: 64px;
}
.fz-bottom-nav-item.active { color: var(--accent); }


/* --- Premium dashboard and compact mobile header --- */
.fz-icon-sprite {
  position: absolute;
  width: 0;
  height: 0;
  overflow: hidden;
  pointer-events: none;
}

.fz-topbar-spacer {
  flex: 1 1 280px;
  min-width: 0;
  max-width: 560px;
  margin-inline: var(--space-5);
}
.fz-topbar-actions {
  display: flex;
  align-items: center;
  flex: 0 0 auto;
}
.fz-home { max-width: 1280px; }
.fz-home-hero {
  display: flex;
  align-items: flex-end;
  justify-content: space-between;
  gap: var(--space-5);
  margin-bottom: var(--space-8);
}
.fz-home-title {
  font-size: clamp(2rem, 5vw, 3.25rem);
  font-weight: 750;
  letter-spacing: -0.045em;
  line-height: 1.05;
}
.fz-home-subtitle {
  margin: var(--space-2) 0 0;
  color: var(--text-secondary);
  font-size: var(--text-base);
}
.fz-home-actions {
  display: flex;
  align-items: center;
  gap: var(--space-2);
  flex-wrap: wrap;
}
.fz-stat-grid {
  display: grid;
  grid-template-columns: repeat(4, minmax(0, 1fr));
  gap: var(--space-3);
  margin-bottom: var(--space-8);
}
.fz-stat-card {
  min-width: 0;
  min-height: 112px;
  padding: var(--space-4);
  display: flex;
  align-items: center;
  gap: var(--space-3);
  background: linear-gradient(145deg, var(--surface-secondary), #171b22);
  border: 1px solid var(--border);
  border-radius: var(--radius-large);
  transition: border-color var(--transition-fast), transform var(--transition-fast);
}
.fz-stat-card:hover {
  border-color: rgba(216, 181, 106, 0.3);
  transform: translateY(-1px);
}
.fz-stat-icon {
  width: 44px;
  height: 44px;
  flex: 0 0 44px;
  display: grid;
  place-items: center;
  border-radius: 13px;
  color: var(--accent);
  background: var(--accent-muted);
}
.fz-stat-value {
  color: var(--text-primary);
  font-size: clamp(1.4rem, 3vw, 2rem);
  font-weight: 750;
  line-height: 1.1;
  font-variant-numeric: tabular-nums;
}
.fz-stat-label {
  margin-top: 6px;
  color: var(--text-muted);
  font-size: var(--text-xs);
  font-weight: 650;
  letter-spacing: .075em;
  text-transform: uppercase;
}
.fz-home-section { margin-bottom: var(--space-8); }
.fz-home-section-title {
  margin-bottom: var(--space-3);
  font-size: var(--text-xl);
  font-weight: 700;
}
.fz-home-empty {
  display: flex;
  align-items: center;
  gap: var(--space-4);
  padding: var(--space-5);
  min-height: 104px;
  background: var(--surface-primary);
  border: 1px solid var(--border);
  border-radius: var(--radius-large);
}
.fz-home-empty-icon {
  width: 48px;
  height: 48px;
  flex: 0 0 48px;
  display: grid;
  place-items: center;
  border-radius: 14px;
  color: var(--accent);
  background: var(--accent-muted);
}
.fz-home-empty-title { color: var(--text-primary); font-weight: 650; }
.fz-home-empty-copy { margin: 4px 0 0; color: var(--text-muted); font-size: var(--text-sm); }
.fz-recent-list { display: grid; gap: var(--space-2); }
.fz-recent-item {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  padding: var(--space-3) var(--space-4);
  background: var(--surface-primary);
  border: 1px solid var(--border);
  border-radius: var(--radius-medium);
}
.fz-recent-art {
  width: 40px;
  height: 40px;
  display: grid;
  place-items: center;
  flex: 0 0 40px;
  border-radius: 10px;
  color: var(--accent);
  background: var(--accent-muted);
}
.fz-recent-title { color: var(--text-primary); font-weight: 600; }
.fz-recent-subtitle { color: var(--text-muted); font-size: var(--text-xs); }

@media (max-width: 900px) {
  .fz-stat-grid { grid-template-columns: repeat(2, minmax(0, 1fr)); }
}
@media (max-width: 768px) {
  .fz-topbar {
    height: auto;
    min-height: 60px;
    flex-wrap: wrap;
    gap: var(--space-2);
    padding: 10px var(--space-3);
  }
  .fz-topbar-spacer {
    order: 3;
    flex: 1 0 100%;
    max-width: none;
    margin: 0;
  }
  .fz-topbar-spacer .fz-input { width: 100%; min-width: 0; }
  .fz-topbar-actions { margin-left: auto; gap: var(--space-1); }
  .fz-home-hero {
    align-items: flex-start;
    flex-direction: column;
    gap: var(--space-4);
    margin-bottom: var(--space-5);
  }
  .fz-home-actions { width: 100%; }
  .fz-home-actions .fz-button { flex: 1 1 auto; min-width: 0; }
  .fz-stat-grid { gap: var(--space-2); margin-bottom: var(--space-6); }
  .fz-stat-card { min-height: 92px; padding: var(--space-3); gap: var(--space-3); }
  .fz-stat-icon { width: 38px; height: 38px; flex-basis: 38px; }
  .fz-home-section { margin-bottom: var(--space-6); }
}
@media (max-width: 380px) {
  .fz-topbar { padding-inline: 10px; }
  .fz-home-title { font-size: 2rem; }
  .fz-stat-card { padding: 10px; gap: 9px; }
  .fz-stat-icon { width: 34px; height: 34px; flex-basis: 34px; }
  .fz-stat-label { font-size: 10px; }
}

@media (max-width: 768px) {
  .fz-sidebar { position: fixed; top: 0; bottom: 0; z-index: 60;
    transform: translatex(-100%); transition: transform var(--transition-base); }
  .fz-sidebar.open { transform: translatex(0); }
  .fz-content { padding: var(--space-4); }
  .fz-body { flex-direction: column; }
  .fz-bottom-nav { display: flex; }
  .fz-grid { grid-template-columns: repeat(auto-fill, minmax(150px, 1fr)); }
}
"#;
