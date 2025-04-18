# Snake Game using Ratatui

It is a terminal-based snake game using the Ratatui crate for rendering.

## Features

- **Terminal UI**: Uses Ratatui for rendering a grid-based game.
- **Game Logic**: Manages snake movement, collisions, and scoring.
- **Multithreading**: Uses multiple threads for input handling, rendering at 60 FPS, and game logic execution.
- **Emoji-based graphics**: Supports rendering the snake using emojis instead of ASCII.
- **Configurable parameters**: With `clap` for command-line arguments.

## TODO

- [ ] Add a save score (local db) with a pseudo got from cmdline
- [ ] Use Velocity value in game
- [ ] Improve 60 FPS accuracy with precise timing and configuration.
- [ ] Add some performance log with tracing for example
- [ ] Some tests example
- [ ] Fix too much life display outside of screen
- [ ] Some performance improvement running [flamegraph](https://github.com/flamegraph-rs/flamegraph)

## References

- Clippy lints: <https://github.com/rust-lang/rust-clippy/>
- Ratatui tutorial: <https://ratatui.rs/tutorials/hello-world/>
- Example: <https://ratatui.rs/examples/widgets/canvas/>

## Architecture

- Uses `Arc` & `RwLock` for synchronization.
- Spawns separate threads for input handling, rendering (60Hz), and game logic execution.

## Documentation generation

- `cargo doc --document-private-items --no-deps --open`
