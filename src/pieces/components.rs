use super::*;

#[derive(Component, Clone, Copy)]
pub struct Piece {
    pub color: PieceColor,
    pub piece_type: PieceType,
    // current position
    pub x: i8,
    pub y: i8
}


impl Piece {
    pub fn is_move_valid(&self, new_pos: (i8, i8), pieces: Vec<Piece>) -> bool {
        // If there's a piece of the same color in the same square, it can't move
        if color_of_square(new_pos, &pieces) == Some(self.color) {
            return false
        }

        match self.piece_type {
            PieceType::King => {
                // Horizontal
                ((self.x - new_pos.0).abs() == 1
                    && (self.y == new_pos.1))
                // Vertical
                ||  ((self.y - new_pos.1).abs() == 1
                    && (self.x == new_pos.0))
                // Diagonal
                ||  ((self.x - new_pos.0).abs() == 1
                    && (self.y - new_pos.1).abs() == 1)
            }
            PieceType::Queen => {
                is_path_empty((self.x, self.y), new_pos, &pieces)
                    && ((self.x - new_pos.0).abs()
                        == (self.y - new_pos.1).abs()
                        ||  ((self.x == new_pos.0 && self.y != new_pos.1)
                            ||  (self.y == new_pos.1 && self.x != new_pos.0)))
            },
            PieceType::Bishop => {
                is_path_empty((self.x, self.y), new_pos, &pieces)
                    && (self.x - new_pos.0).abs()
                        == (self.y - new_pos.1).abs()
            },
            PieceType::Knight => {
                ((self.x - new_pos.0).abs() == 1 
                    && (self.y - new_pos.1).abs() == 2)
                ||  ((self.x - new_pos.0).abs() == 2
                    && (self.y - new_pos.1).abs() == 1)
            },
            PieceType::Rook => {
                is_path_empty((self.x, self.y), new_pos, &pieces)
                    && ((self.x == new_pos.0 && self.y != new_pos.1)
                        || (self.y == new_pos.1 && self.x != new_pos.0))
            },
            PieceType::Pawn => {
                if self.color == PieceColor::White {
                    if new_pos.0 - self.x == 1 && (self.y == new_pos.1) {
                        if color_of_square(new_pos, &pieces).is_none() {
                            return true
                        }
                    }
    
                    if self.x == 1
                        && new_pos.0 - self.x == 2
                        && (self.y == new_pos.1)
                        && is_path_empty((self.x, self.y), new_pos, &pieces) {
                            if color_of_square(new_pos, &pieces).is_none() {
                                return true
                            }
                    }
    
                    if new_pos.0 - self.x == 1
                        && (self.y - new_pos.1).abs() == 1 {
                            if color_of_square(new_pos, &pieces) == Some(PieceColor::Black) {
                                return true
                            }
                    }
                } else {
                    if new_pos.0 - self.x == -1 && (self.y == new_pos.1) {
                        if color_of_square(new_pos, &pieces).is_none() {
                            return true
                        }
                    }
    
                    if self.x == 6
                        && new_pos.0 - self.x == -2
                        && (self.y == new_pos.1)
                        && is_path_empty((self.x, self.y), new_pos, &pieces) {
                            if color_of_square(new_pos, &pieces).is_none() {
                                return true
                            }
                    }
    
                    if new_pos.0 - self.x == -1
                        && (self.y - new_pos.1).abs() == 1 {
                            if color_of_square(new_pos, &pieces) == Some(PieceColor::White) {
                                return true
                            }
                    }
                }

                false
            }
        }
    }
}

