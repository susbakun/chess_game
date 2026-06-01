use spacetimedb::rand::random;
use spacetimedb::{Identity, ReducerContext, SpacetimeType, Table};

use crate::constants::DB_COLOR_WHITE;
use crate::db_piece::{DbPiece, initial_board};
use crate::game_table::GameStatus::*;
use crate::player_table::require_player;

#[derive(SpacetimeType, PartialEq)]
pub enum GameStatus {
    WaitForOpponent,
    InProgress,
    Finished,
}

#[spacetimedb::table(accessor = game, public)]
pub struct Game {
    #[primary_key]
    pub id: u64,
    pub white: Identity,
    pub black: Option<Identity>,
    pub board: Vec<DbPiece>,
    pub game_status: GameStatus,
    pub turn: u8,
    pub winner: Option<u8>,
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
        game_status: WaitForOpponent,
        turn: DB_COLOR_WHITE,
        winner: None,
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

    if game.game_status != WaitForOpponent {
        return Err("Game is not open to join".to_string());
    }

    game.black = Some(sender);
    game.game_status = InProgress;
    game.turn = DB_COLOR_WHITE;
    game.winner = None;
    ctx.db.game().id().update(game);

    log::info!("Player {:?} joined game {game_id} as black", sender);
    Ok(())
}
