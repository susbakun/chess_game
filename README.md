# ♟️ Chess Game

A chess game written in Rust and Bevy.

## Screenshot

<img width="916" height="837" alt="Screenshot 2026-05-03 at 11 14 26 PM" src="https://github.com/user-attachments/assets/c1a0b660-8dd1-4eaa-807a-9ec981335af8" />


## Features

- Full chess rule implementation
- Interactive piece movement and capture
- Move validation and turn-based gameplay
- Clean UI

## Getting Started

### Prerequisites

- Rust (latest stable version)
- Cargo
- **Stockfish** (required for AI opponent)

### Installation & Running

#### Installing Stockfish

This game requires **Stockfish** to be installed on your system. The game will automatically search for it, but you need to install it first. Follow the instructions for your operating system:

#### macOS (Homebrew)
```bash
brew install stockfish
```

#### Linux(Ubuntu/Debian)
```bash
sudo apt-get install stockfish
```

#### Windows
1. Download the latest Stockfish executable from [stockfishchess.org/download]()
2. Extract the files to a location (e.g., C:\Program Files\stockfish)
3. Add the installation directory to your system PATH, or the game will search common installation locations automatically



### Running the project
```bash
# Clone the repository
git clone https://github.com/susbakun/chess_game.git
cd chess_game

# Run the game
cargo run --release
```


## TODO

- [ ] Multiplayer support
- [x] Adding chess engine
- [x] Implement a system that keeps track of the taken pieces and displays them, maybe in UI or as objects next to the board.
- [x] Finish implementing all the rules: castling, check mates.
- [x] Show the result on the UI (instead of console).
- [x] Replay button when the game's over.
- [ ] Play with timer
- [x] WASM

## Development

Built with:
- [Rust](https://www.rust-lang.org/)
- [Bevy](https://bevyengine.org/)


## References

- [Bevy Chess Tutorial](https://caballerocoll.com/blog/bevy-chess-tutorial/) - Excellent reference for chess game implementation in Bevy

