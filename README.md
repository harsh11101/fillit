# Lemme Do It

A modern desktop application for efficient text manipulation and clipboard management, built with Tauri v2, React, and TypeScript.

## Overview

Lemme Do It is a lightweight, cross-platform desktop utility that enhances your text workflow with HTML-aware clipboard operations and intelligent text processing. Whether you're cleaning up copied content, formatting text, or managing clipboard data with HTML preservation, Lemme Do it provides a fast and intuitive interface.

### Key Features

- **Smart Clipboard Management**: Copy, paste, and clear clipboard with HTML support
- **Text Processing**: Add text with or without HTML formatting
- **Performance Optimized**: Singleton pattern prevents thread spawning issues
- **Cross-Platform**: Works seamlessly on Windows, macOS, and Linux
- **Modern UI**: Built with shadcn/ui and Tailwind CSS

## Screenshots

### Dashboard - Main Interface
![Dashbaord - Main Interface](<Screenshot 2025-10-27 143239.png>)

### Adding Text Without HTML
![Adding Text without HTML](<Screenshot 2025-10-27 143155.png>)

### Adding Text With HTML
![Adding Text with HTML](<Screenshot 2025-10-27 143223.png>)

## Architecture

### Tech Stack

**Frontend**
- React 18 with TypeScript
- shadcn/ui component library
- Tailwind CSS for styling
- Vite for blazing-fast builds

**Backend**
- Tauri v2 (Rust-based)
- clipboard_rs for clipboard operations
- Singleton pattern for resource management

### Project Structure

```
LemmeDoIt/
├── src/                            # React frontend
│   ├── components/
│   │   └── ui/                     # shadcn/ui components
│   ├── App.tsx                     # Main application component
│   ├── main.tsx                    # React entry point
│   └── index.css                   # Global styles
│
├── src-tauri/                      # Rust backend
│   ├── src/                        # All rust files handling bankend
│   │   └── command.rs              # Commands to run from frontend
│   │   └── config.rs               # Database Handles
│   │   ├── clipboard_handler.rs    # Core clipboard logic
│   │   └── keyboard_handler.rs     # Core Keyboard logic
│   │   └── main.rs                 # Tauri application setup
│   ├── Cargo.toml                  # Rust dependencies
│   └── tauri.conf.json             # Tauri configuration
│
├── package.json
└── README.md
```

## Design Patterns

### Singleton Pattern
The clipboard context uses a singleton pattern (`OnceLock`) to ensure only one clipboard instance exists throughout the application lifecycle. This prevents:
- Excessive thread creation
- Memory leaks
- Race conditions on clipboard operations

### Command Pattern
Tauri commands (`#[tauri::command]`) provide a clean separation between frontend and backend, enabling type-safe IPC communication.

### Result-Based Error Handling
All clipboard operations return `Result<T, String>` for explicit error handling and user-friendly error messages.

## Getting Started

### Development

```bash
# Start development server with hot reload
bun run tauri:dev
# or
npm run tauri:dev
#or
pnpm run tauri:dev
```

The application will open automatically. Changes to frontend code will hot-reload, while Rust changes require a restart.


## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

Lemme Do It is [MIT licensed](LICENSE)

## Acknowledgments

- Built with [Tauri](https://tauri.app/)
- UI components from [shadcn/ui](https://ui.shadcn.com/)
- Clipboard handling by [clipboard_rs](https://crates.io/crates/clipboard-rs)

---

**Made with ❤️ by harsh11101**