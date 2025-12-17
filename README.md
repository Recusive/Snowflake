<h1 align="center">
  <a href="https://snowflake.build" target="_blank">
  <img src="extra/images/logo.png" width=200 height=200/><br>
  Snowflake
  </a>
</h1>

<h4 align="center">Lightning-fast and Powerful Code Editor</h4>

<div align="center">
  <a href="https://snowflake.build" target="_blank">
    <img src="https://img.shields.io/static/v1?label=Website&message=snowflake.build&color=blue" alt="Snowflake Website">
  </a>
  <a href="https://github.com/Recusive/Snowflake/releases" target="_blank">
    <img src="https://img.shields.io/github/v/release/Recusive/Snowflake?label=Release" alt="Release">
  </a>
  <a href="https://github.com/Recusive/Snowflake/blob/main/LICENSE" target="_blank">
    <img src="https://img.shields.io/badge/License-Apache%202.0-blue.svg" alt="License">
  </a>
</div>
<br/>

**Snowflake** is a lightning-fast, modern code editor built entirely in Rust. Designed for performance and extensibility, Snowflake delivers a seamless coding experience with intelligent features powered by LSP and a beautiful, responsive UI.

<p align="center">
  <i>Built by <a href="https://snowflake.build">Recursive Labs</a></i>
</p>

![Snowflake Screenshot](https://github.com/Recusive/Snowflake/blob/main/extra/images/screenshot.png?raw=true)

## Features

- **Blazing Fast Performance** - Written in pure Rust with [Rope Science](https://xi-editor.io/docs/rope_science_00.html) for lightning-fast text manipulation
- **GPU-Accelerated Rendering** - Powered by [wgpu](https://github.com/gfx-rs/wgpu) for smooth, responsive UI
- **Intelligent Code Completion** - Built-in LSP support for completions, diagnostics, and code actions
- **Modal Editing** - First-class Vim-style editing (toggleable)
- **Remote Development** - Seamlessly work on remote machines with full local experience
- **Plugin System** - Extend functionality with WASI-compatible plugins (Rust, C, AssemblyScript)
- **Integrated Terminal** - Execute commands without leaving the editor

## Installation

### Pre-built Releases

Download the latest release for your platform:
- [macOS](https://github.com/Recusive/Snowflake/releases)
- [Windows](https://github.com/Recusive/Snowflake/releases)
- [Linux](https://github.com/Recusive/Snowflake/releases)

### Build from Source

```bash
# Clone the repository
git clone https://github.com/Recusive/Snowflake.git
cd Snowflake

# Build release version
cargo build --release

# Run
cargo run --release -p snowflake-app
```

#### Requirements
- Rust 1.87.0 or later
- Platform-specific dependencies (see [Building Guide](docs/building-from-source.md))

## Documentation

- [Building from Source](docs/building-from-source.md)
- [Installing with Package Manager](docs/installing-with-package-manager.md)

## Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

## License

Snowflake is released under the [Apache License 2.0](LICENSE).

---

<p align="center">
  <b>Recursive Labs</b><br>
  <a href="https://snowflake.build">snowflake.build</a>
</p>
