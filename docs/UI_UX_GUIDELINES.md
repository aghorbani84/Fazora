# FAZORA UI/UX Guidelines

Design philosophy: **Luxury Engineering × Minimalism × High-Performance
Multimedia**. The product must feel engineered, refined, fast, powerful and
effortless to operate.

---

## 1. Application shell (desktop)

```
[ Sidebar ] [ Main content ] [ Optional secondary panel ]
[ Persistent player dock ]
```

- **Sidebar:** main destinations (Home, Music, Albums, Artists, Tracks,
  Genres, Playlists, Videos, Folders, Favorites, Recently Played, History,
  Statistics, Search, Settings). Collapsible to icon-only compact mode with
  tooltips. Active destination indicated with a gold indicator line/fill.
- **Topbar:** FAZORA wordmark, global search, device selector, settings.
- **Main content:** scrollable, max-width 1400px centered container.
- **Secondary panel:** queue, lyrics, or details — 360px, hideable/resizable.
- **Player dock:** persistent, 88px — artwork thumbnail, title/artist,
  play/pause, next, queue, lyrics, timeline, volume.

---

## 2. Navigation

Keyboard-navigable with visible focus indicators. Current location always
obvious (highlighted sidebar item + page title). Avoid deep nesting — flat
destinations with in-page filtering/sorting instead.

Drag-and-drop playlist organization is supported in the queue panel.

---

## 3. Home dashboard

Data-driven only — no fictitious media or fake statistics. Sections hide when
they have no data:

- **Continue Listening** — recently played tracks.
- **Continue Watching** — video playback progress.
- **Recently Added** — newly indexed media.
- **Your Favorites** — favorite tracks, albums, playlists.
- **Most Played** — play-count ranked content.
- **Discover Your Library** — metadata-driven collections.
- **Quick Actions** — Open File, Open Folder, Scan Media, Create Playlist,
  Open URL, Resume Playback.

---

## 4. Music library

Album grid cards show cover art, title, artist, year, track count. Hover
interactions are subtle; no excessive shadows or oversized cards. Interactions
per card: open, play, shuffle, add to queue, add to playlist, context menu.

Artist page: artwork, name, album/track counts, top tracks, albums, play and
shuffle actions.

Album page: large artwork, title, artist, year, genre, track table with
duration, playback actions.

Track list: multi-selection, selection indicators, play, add to queue,
add to playlist, favorite, metadata editor, file location, properties,
sorting.

---

## 5. Music player

**Persistent mini-player:** artwork thumbnail, title, artist, favorite,
playback controls, progress, duration, volume, queue, lyrics.

**Expanded Now Playing:** large artwork as the main visual focus, dynamic
background derived from artwork colors, track/artist info, waveform seekbar,
playback controls, queue panel, lyrics panel, favorite actions.

### Playback states (all visualised)

Idle, Loading, Playing, Paused, Buffering, Seeking, Ended, Error. Controls
reflect the real engine state — never fake states.

---

## 6. Soundstage (audio experience)

Equalizer: 10 bands (32 Hz – 16 kHz), dB values, preamp, presets (Flat,
Bass Boost, Vocal, Classical, Electronic, Jazz, Rock, Custom), custom preset
saving, enabled/disabled state. Frequency visualisation, audio output device
selection, ReplayGain, crossfade, gapless playback. Pointer, keyboard and
touch supported.

---

## 7. Lyrics

Synchronised lyrics with current-line highlighting and smooth auto-scroll
following playback. Manual scrolling preserved when appropriate. Font
customisation, sync adjustment, fullscreen mode. Missing lyrics handled with
an empty state. Restrained animations.

---

## 8. Video library

Thumbnail or list view, folder browsing, recently watched, continue watching
with progress bar, favorites, metadata, sort/filter/search. Video cards show
thumbnail, title, duration, resolution, watched progress, context menu. No
invented posters — thumbnails come from actual media only.

---

## 9. Video player

Immersive, distraction-free: fullscreen video surface, minimal top overlay,
center playback controls, timeline with buffered region, volume, subtitle and
audio track selection, playback speed, aspect ratio, fullscreen controls.
Advanced panel shows decoder info, renderer, resolution, frame rate, codec,
subtitle config, hardware acceleration.

Keyboard: Space play/pause, Left/Right seek, Up/Down volume, F fullscreen,
M mute, S subtitles, A audio track, Escape exit fullscreen. Shortcuts
configurable.

---

## 10. Queue and playlist

Current track, upcoming tracks, reorder by drag-and-drop, remove, clear, save
as playlist, play next, queue history, shuffle, repeat. Queue operations
update the real playback service — no UI-only fake queue.

---

## 11. Search

Global search across tracks, artists, albums, videos, playlists, folders,
lyrics and metadata. Suggestions, recent queries, filter chips, keyboard
focus, category navigation, empty results, loading feedback. Non-blocking.

---

## 12. Settings

Categories: General, Appearance, Audio, Video, Library, Playback, Advanced,
About. Organised and searchable. No unsupported engine settings exposed.
Settings persist to disk (settings.json in platform config dir).

---

## 13. Interaction and motion

- 120–180ms small interactions; 180–240ms panels/menus; 240–320ms page
  transitions.
- Opacity, subtle scale, panel movement, waveform progression.
- No bouncing, neon glow, large animated backgrounds, constant motion,
  expensive blurs, or animations that delay user actions.
- Respect reduced motion.

---

## 14. States and errors

Every major screen: initial loading, empty, no results, permission denied,
unsupported media, missing artwork, missing metadata, playback failure,
network unavailable, storage unavailable, engine unavailable, database
recovery, offline, background indexing, background playback.

Errors are informative: what happened + useful next action. Raw native errors
never shown to ordinary users; technical details live in diagnostics.

---

## 15. Performance

Virtualised long lists, lazy-loaded artwork, cached thumbnails, bounded
background work, no UI-thread blocking, focused Dioxus components, efficient
event handling. Native video rendering uses the proven playback architecture
— never simulated video.

---

## 16. Platform specifics

**Linux:** keyboard shortcuts, system theme integration, Wayland/X11,
desktop notifications, MPRIS, native file dialogs, media keys.

**Windows:** native window behavior, shortcuts, system media controls, file
associations, HiDPI scaling, native dialogs.

**Android:** bottom navigation (Home, Music, Videos, Library, Settings),
mini-player above bottom nav, fullscreen player, gesture controls, 48dp
minimum touch targets, bottom sheets for contextual operations, back behavior
respected, rotation state preserved.

FAZORA keeps its own visual identity on every platform — it never imitates
the OS.
