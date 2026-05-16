# Accessible File Explorer

A cross-platform desktop file explorer built for blind and vision-impaired users. Designed to work seamlessly with existing screen readers (JAWS, VoiceOver, NVDA, Orca) without any custom audio output. Keyboard-first, fast, and distraction-free.

---

## Tech Stack

| Layer | Technology |
|---|---|
| Application shell | Tauri v2 |
| Backend / performance logic | Rust |
| UI / accessibility layer | HTML + CSS + JavaScript (semantic) |
| Accessibility tree | Native OS webview (WKWebView, WebView2, WebKitGTK) |
| File indexing | SQLite (via Rust) |
| Screen reader support | JAWS, VoiceOver, NVDA, Orca — via ARIA |

---

## Core Design Principles

- **Screen reader delegation** — the app produces no custom TTS or audio output. All announcements are handled by the user's existing screen reader via a correct semantic and ARIA structure.
- **One persistent layout** — current path is always at the top, file and folder contents always below. The screen reader never has to relearn the structure.
- **Keyboard first** — every action is reachable by shortcut. No mouse required.
- **Minimal UI surface** — no competing sidebars, panels, or toolbars. One focused region at a time.
- **Popup on demand** — search, context menu, and bookmarks appear as focused popups and dismiss back to the previous focus point.

---

## UI Model

### Layout (always)

```
[ Current location: Home / Documents / Projects ]
[ file or folder 1                              ]
[ file or folder 2                              ]
[ file or folder 3                              ]
[ ...                                           ]
```

The location breadcrumb is always the first element. Contents follow. This structure never changes regardless of browse or search state.

### Search bar (popup)
- Triggered by shortcut
- Focus jumps to input immediately on open
- Results load on `Enter` — no live update while typing
- Results replace the content area below the current path
- Each result shows: filename, full parent path, modified date, file type
- Ambiguous results listed top to bottom, ranked by relevance and recency
- A "more results" item appears at the bottom when results are truncated
- `Escape` dismisses and restores the previous content

### Context menu (popup, command palette style)
- Triggered by `Apps` key (Windows convention, familiar to JAWS users)
- Focus jumps to a text input immediately
- User types an operation (e.g. `cop`, `del`, `ren`) and suggestions appear live
- Arrow down to select a suggestion, `Enter` to execute
- If input is empty and user presses arrow down, all available operations are listed — for discoverability
- `Escape` dismisses without action

**v1 operations:**
- Open
- Copy
- Cut
- Paste
- Delete
- Rename
- Bookmark current path

### Bookmarks (popup)
- Triggered by shortcut
- List of saved locations, arrow to navigate, `Enter` to go there
- Bookmarks are created via the context menu ("Bookmark current path")
- `Escape` dismisses

---

## Keyboard Shortcuts (planned)

| Action | Shortcut |
|---|---|
| Navigate into folder | `Enter` |
| Go up one level | `Backspace` or `Alt + Up` |
| Open search bar | `Ctrl + F` |
| Open context menu | `Apps` key |
| Open bookmarks | `Ctrl + B` |
| Dismiss any popup | `Escape` |
| Move through file list | `Arrow Up / Down` |

---

## Accessibility Requirements

- `role="tree"` and `role="treeitem"` for the directory listing
- `role="searchbox"` for the search input
- `role="status"` or `aria-live="polite"` for search result updates and operation feedback
- `aria-live="assertive"` for errors only
- `aria-expanded` on folder items (collapsed / expanded state)
- `aria-describedby` for file metadata (size, date, type) per item
- `aria-label` on every interactive element — no unlabelled controls
- Focus moves automatically on popup open and returns to previous element on dismiss
- WCAG 2.2 AA compliance target

---

## Architecture

```
┌─────────────────────────────────────────┐
│              Input layer                │
│   Keyboard shortcuts · Apps key · Tab   │
└────────────────┬────────────────────────┘
                 │
┌────────────────▼────────────────────────┐
│            Tauri v2 backend (Rust)      │
│  File system access · SQLite index      │
│  Search & path resolution · FS watcher  │
└────────────────┬────────────────────────┘
                 │
┌────────────────▼────────────────────────┐
│         Frontend (HTML / ARIA)          │
│  Semantic structure · Focus management  │
│  ARIA live regions · Popup layer        │
└────────────────┬────────────────────────┘
                 │
┌────────────────▼────────────────────────┐
│        OS native webview                │
│  WKWebView · WebView2 · WebKitGTK       │
│  Exposes full accessibility tree        │
└────────────────┬────────────────────────┘
                 │
┌────────────────▼────────────────────────┐
│         Screen reader                   │
│  JAWS · VoiceOver · NVDA · Orca         │
│  Handles all audio output               │
└─────────────────────────────────────────┘
```

---

## v1 Scope

The first version focuses on core navigation and search. Everything listed below is in scope; nothing else is.

**In scope**
- Browse mode: navigate the file system with keyboard shortcuts
- Search popup: find files and folders by name or natural language description
- Context menu: command palette for file operations
- Bookmarks: save and jump to frequent locations
- Full screen reader compatibility via ARIA
- Cross-platform: Windows, macOS, Linux

**Explicitly out of scope for v1**
- Custom TTS or audio output
- Voice input
- Multi-file selection
- File preview
- Network / cloud storage locations
- Settings UI

---

## Platform Support

| Platform | Webview | Screen readers |
|---|---|---|
| Windows 10 / 11 | WebView2 (Edge) | JAWS, NVDA |
| macOS | WKWebView (Safari) | VoiceOver |
| Linux | WebKitGTK | Orca |
