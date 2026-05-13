use bevy::color::Color;

// king
pub const INITIAL_BLACK_KING_POS: (i8, i8) = (7, 4);
pub const INITIAL_WHITE_KING_POS: (i8, i8) = (0, 4);
pub const WHITE_KING_POS_KING_SIDE_CASTLING: (i8, i8) = (0, 2);
pub const WHITE_KING_POS_QUEEN_SIDE_CASTLING: (i8, i8) = (0, 6);
pub const BLACK_KING_POS_KING_SIDE_CASTLING: (i8, i8) = (7, 6);
pub const BLACK_KING_POS_QUEEN_SIDE_CASTLING: (i8, i8) = (7, 2);


// rook
pub const INITIAL_WHITE_ROOK_POS1: (i8, i8) = (0, 0);
pub const INITIAL_BLACK_ROOK_POS1: (i8, i8) = (7, 0);
pub const INITIAL_BLACK_ROOK_POS2: (i8, i8) = (7, 7);
pub const INITIAL_WHITE_ROOK_POS2: (i8, i8) = (0, 7);
pub const WHITE_ROOK_POS_KING_SIDE_CASTLING: (i8, i8) = (0, 5);
pub const WHITE_ROOK_POS_QUEEN_SIDE_CASTLING: (i8, i8) = (0, 3);
pub const BLACK_ROOK_POS_KING_SIDE_CASTLING: (i8, i8) = (7, 5);
pub const BLACK_ROOK_POS_QUEEN_SIDE_CASTLING: (i8, i8) = (7, 3);



// colors
pub const NORMAL_END_MENU_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
pub const HOVERED_END_MENU_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
pub const PRESSED_END_MENU_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);
pub const NORMAL_START_MENU_BUTTON: Color = Color::linear_rgb(0.0, 0.1, 0.1);
pub const HOVERED_START_MENU_BUTTON: Color = Color::linear_rgb(0.25, 0.6, 0.9);
pub const PRESSED_START_MENU_BUTTON: Color = Color::linear_rgb(0.15, 0.4, 0.7);