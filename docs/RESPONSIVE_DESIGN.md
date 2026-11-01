# Responsive Design

FAZORA uses intentionally different desktop and mobile experiences — desktop
layouts are never forced onto Android.

---

## 1. Breakpoints

| Breakpoint | Max width | Experience |
|---|---|---|
| Mobile | 768px | Bottom navigation, stacked layout, compact player |
| Tablet | 769–1024px | Adaptive: bottom nav + optional side panels |
| Desktop | 1025px+ | Sidebar shell, topbar, player dock |

CSS is injected via `src/design/tokens.css`; the `@media (max-width: 768px)`
block handles mobile overrides.

---

## 2. Desktop layout

- Fixed collapsible sidebar (260px / 72px compact).
- Topbar 56px.
- Player dock 88px persistent at the bottom.
- Main content fills the remaining space; max-width 1400px page container.
- Secondary panel 360px, overlay or side panel depending on width.
- Overlay backdrop when the sidebar opens on smaller desktop windows.

---

## 3. Mobile layout (Android)

- Bottom navigation with five primary destinations: Home, Music, Videos,
  Library, Settings. Active destination in gold.
- Secondary destinations via menus and in-page navigation.
- Mini-player above the bottom navigation: artwork thumbnail, title,
  artist, play/pause.
- Fullscreen music player: large artwork, lyrics access, waveform, queue
  access, gesture support.
- Video player: fullscreen native surface, minimal overlays, gesture-based
  controls (tap to toggle, horizontal swipe seek, vertical swipe volume/
  brightness, double tap seek, pinch zoom, long press speed).

### Touch requirements

- Minimum touch target 48dp — no tiny icon buttons on mobile.
- No hover-dependent interactions on touch.
- Bottom sheets for mobile contextual operations.
- Navigation state preserved across rotation.
- Android back behavior respected.

---

## 4. Tablet and foldable support

When sufficient width is available (≥ 1024px), tablets render the desktop
shell with the sidebar expanded and secondary panels shown side-by-side.
Between breakpoints the layout degrades gracefully: sidebar becomes an
overlay drawer, grid columns reduce (`minmax(150px, 1fr)` on mobile).

---

## 5. Adaptive behaviors

- Grid columns: `repeat(auto-fill, minmax(200px, 1fr))` desktop;
  `minmax(150px, 1fr)` mobile.
- Sidebar collapses to icons-only below tablet width unless manually pinned.
- Player dock stays persistent on desktop; on mobile the mini-player sits
  above bottom navigation and the expanded player is fullscreen.
- Touch targets scale with UI density setting (compact / comfortable /
  spacious).

---

## 6. Validation checklist

- [ ] Sidebar collapsible and icon-only mode works.
- [ ] Secondary panel hideable; no permanent screen space wasted.
- [ ] Bottom navigation reachable one-handed on mobile.
- [ ] All touch targets ≥ 48dp.
- [ ] No hover-only controls on mobile.
- [ ] Layouts remain usable at 200% system scaling / HiDPI.
- [ ] RTL layout (Persian) mirrors navigation without breaking media
  controls or timelines.
