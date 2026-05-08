# ♟️ Chess Game

A chess game written in Rust and Bevy.

## Screenshot

<img width="916" height="837" alt="Screenshot 2026-05-03 at 11 14 26 PM" src="https://github.com/user-attachments/assets/c1a0b660-8dd1-4eaa-807a-9ec981335af8" />


## Features

- Full chess rule implementation(except the ones in the [todo section](#todo))
- Interactive piece movement and capture
- Move validation and turn-based gameplay
- Clean UI

## Getting Started

### Prerequisites

- Rust (latest stable version)
- Cargo

### Installation & Running

```bash
# Clone the repository
git clone https://github.com/susbakun/chess_game.git
cd chess_game

# Run the game
cargo run --release
```


## TODO

- [ ] Multiplayer support
- [ ] Adding chess engine
- [x] Implement a system that keeps track of the taken pieces and displays them, maybe in UI or as objects next to the board.
- [x] Finish implementing all the rules: castling, check mates.
- [ ] Try moving the camera around!.
- [ ] Show the result on the UI (instead of console)
- [ ] WASM

## Development

Built with:
- [Rust](https://www.rust-lang.org/)
- [Bevy](https://bevyengine.org/)


## References

- [Bevy Chess Tutorial](https://caballerocoll.com/blog/bevy-chess-tutorial/) - Excellent reference for chess game implementation in Bevy

