use bevy::prelude::*;

use crate::engine::*;

use shared::*;

#[derive(Resource, Deref, DerefMut)]
pub struct ClientGameState {
    #[deref]
    pub shared: shared::GameState,
    // counting removed pieces (whites, blacks)
    pub removed_counts: (u8, u8),
    #[cfg(not(target_arch = "wasm32"))]
    pub engine: Option<StockfishEngine>,
    #[cfg(not(target_arch = "wasm32"))]
    pub difficulty: Option<u32>,
}

#[cfg(not(target_arch = "wasm32"))]
impl Default for ClientGameState {
    fn default() -> Self {
        Self {
            shared: shared::GameState::default(),
            removed_counts: (0, 0),
            engine: None,
            difficulty: None,
        }
    }
}

#[cfg(target_arch = "wasm32")]
impl Default for ClientGameState {
    fn default() -> Self {
        Self {
            shared: shared::GameState::default(),
            removed_counts: (0, 0),
        }
    }
}

impl ClientGameState {
    pub fn change_turn(&mut self) {
        self.player.0 = match self.player.0 {
            PieceColor::White => PieceColor::Black,
            PieceColor::Black => PieceColor::White,
        }
    }

    pub fn set_winner(&mut self, winner: PieceColor) {
        self.winner = Some(winner);
    }

    pub fn toggle_game_over(&mut self) {
        self.game_over = !self.game_over;
    }
}
