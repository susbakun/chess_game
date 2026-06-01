use chess_core::{Piece, PieceType, Player};

use crate::constants::{DB_COLOR_BLACK, DB_COLOR_WHITE, DB_PIECE_QUEEN};
use crate::db_piece::{
    db_color_to_piece_color, db_piece_type_to_piece_type, piece_color_to_db, piece_type_to_db,
    DbPiece,
};
use crate::game_table::GameStatus::*;
use crate::player_table::require_player;

use spacetimedb::{ReducerContext, Table};

use crate::game_table::game;

/// Apply a move. The caller's identity must match the current turn (white/black).
#[spacetimedb::reducer]
pub fn make_move(
    ctx: &ReducerContext,
    game_id: u64,
    from_x: i8,
    from_y: i8,
    to_x: i8,
    to_y: i8,
    promotion: Option<u8>,
) -> Result<(), String> {
    require_player(ctx)?;

    let Some(mut game) = ctx.db.game().id().find(game_id) else {
        return Err(format!("Game {game_id} not found"));
    };

    if game.game_status != InProgress {
        return Err("Game is not in progress".to_string());
    }

    let sender = ctx.sender();
    let sender_color = player_color_in_game(&game, sender)?;

    if game.turn != sender_color {
        let name = if sender_color == DB_COLOR_WHITE {
            "white"
        } else {
            "black"
        };
        return Err(format!("It is not {name}'s turn"));
    }

    let pieces = active_pieces(&game.board)?;
    let moving_piece = pieces
        .iter()
        .find(|p| p.x == from_x && p.y == from_y && piece_color_to_db(p.color) == sender_color)
        .ok_or_else(|| format!("No piece at ({from_x}, {from_y})"))?
        .clone();

    let player = Player(db_color_to_piece_color(sender_color)?);
    let destination = (to_x, to_y);

    if !moving_piece.is_move_valid(destination, &player, &pieces) {
        return Err("Invalid move".to_string());
    }

    if moving_piece.piece_type == PieceType::Pawn {
        let promotes = (moving_piece.color == chess_core::PieceColor::White && to_x == 7)
            || (moving_piece.color == chess_core::PieceColor::Black && to_x == 0);
        if promotes {
            let promo = promotion.unwrap_or(DB_PIECE_QUEEN);
            db_piece_type_to_piece_type(promo)?;
        }
    }

    apply_move_to_board(
        &mut game.board,
        (from_x, from_y),
        destination,
        &moving_piece,
        &player,
        &pieces,
        promotion,
    )?;

    let updated_pieces = active_pieces(&game.board)?;
    let opponent_color = opposite_color(sender_color);
    let opponent = Player(db_color_to_piece_color(opponent_color)?);
    let mut check_pieces = updated_pieces.clone();

    if opponent.is_check_mate(&mut check_pieces) {
        game.game_status = Finished;
        game.winner = Some(sender_color);
    } else {
        game.turn = opponent_color;
    }

    ctx.db.game().id().update(game);
    Ok(())
}

fn player_color_in_game(
    game: &crate::game_table::Game,
    identity: spacetimedb::Identity,
) -> Result<u8, String> {
    if game.white == identity {
        Ok(DB_COLOR_WHITE)
    } else if game.black == Some(identity) {
        Ok(DB_COLOR_BLACK)
    } else {
        Err("You are not a player in this game".to_string())
    }
}

fn opposite_color(color: u8) -> u8 {
    if color == DB_COLOR_WHITE {
        DB_COLOR_BLACK
    } else {
        DB_COLOR_WHITE
    }
}

fn active_pieces(board: &[DbPiece]) -> Result<Vec<Piece>, String> {
    board
        .iter()
        .filter(|p| !p.taken)
        .map(|p| Piece::try_from(*p))
        .collect()
}

fn apply_move_to_board(
    board: &mut Vec<DbPiece>,
    from: (i8, i8),
    to: (i8, i8),
    moving_piece: &Piece,
    player: &Player,
    pieces_before: &[Piece],
    promotion: Option<u8>,
) -> Result<(), String> {
    for db in board.iter_mut() {
        if !db.taken && db.x == to.0 && db.y == to.1 {
            db.taken = true;
        }
    }

    let mover_color = piece_color_to_db(moving_piece.color);
    let mover_idx = board
        .iter()
        .position(|p| !p.taken && p.x == from.0 && p.y == from.1 && p.color == mover_color)
        .ok_or_else(|| format!("Piece missing at ({}, {})", from.0, from.1))?;

    board[mover_idx].x = to.0;
    board[mover_idx].y = to.1;

    if moving_piece.piece_type == PieceType::Pawn {
        let promotes = (moving_piece.color == chess_core::PieceColor::White && to.0 == 7)
            || (moving_piece.color == chess_core::PieceColor::Black && to.0 == 0);
        if promotes {
            board[mover_idx].piece_type = promotion.unwrap_or(DB_PIECE_QUEEN);
        }
    }

    if moving_piece.castling_rule(to, player, &pieces_before.to_vec()) {
        if let Some(rook) = moving_piece.find_castle_in_castling_move(to, &pieces_before.to_vec())
        {
            let rook_pos = moving_piece.find_pos_for_castle(to);
            for db in board.iter_mut() {
                if !db.taken
                    && db.x == rook.x
                    && db.y == rook.y
                    && db.color == piece_color_to_db(rook.color)
                    && db.piece_type == piece_type_to_db(PieceType::Rook)
                {
                    db.x = rook_pos.0;
                    db.y = rook_pos.1;
                    break;
                }
            }
        }
    }

    Ok(())
}
