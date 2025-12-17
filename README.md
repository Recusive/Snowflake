<p align="center">
  <a href="https://snowflake.build" target="_blank">
    <img src="extra/images/logo.png" width="180" height="180" alt="Snowflake"/>
  </a>
</p>

<h1 align="center">Snowflake</h1>

<p align="center">
  <strong>Lightning-fast, modern code editor built in Rust</strong>
</p>

<p align="center">
  <a href="https://snowflake.build">
    <img src="https://img.shields.io/static/v1?label=Website&message=snowflake.build&color=E07A6B&style=flat-square" alt="Website">
  </a>
  <a href="https://github.com/Recusive/Snowflake/releases">
    <img src="https://img.shields.io/github/v/release/Recusive/Snowflake?color=C4A98B&style=flat-square&label=Release" alt="Release">
  </a>
  <a href="https://github.com/Recusive/Snowflake/blob/main/LICENSE">
    <img src="https://img.shields.io/badge/License-Apache%202.0-8B7355?style=flat-square" alt="License">
  </a>
</p>

---

**Snowflake** is a high-performance code editor designed for developers who value speed, simplicity, and a beautiful coding experience. Built entirely in Rust with GPU-accelerated rendering, Snowflake delivers instant response times and a smooth, native feel.

## Features

- **Blazing Fast** — Written in pure Rust with rope-based text handling for instant operations on files of any size
- **GPU-Accelerated** — Smooth 60fps rendering powered by wgpu
- **Intelligent Editing** — Built-in LSP support for completions, diagnostics, and code actions
- **Modal Editing** — First-class Vim-style keybindings (toggleable)
- **Remote Development** — Work seamlessly on remote machines
- **Extensible** — Plugin system supporting Rust, C, and AssemblyScript via WASI
- **Integrated Terminal** — Full terminal emulator built-in

## Installation

### Download

Get the latest release for your platform from [GitHub Releases](https://github.com/Recusive/Snowflake/releases).

### Build from Source

```bash
git clone https://github.com/Recusive/Snowflake.git
cd Snowflake
cargo build --release
cargo run --release -p snowflake-app
```

**Requirements:** Rust 1.87.0+

## Contributing

We welcome contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

Apache License 2.0 — see [LICENSE](LICENSE)

---

<p align="center">
  <strong>Recursive Labs</strong><br>
  <a href="https://snowflake.build">snowflake.build</a>
</p>
