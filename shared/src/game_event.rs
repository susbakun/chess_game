use serde::{Deserialize, Serialize};

use crate::PieceColor;

type PlayerId = u64;

#[derive(Serialize, Deserialize)]
pub enum GameEvent {
    PlayerJoined {
        player_id: PlayerId,
        color: PieceColor,
    },
    BeginGame,
    MovePiece {
        from: (i8, i8),
        to: (i8, i8),
    },
    EndGame {
        winner: Option<PieceColor>,
    },
    PlayerDisconnected {
        player_id: PlayerId,
    },
}
