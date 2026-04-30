# Chess Game in Bevy

## Description

A 3D chess game built with **Rust** and **Bevy 0.18.1**. This project implements an interactive chess board with 3D pieces rendered using Bevy's rendering engine. The board features a proper 8×8 checkerboard pattern with alternating white and black squares, and chess pieces are loaded from GLB 3D models.

## Key Technical Details

### Board Generation
- Uses `Plane3d` meshes with half-extents of `Vec2::new(0.5, 0.5)` for 1×1 unit squares

### Piece Loading
- Pieces are loaded from a single GLB file using mesh handle paths
- Example: `asset_server.load("models/pieces.glb#Mesh0/Primitive0")`
- Child entity spawning with `with_children()` for composite pieces

### Camera Setup
- Uses quaternion rotation for precise isometric perspective
- Positioned at `Vec3::new(-7.0, 20.0, 4.0)` with normalized quaternion transformation

## Dependencies

```toml
[dependencies]
bevy = "0.18.1"
```

## Building and Running

```bash
cargo build --release
cargo run
```

## References

- [Bevy Chess Tutorial](https://caballerocoll.com/blog/bevy-chess-tutorial/) - Excellent reference for chess game implementation in Bevy