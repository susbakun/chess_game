use std::path::PathBuf;

use bevy::prelude::*;
use spacetimedb_sdk::DbContext;

use super::SpacetimeConnection;
use crate::module_bindings::DbConnection;
use crate::module_bindings::game_table::gameQueryTableAccess;
use crate::module_bindings::player_table::{PlayerTableAccess, playerQueryTableAccess};

const SPACETIMEDB_URI: &str = "http://127.0.0.1:3000";
const DATABASE_NAME: &str = "chess-db";
const TOKEN_FILENAME: &str = "spacetimedb_token";

fn token_path() -> Option<PathBuf> {
    let exe = std::env::current_exe().ok()?;
    Some(exe.parent()?.join(TOKEN_FILENAME))
}

fn load_token() -> Option<String> {
    let path = token_path()?;
    let contents = std::fs::read_to_string(&path).ok()?;
    let trimmed = contents.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

fn save_token(token: &str) -> std::io::Result<()> {
    let path = token_path().ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::Other, "could not determine executable")
    })?;
    std::fs::write(path, token)
}

pub fn connect_to_spacetimedb(mut commands: Commands) {
    let token = load_token();

    let conn = DbConnection::builder()
        .with_uri(SPACETIMEDB_URI)
        .with_database_name(DATABASE_NAME)
        .with_token(token)
        .on_connect(|ctx, _identity, token| {
            if let Err(e) = save_token(token) {
                error!("Failed to save SpacetimeDB token: {e}");
            }
            info!("Connected to SpacetimeDB");
            ctx.subscription_builder()
                .on_applied(|ctx| {
                    if let Some(identity) = ctx.try_identity() {
                        if let Some(player) = ctx.db.player().identity().find(&identity) {
                            info!("Playing as: {}", player.username);
                            return;
                        }
                    }
                    info!("Player subscription applied");
                })
                .on_error(|_ctx, err| {
                    error!("Subscription error: {err}");
                })
                .add_query(|q| q.from.player())
                .add_query(|q| q.from.game())
                .subscribe();
        })
        .on_connect_error(|_ctx, err| error!("SpacetimeDB connection error: {err}"))
        .on_disconnect(|_ctx, err| {
            if let Some(e) = err {
                warn!("Disconnected from SpacetimeDB with error: {e}");
            } else {
                info!("Disconnected from SpacetimeDB");
            }
        })
        .build();

    match conn {
        Ok(conn) => {
            info!("SpacetimeDB connection initiated");
            commands.insert_resource(SpacetimeConnection { conn });
        }
        Err(e) => {
            error!("Failed to initiated SpacetimeDB connection: {e}")
        }
    }
}

pub fn process_spacetimedb_messages(connection: Res<SpacetimeConnection>) {
    if let Err(e) = connection.conn.frame_tick() {
        error!("SpacetimeDB frame_tick error: {e}")
    }
}

pub fn cleanup_network(mut commands: Commands, connection: Res<SpacetimeConnection>) {
    let _ = connection.conn.disconnect();
    commands.remove_resource::<SpacetimeConnection>();
    info!("Network cleaned up");
}
