/// Wire encoding for [`PieceColor`].
pub const DB_COLOR_WHITE: u8 = 0;
pub const DB_COLOR_BLACK: u8 = 1;

/// Wire encoding for [`PieceType`] (matches variant order in `chess_core`).
pub const DB_PIECE_KING: u8 = 0;
pub const DB_PIECE_QUEEN: u8 = 1;
pub const DB_PIECE_BISHOP: u8 = 2;
pub const DB_PIECE_KNIGHT: u8 = 3;
pub const DB_PIECE_ROOK: u8 = 4;
pub const DB_PIECE_PAWN: u8 = 5;
