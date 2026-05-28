use spacetimedb::rand::random;
use spacetimedb::{Identity, ReducerContext, Table};

use crate::db_piece::{DbPiece, initial_board};
use crate::player_table::require_player;

#[spacetimedb::table(accessor = game, public)]
pub struct Game {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    pub white: Identity,
    pub black: Option<Identity>,
    pub board: Vec<DbPiece>,
}

/// Host a new game. Caller becomes white; black is open until [`join_game`].
#[spacetimedb::reducer]
pub fn create_game(ctx: &ReducerContext) -> Result<(), String> {
    require_player(ctx)?;
    let id: u64 = random();

    let sender = ctx.sender();
    let game = ctx.db.game().insert(Game {
        id,
        white: sender,
        black: None,
        board: initial_board(),
    });

    log::info!("Game {} created by {:?}", game.id, sender);
    Ok(())
}

/// Join an existing game as black.
#[spacetimedb::reducer]
pub fn join_game(ctx: &ReducerContext, game_id: u64) -> Result<(), String> {
    require_player(ctx)?;

    let sender = ctx.sender();

    let Some(mut game) = ctx.db.game().id().find(game_id) else {
        return Err(format!("Game {game_id} not found"));
    };

    if game.white == sender {
        return Err("You are already white in this game".to_string());
    }

    if game.black.is_some() {
        return Err("Game is full".to_string());
    }

    game.black = Some(sender);
    ctx.db.game().id().update(game);

    log::info!("Player {:?} joined game {game_id} as black", sender);
    Ok(())
}
