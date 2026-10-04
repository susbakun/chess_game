use bevy::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
use crate::engine::*;

#[derive(PartialEq, Clone)]
pub enum GameType {
    PlayWithAi,
    PlayOffline,
    Multiplayer,
}

#[derive(Resource, Deref, DerefMut)]
pub struct ClientGameState {
    #[deref]
    pub shared: shared::GameState,
    pub game_type: Option<GameType>,
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
            game_type: None,
            difficulty: None,
        }
    }
}

#[cfg(target_arch = "wasm32")]
impl Default for ClientGameState {
    fn default() -> Self {
        Self {
            shared: shared::GameState::default(),
            game_type: None,
            removed_counts: (0, 0),
        }
    }
}
