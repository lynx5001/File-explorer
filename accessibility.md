# Accessibility Implementation Rules

## Screen Reader Strategy
*   **Delegation:** Never use a Custom Text-to-Speech (TTS) engine.
*   **Structure:** Use `role="tree"` for the main file list and `role="treeitem"` for entries.
*   **States:** Always update `aria-selected`, `aria-expanded`, and `aria-busy` states.

## Focus Management
*   When a folder is opened, move focus to the first item in the new list.
*   When a popup (Search/Context Menu) closes, restore focus to the previous element.
*   Use "Type-to-jump" (A-Z) to move focus within a directory.

## Metadata
*   Filename is the primary label.
*   Size and Date must be linked via `aria-describedby` so they aren't read unless the user requests more info.