use spacetimedb::SpacetimeType;

use crate::constants::*;
use chess_core::{Piece, PieceColor, PieceType};

#[derive(SpacetimeType, Clone, Copy, Debug, PartialEq, Eq)]
pub struct DbPiece {
    pub color: u8,
    pub piece_type: u8,
    pub x: i8,
    pub y: i8,
    pub taken: bool,
}

/// Decode a stored color tag into [`PieceColor`].
pub fn db_color_to_piece_color(color: u8) -> Result<PieceColor, String> {
    match color {
        DB_COLOR_WHITE => Ok(PieceColor::White),
        DB_COLOR_BLACK => Ok(PieceColor::Black),
        _ => Err(format!("invalid board piece color: {color}")),
    }
}

/// Decode a stored piece-type tag into [`PieceType`].
pub fn db_piece_type_to_piece_type(piece_type: u8) -> Result<PieceType, String> {
    match piece_type {
        DB_PIECE_KING => Ok(PieceType::King),
        DB_PIECE_QUEEN => Ok(PieceType::Queen),
        DB_PIECE_BISHOP => Ok(PieceType::Bishop),
        DB_PIECE_KNIGHT => Ok(PieceType::Knight),
        DB_PIECE_ROOK => Ok(PieceType::Rook),
        DB_PIECE_PAWN => Ok(PieceType::Pawn),
        _ => Err(format!("invalid board piece type: {piece_type}")),
    }
}

pub fn piece_color_to_db(color: PieceColor) -> u8 {
    match color {
        PieceColor::White => DB_COLOR_WHITE,
        PieceColor::Black => DB_COLOR_BLACK,
    }
}

pub fn piece_type_to_db(piece_type: PieceType) -> u8 {
    match piece_type {
        PieceType::King => DB_PIECE_KING,
        PieceType::Queen => DB_PIECE_QUEEN,
        PieceType::Bishop => DB_PIECE_BISHOP,
        PieceType::Knight => DB_PIECE_KNIGHT,
        PieceType::Rook => DB_PIECE_ROOK,
        PieceType::Pawn => DB_PIECE_PAWN,
    }
}

impl TryFrom<DbPiece> for Piece {
    type Error = String;

    fn try_from(value: DbPiece) -> Result<Self, Self::Error> {
        Ok(Self {
            color: db_color_to_piece_color(value.color)?,
            piece_type: db_piece_type_to_piece_type(value.piece_type)?,
            x: value.x,
            y: value.y,
            taken: value.taken,
        })
    }
}

impl From<Piece> for DbPiece {
    fn from(value: Piece) -> Self {
        Self {
            color: piece_color_to_db(value.color),
            piece_type: piece_type_to_db(value.piece_type),
            x: value.x,
            y: value.y,
            taken: value.taken,
        }
    }
}

fn db_piece(color: PieceColor, piece_type: PieceType, x: i8, y: i8) -> DbPiece {
    DbPiece::from(Piece {
        color,
        piece_type,
        x,
        y,
        taken: false,
    })
}

/// Standard starting position (matches the Bevy client's `create_pieces` layout).
pub fn initial_board() -> Vec<DbPiece> {
    use PieceColor::{Black, White};
    use PieceType::{Bishop, King, Knight, Pawn, Queen, Rook};

    let mut board = Vec::with_capacity(32);

    let white_back_rank = [Rook, Knight, Bishop, Queen, King, Bishop, Knight, Rook];
    for (y, piece_type) in white_back_rank.into_iter().enumerate() {
        board.push(db_piece(White, piece_type, 0, y as i8));
    }
    for y in 0..8 {
        board.push(db_piece(White, Pawn, 1, y));
    }

    let black_back_rank = [Rook, Knight, Bishop, Queen, King, Bishop, Knight, Rook];
    for (y, piece_type) in black_back_rank.into_iter().enumerate() {
        board.push(db_piece(Black, piece_type, 7, y as i8));
    }
    for y in 0..8 {
        board.push(db_piece(Black, Pawn, 6, y));
    }

    board
}
