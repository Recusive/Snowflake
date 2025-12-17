# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build Commands

```bash
# Development build
cargo build

# Release build
cargo build --release

# Release with LTO (optimized)
cargo build --profile release-lto

# Fast development (optimized deps, debug app code)
cargo build --profile fastdev

# Run the editor
cargo run --release -p snowflake-app

# Format code
cargo fmt --all

# Run clippy lints
cargo clippy

# Run doc tests
cargo test --doc --workspace

# Run benchmarks
cargo bench -p snowflake-app
```

### macOS-specific (requires signing identity)

```bash
make binary          # Build native binary
make binary-universal # Build universal binary (x86_64 + arm64)
make app             # Create Snowflake.app bundle
make dmg             # Create disk image
```

### Ubuntu dependencies

```bash
sudo make ubuntu-deps
```

## Architecture

Snowflake is a GPU-accelerated code editor built in Rust. The workspace consists of four main crates:

### snowflake-app

The GUI application built on **Floem** (a reactive UI framework). Key modules:

- `app.rs` - Application entry point, window management, and main event loop
- `window_tab.rs` - Tab/workspace management within windows
- `main_split.rs` - Editor split pane logic
- `editor.rs` - Core editor view and editing logic
- `doc.rs` - Document/buffer management in the UI layer
- `palette.rs` - Command palette implementation
- `config/` - User configuration handling with hot-reload
- `panel/` - Side panels (file explorer, search, terminal, plugins, debug)
- `terminal/` - Integrated terminal using alacritty_terminal

### snowflake-proxy

Backend process handling file I/O, LSP, plugins, and terminals. Runs as a separate process for remote development support.

- `dispatch.rs` - Main message dispatcher, handles all proxy requests/notifications
- `plugin/` - Plugin system using WASI (wasmtime) for sandboxed plugin execution
  - `lsp.rs` - LSP client implementation
  - `dap.rs` - Debug Adapter Protocol
  - `wasi.rs` - WASM plugin runtime
- `terminal.rs` - PTY/terminal management
- `buffer.rs` - File buffer management (reading, writing, change tracking)

### snowflake-rpc

RPC protocol definitions between app and proxy:

- `core.rs` - Messages from proxy to app (notifications, responses)
- `proxy.rs` - Messages from app to proxy (requests, notifications)
- `stdio.rs` - JSON-RPC transport over stdin/stdout

### snowflake-core

Shared utilities and core functionality:

- `syntax/` - Tree-sitter based syntax highlighting (adapted from Helix)
- `language.rs` - Language definitions and tree-sitter grammar loading
- `directory.rs` - Platform-specific directory paths (config, data, cache)
- Re-exports `floem_editor_core` for buffer/cursor primitives

## Key Dependencies

- **floem** - Reactive UI framework (from lapce/floem repo)
- **tree-sitter** - Syntax parsing and highlighting
- **lsp-types** - Language Server Protocol types
- **alacritty_terminal** - Terminal emulation
- **wasmtime** - WASM runtime for plugins
- **git2** - Git integration

## App-Proxy Communication

The app spawns a proxy process and communicates via JSON-RPC over stdin/stdout. This separation enables:

- Remote development (proxy runs on remote machine)
- Isolated plugin execution
- Non-blocking UI (heavy operations in proxy)

## Plugin System

Plugins are WASM modules loaded via wasmtime with WASI support. Plugin metadata comes from `plugins.lapce.dev`. Plugins can provide:

- Language servers (LSP)
- Debug adapters (DAP)
- Custom commands
