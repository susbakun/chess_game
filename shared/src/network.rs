use renet::ConnectionConfig;
use serde::{Deserialize, Serialize};

use crate::{game_event::GameEvent, GameState, PieceColor};

#[derive(Serialize, Deserialize)]
pub enum ClientMessage {
    SendEvent(GameEvent),
}

#[derive(Serialize, Deserialize)]
pub enum ServerMessage {
    SyncState(GameState),
    SyncTimer(u16, u16),
    AssignColor(PieceColor),
}

pub fn connection_config() -> ConnectionConfig {
    ConnectionConfig::default()
}
