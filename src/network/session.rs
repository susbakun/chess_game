use bevy::prelude::*;
use chess_core::PieceColor;
use spacetimedb_sdk::{DbContext, Table};

use super::{OnlineSession, SpacetimeConnection};
use crate::game_state::GameState;
use crate::module_bindings::Game;
use crate::module_bindings::game_table::GameTableAccess;
use crate::module_bindings::player_table::PlayerTableAccess;

/// Keeps [`OnlineSession`] in sync with the SpacetimeDB client cache.
///
/// Runs after `frame_tick` so subscription updates are visible before we read them.
pub fn sync_online_session(
    connection: Res<SpacetimeConnection>,
    mut session: ResMut<OnlineSession>,
) {
    let identity = match connection.conn.try_identity() {
        Some(id) => id,
        None => return,
    };

    session.connected = true;
    session.local_identity = Some(identity);

    session.subscription_applied = connection
        .conn
        .db
        .player()
        .identity()
        .find(&identity)
        .is_some();

    if !session.subscription_applied {
        session.active_game_id = None;
        session.local_color = None;
        session.waiting_for_opponent = false;
        return;
    }

    if let Some((game_id, color, waiting)) =
        find_active_game(identity, connection.conn.db.game().iter())
    {
        session.active_game_id = Some(game_id);
        session.local_color = Some(color);
        session.waiting_for_opponent = waiting;
    } else {
        session.active_game_id = None;
        session.local_color = None;
        session.waiting_for_opponent = false;
    }
}

/// Ends the loading screen once the online session is ready for the lobby.
pub fn end_multiplayer_loading(mut game_state: ResMut<GameState>, session: Res<OnlineSession>) {
    if !game_state.is_loading || !game_state.is_multiplayer() {
        return;
    }

    if session.is_ready_for_lobby() {
        game_state.is_loading = false;
        info!(
            "Online session ready (game_id={:?}, color={:?}, waiting={})",
            session.active_game_id, session.local_color, session.waiting_for_opponent
        );
    }
}

pub fn reset_online_session(mut session: ResMut<OnlineSession>) {
    *session = OnlineSession::default();
}

fn find_active_game(
    identity: spacetimedb_sdk::Identity,
    games: impl Iterator<Item = Game>,
) -> Option<(u64, PieceColor, bool)> {
    let mut best: Option<(u64, PieceColor, bool)> = None;

    for game in games {
        let Some((color, waiting)) = player_slot_in_game(&game, identity) else {
            continue;
        };

        let replace = match &best {
            None => true,
            Some((best_id, _, best_waiting)) => {
                if *best_waiting && !waiting {
                    true
                } else if waiting && !*best_waiting {
                    false
                } else {
                    game.id > *best_id
                }
            }
        };

        if replace {
            best = Some((game.id, color, waiting));
        }
    }

    best
}

fn player_slot_in_game(
    game: &Game,
    identity: spacetimedb_sdk::Identity,
) -> Option<(PieceColor, bool)> {
    if game.white == identity {
        Some((PieceColor::White, game.black.is_none()))
    } else if game.black == Some(identity) {
        Some((PieceColor::Black, false))
    } else {
        None
    }
}
