# ♟️ Chess Game

A chess game written in Rust and Bevy, with local offline play, an AI opponent, and local-network multiplayer.

## Screenshot

<img width="916" height="837" alt="Screenshot 2026-05-03 at 11 14 26 PM" src="https://github.com/user-attachments/assets/c1a0b660-8dd1-4eaa-807a-9ec981335af8" />


## Features

- Full chess rule implementation
- Interactive piece movement and capture
- Move validation and turn-based gameplay
- Local multiplayer over a network (server-authoritative, two players)
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

# Offline / AI mode — just run the client
cargo run -p client --release
```

#### Playing multiplayer

Multiplayer requires the server to be running first, then two client instances to connect to it. Currently this only works on `localhost` (both players on the same machine) or over a LAN if you point the client at the server's local IP — it is not deployable over the public internet yet.

```bash
# Terminal 1 — start the server
cargo run -p server --release

# Terminal 2 and 3 — start a client for each player
cargo run -p client --release
```

Select "Play Multiplayer" from the in-game menu in each client window. The first to connect plays White, the second plays Black.

> **Note:** multiplayer is native-only and unavailable in the WASM/web build.


## TODO

- [x] Multiplayer support (local/LAN, two-player, server-authoritative)
- [x] Adding chess engine
- [x] Implement a system that keeps track of the taken pieces and displays them, maybe in UI or as objects next to the board.
- [x] Finish implementing all the rules: castling, check mates.
- [x] Show the result on the UI (instead of console).
- [x] Replay button when the game's over.
- [x] Play with timer (only on offline mode)
- [x] WASM
- [ ] Multiplayer over the internet (currently localhost/LAN only)
- [ ] Reconnection handling after a disconnect mid-game

## Development

Built with:
- [Rust](https://www.rust-lang.org/)
- [Bevy](https://bevyengine.org/)
- [renet](https://github.com/lucaspoffo/renet) - networking for the multiplayer server/client


## References

- [Bevy Chess Tutorial](https://caballerocoll.com/blog/bevy-chess-tutorial/) - Excellent reference for chess game implementation in Bevy
- [Making a Turn-Based Multiplayer Game in Rust](https://herluf-ba.github.io/making-a-turn-based-multiplayer-game-in-rust-01-whats-a-turn-based-game-anyway.html) - Reference for the client/server/shared architecture and the validate/consume event pattern used for multiplayer
