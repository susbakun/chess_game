use bevy::prelude::*;

use crate::engine::*;
use crate::pieces::*;
use crate::player::*;

#[derive(PartialEq)]
pub enum GameType {
    PlayOnline,
    PlayWithAi,
    PlayOffline,
}

#[derive(Resource)]
pub struct GameState {
    pub game_over: bool,
    pub winner: Option<PieceColor>,
    pub player: Player,
    pub game_type: Option<GameType>,
    pub is_loading: bool,
    // counting removed pieces (whites, blacks)
    pub removed_counts: (u8, u8),
    #[cfg(not(target_arch = "wasm32"))]
    pub engine: Option<StockfishEngine>,
    #[cfg(not(target_arch = "wasm32"))]
    pub difficulty: Option<u32>,
}

#[cfg(not(target_arch = "wasm32"))]
impl Default for GameState {
    fn default() -> Self {
        Self {
            game_over: false,
            winner: None,
            player: Player::default(),
            game_type: None,
            is_loading: false,
            removed_counts: (0, 0),
            engine: None,
            difficulty: None,
        }
    }
}

#[cfg(target_arch = "wasm32")]
impl Default for GameState {
    fn default() -> Self {
        Self {
            game_over: false,
            winner: None,
            player: Player::default(),
            game_type: None,
            is_loading: false,
            removed_counts: (0, 0),
        }
    }
}

impl GameState {
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

    pub fn is_multiplayer(&self) -> bool {
        self.game_type
            .as_ref()
            .is_some_and(|gt| *gt == GameType::PlayOnline)
    }
}

pub fn is_loading_multiplayer(game_state: Res<GameState>) -> bool {
    game_state.is_loading && game_state.is_multiplayer()
}

pub fn back_to_main_menu(game_state: Res<GameState>) -> bool {
    game_state.game_type.is_none()
}
