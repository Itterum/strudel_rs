# strudel_rs

**strudel_rs** is a TUI application written in Rust, inspired by the original [Strudel](https://strudel.cc/) project (AGPL-3.0 licensed).  
This is an independent from-scratch port featuring a terminal-based live coding music environment in the style of TidalCycles, powered by Ratatui.

Built using the [Ratatui](https://ratatui.rs).

## Features
- Interactive REPL in the terminal for patterns and cycles.
- Support for core primitives: `note`, `sound`, `stack`, `slowcat`, etc.
- Fully terminal-based TUI with syntax highlighting and input.
- Cargo-ready: just `cargo run` to start.

## Quick Start
```bash
git clone https://github.com/itterum/strudel_rs.git
cd strudel_rs
cargo run
```
Enter patterns like `note("c d e f")` and hit Enter to play.

## Differences from Original
- **Rust + TUI**: Native terminal interface instead of web REPL.
- **Performance**: Optimized for Linux (Pop!_OS, Arch, etc.).
- **Dependencies**: Ratatui, crossterm + minimal audio backend (e.g., rodio or cpal).

## License
MIT License. See the [LICENSE](./LICENSE) file or [opensource.org/licenses/MIT](http://opensource.org/licenses/MIT).  
The original Strudel uses AGPL-3.0 — this port is independent and does not copy any code.

## Contributing
PRs welcome! Fork, add audio engines or SuperCollider integration.  
Use issues for bugs/ideas.