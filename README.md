<div align="center">

# RAT-PONG

**A retro pong game for your terminal, built with Rust and [ratatui](https://github.com/ratatui/ratatui).**

[![Rust](https://img.shields.io/badge/Rust-2024_edition-b7410e?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![ratatui](https://img.shields.io/badge/ratatui-0.29-blue?logo=data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIxMDAiIGhlaWdodD0iMTAwIj48dGV4dCB5PSI4MCIgZm9udC1zaXplPSI4MCI+8J+QgDwvdGV4dD48L3N2Zz4=)](https://github.com/ratatui/ratatui)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

https://github.com/user-attachments/assets/05828f48-6319-45d9-ad44-4255abec58c7

</div>

---

## Features

- **Local 2-player** — grab a friend and settle the score
- **60 FPS game loop** — smooth paddle and ball movement
- **Ball trail** — fading trail effect that tracks the ball's path (toggleable)
- **Collision sparks** — paddle hits produce a burst of yellow sparks
- **Render modes** — switch between block and dot markers on the fly
- **Adaptive layout** — the play area scales to your terminal size

## Controls

| Key | Action |
|---|---|
| `W` / `S` | Move left paddle up / down |
| `↑` / `↓` | Move right paddle up / down |
| `T` | Toggle ball trail |
| `M` | Toggle marker style (block / dot) |
| `Q` / `Esc` | Quit |

## Getting Started

```sh
# clone it
git clone https://github.com/tetra/RAT-PONG.git
cd RAT-PONG

# run it
cargo run --release
```

> Requires [Rust](https://rustup.rs/) (2024 edition).

## Project Structure

```
src/
├── main.rs    # entry point — sets up the terminal and launches the app
├── app.rs     # game loop, rendering chrome, input dispatch
├── pong.rs    # all game logic — paddles, ball, collisions, scoring, rendering
└── input.rs   # key event handling and input action mapping
```

## License

MIT
