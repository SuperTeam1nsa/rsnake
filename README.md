[![GPLv3 License](https://img.shields.io/badge/License-GPL%20v3-yellow.svg)](https://opensource.org/licenses/)
[![Rust](https://github.com/FromTheRags/rsnake/actions/workflows/rust.yml/badge.svg)](https://github.com/FromTheRags/rsnake/actions/workflows/rust.yml)
[![crates.io](https://img.shields.io/crates/v/nom_du_crate.svg)](https://crates.io/crates/rsnake)
[![docs](https://img.shields.io/badge/docs-online-blue)](https://fromtherags.github.io/rsnake/rsnake/index.html)
[![Clippy](https://github.com/FromTheRags/rsnake/actions/workflows/doc.yml/badge.svg)](https://github.com/FromTheRags/rsnake/actions/workflows/doc.yml)
[![codecov](https://codecov.io/gh/fromtherags/rsnake/branch/main/graph/badge.svg)](https://codecov.io/gh/FromTheRags/rsnake)
[![Last Commit](https://img.shields.io/github/last-commit/fromtherags/rsnake)](https://github.com/FromTheRags/rsnake/commits)
[![Issues](https://img.shields.io/github/issues/fromtherags/rsnake)](https://github.com/FromTheRags/rsnake/issues)
[![Pull Requests](https://img.shields.io/github/issues-pr/fromtherags/rsnake)](https://github.com/FromTheRags/rsnake/pulls)

# Snake Game using Ratatui

It is a terminal-based snake game using the Ratatui crate for rendering.
![Terminal Output Menu](demo_images/snake_setup.png)
![Terminal Output Running](demo_images/snake_running.png)

## Features

- **Terminal UI**: Uses Ratatui for rendering a grid-based game.
- **Game Logic**: Manages snake movement, collisions, and scoring.
- **Multithreading**: Uses multiple threads for input handling, rendering at 60 FPS, and game logic execution.
- **Emoji-based graphics**: Supports rendering the snake using emojis instead of ASCII.
- **Configurable parameters**: With `clap` for command-line arguments.

## TODO

- [ ] Add a save score (local db) with a pseudo got from cmdline
- [ ] Add some performance log with tracing, for example,
  see https://github.com/ratatui/ratatui/blob/main/examples/apps/tracing/src/main.rs
- [ ] Show game options in the menu, and visually change them (tab, direction, using a Params struct, containing a vec
  of ButtonWidget, implementing a vec of them, each one displaying
  a list, prev/next, selected, and an internal index, so input just call function of it, in case of enter, start as
  it !), start entering s
- [ ] Enhance fruits eaten detection and grid management with multiple emojis as body

- ## 💖 Support
- If you like this game and want to contribute to more amazing features, consider giving me a coffee
  on [kofi](https://ko-fi.com/retrosnake)
  🥤or [![Donate](https://img.shields.io/badge/Donate-PayPal-blue.svg)](https://www.paypal.me/Rsnake42)
  to support development.

## Installation

### ✅ Prerequisites

- 🦀 **Have Rust and compilation tools installed**  
  If you don't have Rust yet:
- On **Windows**, Install Rust using the official .exe installer https://www.rust-lang.org/tools/install (as it works
  Out-Of-The-Box on
  windows)
- On **Linux**/[Android using Ubuntu](https://github.com/CypherpunkArmory/UserLAnd), See  
  👉 [Installation Rust and tools for Linux](#Installation-Rust-and-tools-for-Linux) for instructions

- 💻 **Use a terminal that supports emoji**
    - On **Windows**, the default terminal supports emoji out of the box.
    - On **Linux** / [Android using Ubuntu](https://github.com/CypherpunkArmory/UserLAnd), install the Noto Emoji font.
      See  
      👉 [Emoji font support](#Enable-Emoji-Font-Support) for instructions.

### For this project

- Clone this repository
  `git clone <url>`
- Go to the directory `cd rsnake`

### Run the Game

- To run the game, either:`cargo run` or `cargo run --manifest-path rsnake/Cargo.toml` (if in an another directory)
- To see run options, use: `rsnake --help`
- E.g., `rsnake -z 🐼 -b 🍥` or `cargo run -- -z 🐼 -b 🍥`
- To install the game as a command:  
  `cargo install --path .`  
  And then run the game with: `rsnake`

## Architecture

- Uses `Arc` & `RwLock` for synchronization.
- Spawns separate threads for input handling, rendering (60Hz), and game logic execution.

## Documentation generation

- `cargo doc --document-private-items --no-deps --open`

## Tests

- As usual run them with `cargo test` the project is set up with a lib containing all the code, and a main.rs just
  calling it
- As this is a widespread pattern providing full compliance with the Rust test ecosystem, allowing doc comment to be
  automatically tested, for example.
- To have a coverage report, install `llvm-cov`:
  `rustup component add llvm-tools-preview
  cargo install cargo-llvm-cov`
- And run `cargo llvm-cov --open`
- A great coverage is not a goal for this project (tests are only there to showcase tests in rust),
- For reference the current coverage
  is :[![codecov](https://codecov.io/gh/FromTheRags/rsnake/graph/badge.svg?token=XJXII0CMQF)](https://codecov.io/gh/FromTheRags/rsnake)

## Installation Rust and tools for Linux

Make sure your system has `curl`, `gcc` and `git` installed:

```bash
sudo apt update
sudo apt install curl git gcc -y
```

Use the official installer `rustup`, or any alternative method on https://www.rust-lang.org/tools/install (by your own):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

- Follow the prompts in the terminal.
- After installation, run:

```bash
echo "source '$HOME/.cargo/env'" >> ~/.bashrc
source ~/.bashrc
```

- Verify the installation:

```bash
rustc --version
```

---

## Enable Emoji Font Support

To properly display emoji characters in your terminal and system fonts, install an emoji-compatible font.

### For Ubuntu/Debian-based distros:

```bash
sudo apt install fonts-noto-color-emoji
```

### For Arch Linux:

```bash
sudo pacman -S noto-fonts-emoji
```

### For Fedora:

```bash
sudo dnf install google-noto-emoji-color-fonts
```

---

## Optional: Configure Font Fallback (if emojis still do not render)

Create or edit the following file:

```bash
~/.config/fontconfig/fonts.conf
```

Add:

```xml
<?xml version="1.0"?>
<!DOCTYPE fontconfig SYSTEM "fonts.dtd">
<fontconfig>
    <alias>
        <family>sans-serif</family>
        <prefer>
            <family>Noto Color Emoji</family>
        </prefer>
    </alias>
</fontconfig>
```

Then refresh the font cache:

```bash
fc-cache -f -v
```

---

## Test Your Setup

Run:

```bash
echo "Rust is awesome! 🦀🔥🚀"
```

You should see emojis rendered correctly in your terminal or text editors.

- Then follow quick installation instructions

## References

- Clippy lints: <https://github.com/rust-lang/rust-clippy/>
- Ratatui tutorial: <https://ratatui.rs/tutorials/hello-world/>
- Example: <https://ratatui.rs/examples/widgets/canvas/>

[![Contributors](https://img.shields.io/github/contributors/user/repo)](https://github.com/user/repo/graphs/contributors)
