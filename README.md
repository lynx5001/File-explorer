# 📁 Rust File Explorer

A fast, lightweight, and cross-platform **file explorer** built with a **Rust backend** for performance and a **React frontend** for a clean and responsive user interface.  
This project enables users to seamlessly browse, search, and manage files and directories through an intuitive and efficient UI.

---

## Project Overview

The goal of this project is to provide a modern, cross-platform file management tool with smooth UX and system-level performance.  
By leveraging **Rust** for the core logic and **React** for the interface, the app achieves both speed and usability.

---

## Key Features

- **File Operations**: Perform essential file tasks like **copy**, **move**, and **delete** to manage files directly from the UI.
- **Directory Navigation**: Traverse through file system hierarchies with a click-based, responsive interface.
- **Search Functionality**: Quickly locate files and folders using keyword-based search input.
- **Cross-Platform Compatibility**: Works on **Windows**, **macOS**, and **Linux** with consistent behavior and UI.
- **Error Handling & Feedback**: Gracefully manages invalid paths and read errors with user-visible error messages.
- **File Opening Support**: Open files using the system default application (platform-specific handling).
- **Minimalist UX**: Clean layout with intuitive navigation and file detail display (name, size, icon).

---

## 🛠️ Technology Stack

| Layer      | Technology     |
|------------|----------------|
| **Frontend** | egui, React (planned or external) |
| **Backend** | Rust |

---

### Requirements

- [Rust](https://rustup.rs/) (latest stable)
- Node.js + npm (if using React frontend)

### Run the Rust GUI App

```
bash
cargo run
```

This command launches the native file explorer UI using egui.

### Planned Enhancements
- [ ] Full React-based web frontend
- [ ] Multi-select and batch file operations
- [ ] File previews and additional metadata
- [ ] Desktop packaging via tauri or similar tools

### Learning outcomes
- Developed a native GUI using Rust + egui
- Implemented platform-aware file opening and path handling
- Practiced modular architecture with custom components (mod file_entry)
- Gained experience with cross-platform GUI development
