use petname::Generator;
use spacetimedb::{Identity, ReducerContext, Table};

#[spacetimedb::table(accessor = player, public)]
pub struct Player {
    #[primary_key]
    identity: Identity,
    #[unique]
    username: String,
    is_online: bool,
}

fn generate_username(ctx: &ReducerContext) -> String {
    let mut rng = ctx.rng();
    let petnames = petname::Petnames::default();
    petnames
        .generate(&mut rng, 3, "-")
        .unwrap_or_else(|| format!("player-{}", ctx.timestamp.to_micros_since_unix_epoch()))
}

#[spacetimedb::reducer(init)]
pub fn init(_ctx: &ReducerContext) {
    log::info!("Server module initilized");
}

#[spacetimedb::reducer(client_connected)]
pub fn identity_connected(ctx: &ReducerContext) {
    let sender = ctx.sender();
    if let Some(player) = ctx.db.player().identity().find(sender) {
        log::info!("Player '{}' reconnected", player.username);
        ctx.db.player().identity().update(Player {
            is_online: true,
            ..player
        });
    } else {
        let username = generate_username(ctx);
        log::info!("New player '{}' joined", username);
        ctx.db.player().insert(Player {
            identity: sender,
            username,
            is_online: true,
        });
    }
}

#[spacetimedb::reducer(client_disconnected)]
pub fn identity_disconnected(ctx: &ReducerContext) {
    let sender = ctx.sender();
    if let Some(player) = ctx.db.player().identity().find(sender) {
        log::info!("Player '{} disconnected", player.username);
        ctx.db.player().identity().update(Player {
            is_online: false,
            ..player
        });
    } else {
        log::warn!("Disconnect for unknown identity {:?}", sender)
    }
}

#[spacetimedb::reducer]
pub fn register_player(ctx: &ReducerContext, username: String) -> Result<(), String> {
    if username.is_empty() {
        return Err("Username must not be empty".to_string());
    }

    if username.len() > 32 {
        return Err("Username length must be 32 or less".to_string());
    }

    let sender = ctx.sender();

    if ctx
        .db
        .player()
        .username()
        .find(&username)
        .is_some_and(|p| p.identity != sender)
    {
        return Err(format!("{username} is already taken"));
    }

    if let Some(player) = ctx.db.player().identity().find(sender) {
        log::info!("Player '{}' renamed to '{}'", player.username, username);
        ctx.db
            .player()
            .identity()
            .update(Player { username, ..player });
        Ok(())
    } else {
        Err("Cannot rename: player not found. Connect first".to_string())
    }
}
