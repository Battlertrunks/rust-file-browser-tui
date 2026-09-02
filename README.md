# file-browser-tui

A terminal user interface (TUI) for exploring your folders and files, built with [Ratatui] and [Crossterm].

Started from the [Ratatui Hello World template].

## Features

- Lists the subdirectories of the current working directory as clickable boxes in a grid (up to 4 columns)
- Left-click a folder to navigate into it
- Terminal setup and teardown (alternate screen, raw mode, mouse capture) with a panic hook that restores the terminal on crash

## Controls

| Input | Action |
| --- | --- |
| Left mouse click on a folder | Navigate into that folder |
| `q` or `Esc` | Quit the app |

## Getting Started

Requires Rust (edition 2024 or newer, e.g. installed via [rustup]).

```sh
# Run in debug mode
cargo run

# Build an optimized release binary
cargo build --release
```

The app starts in the directory it was launched from and shows that directory's subdirectories.

## Project Structure

- `src/main.rs` — entry point; sets up the terminal and runs the main render/update loop
- `src/tui.rs` — terminal lifecycle: enter/exit the alternate screen, raw mode, mouse capture, and panic handling
- `src/app.rs` — application state, folder loading, and input handling
- `src/action.rs` — maps crossterm events to `Action`s (quit, key, mouse, resize)
- `src/components/folders_widget.rs` — renders the folder grid and registers clickable areas

## License

Copyright (c) Battlertrunks <gavinszczesniak@gmail.com>

This project is licensed under the MIT license ([LICENSE] or <http://opensource.org/licenses/MIT>)

[Ratatui]: https://ratatui.rs
[Crossterm]: https://github.com/crossterm-rs/crossterm
[Ratatui Hello World Template]: https://github.com/ratatui/templates/tree/main/hello-world
[rustup]: https://rustup.rs
[LICENSE]: ./LICENSE
