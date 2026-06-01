use chess_core::PieceColor;
use spacetimedb_sdk::Identity;

use super::*;

#[derive(Resource)]
pub struct SpacetimeConnection {
    pub conn: DbConnection,
}

#[derive(Resource, Default)]
pub struct OnlineSession {
    pub connected: bool,
    pub subscription_applied: bool,
    pub local_identity: Option<Identity>,
    pub active_game_id: Option<u64>,
    pub local_color: Option<PieceColor>,
    pub waiting_for_opponent: bool,
}

impl OnlineSession {
    /// Connected, identity known, and player/game subscriptions are in the local cache.
    pub fn is_ready_for_lobby(&self) -> bool {
        self.connected && self.subscription_applied && self.local_identity.is_some()
    }

    pub fn is_in_active_game(&self) -> bool {
        self.active_game_id.is_some() && self.local_color.is_some() && !self.waiting_for_opponent
    }
}
