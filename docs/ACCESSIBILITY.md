# Accessibility

FAZORA is designed to be operable by keyboard, screen readers and assistive
technology on all platforms.

---

## 1. Keyboard navigation

- All interactive components are reachable via Tab in logical order.
- Sidebar, grids, lists and tables support arrow-key navigation where
  appropriate.
- Space/Enter activate buttons; Escape closes dialogs, overlays and
  fullscreen; media keys (play/pause, next, previous) are honoured.
- Focus never gets trapped; modals return focus to the trigger on close.

## 2. Focus indicators

` :focus-visible` renders a 2px gold outline with 2px offset on every
interactive element. Custom components (sliders, toggles, chip groups)
declare `tabindex="0"` and explicit keyboard handlers.

## 3. Semantics and screen readers

- Buttons use native `<button>` with `aria-label` where icons carry no text.
- Toggles use `role="switch"` with `aria-checked`.
- Slider/range inputs carry `aria-label` and `aria-valuenow`.
- Active states expose `aria-selected="true"` / `aria-checked="true"`.
- The application shell defines landmark regions: navigation, main, region
  (now playing), plus `aria-label` on each.
- `.fz-sr-only` class provides screen-reader-only text for icon-only
  controls.
- Page titles announce the current destination on route change.

## 4. Contrast

- Text on surfaces meets WCAG AA contrast (ivory on obsidian/carbon).
- Gold accent (#D8B56A) on obsidian is used for focus and selection — never
  as the sole carrier of meaning; state changes are always accompanied by
  non-color indicators (icon change, fill change, text label).
- Selection and focus are visible without relying on color alone.

## 5. Typography

- Font sizes scale with system settings; relative units (rem) throughout.
- Line height ≥ 1.5 for body; letter-spacing tracked for headings.
- Persian/Arabic UI uses Vazirmatn with `dir="rtl"`; numbers and technical
  data remain left-aligned where appropriate.

## 6. Reduced motion

`@media (prefers-reduced-motion: reduce)` collapses all transitions and
animations to 0.01ms and disables scroll-behavior smoothness. The waveform
and spectrum visualisers respect the setting by drawing static or minimally
animated content.

## 7. Touch accessibility

- Minimum touch target 48dp on mobile; icon buttons are 44×44dp minimum.
- No hover-dependent interactions.
- Bottom sheets for mobile contextual operations.
- Gestures do not interfere with OS navigation (swipe edges reserved).

## 8. Error handling

Error states include:
- A plain-language heading describing what happened.
- A useful next action (button or link).
- Technical details available in diagnostics, never raw native errors shown
  directly.
- `role="alert"` or `aria-live="polite"` on toast notifications.

## 9. High-DPI and scaling

- SVG icons and artwork scale crisply at 1×, 2× and 3×.
- Layout uses logical pixels; widths are percentage- or constraint-based,
  never fixed pixel values that break at 200% scaling.

## 10. Validation checklist

- [ ] Every screen reachable and operable with keyboard only.
- [ ] Focus visible and never lost.
- [ ] Screen reader labels on all icon-only controls.
- [ ] Contrast verified on all text/background combinations.
- [ ] RTL mode functional (Persian interface).
- [ ] Reduced motion respected.
- [ ] Touch targets ≥ 48dp on mobile layouts.
- [ ] Error states present on every major screen with next actions.
