use crate::constants::*;
use crate::{PieceColor, Player};

#[derive(PartialEq)]
pub enum GameType {
    PlayWithAi,
    PlayOffline,
}

pub struct GameState {
    pub game_over: bool,
    pub winner: Option<PieceColor>,
    pub player: Player,
    pub game_type: Option<GameType>,
    pub timer: (u16, u16),
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            game_over: false,
            winner: None,
            player: Player::default(),
            game_type: None,
            // 20 minutes each
            timer: (TIMER_DURATION_SECS, TIMER_DURATION_SECS),
        }
    }
}
