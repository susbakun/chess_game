use bevy::prelude::*;

use crate::pieces::*;
use crate::player::*;

#[derive(Resource)]
pub struct GameState {
    pub game_over: bool,
    pub winner: Option<PieceColor>,
    pub player: Player,
    // counting removed pieces (whites, blacks)
    pub removed_counts: (u8, u8)
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            game_over: false,
            winner: None,
            player: Player::default(),
            removed_counts: (0, 0)
        }
    }
}

impl GameState {
    pub fn change_turn(&mut self) {
        self.player.0 = match self.player.0 {
            PieceColor::White => PieceColor::Black,
            PieceColor::Black => PieceColor::White
        }
    }

    pub fn set_winner(&mut self, winner: PieceColor) {
        self.winner = Some(winner);
    }

    pub fn toggle_game_over(&mut self) {
        self.game_over = !self.game_over;
    }
}