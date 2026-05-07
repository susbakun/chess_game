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
    pub fn is_move_valid(&self, new_pos: (i8, i8), pieces: &Vec<Piece>) -> bool {
        // If there's a piece of the same color in the same square, it can't move
        if color_of_square(new_pos, &pieces) == Some(self.color) {
            return false
        }

        if self.is_check(new_pos, pieces) {
            return false
        }

        // rules
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

    pub fn is_check(&self, new_pos: (i8, i8), pieces: &Vec<Piece>) -> bool {
        // Create a simulated board after the move
        let mut pieces_after_move = pieces.clone();
        
        // Remove the piece from its current position
        pieces_after_move
            .retain(|p| 
                !(p.x == self.x && p.y == self.y 
                    && p.color == self.color && p.piece_type == self.piece_type));
        
        // Remove any captured piece at new_pos
        pieces_after_move
            .retain(|p| 
                !(p.x == new_pos.0 && p.y == new_pos.1));
        
        // Add the moving piece at its new position
        let mut moved_piece = *self;
        moved_piece.x = new_pos.0;
        moved_piece.y = new_pos.1;
        pieces_after_move.push(moved_piece);

        let enemy_pieces = pieces_after_move
            .iter()
            .filter(|piece| piece.color != self.color);

        let king = pieces_after_move
            .iter()
            .find(|piece| piece.piece_type == PieceType::King 
                && piece.color == self.color)
            .expect("the king wasn't found");

        for piece in enemy_pieces {
            match piece.piece_type {
                PieceType::Pawn => {
                    if piece.color == PieceColor::White {
                        if (piece.x + 1, piece.y + 1) == (king.x, king.y) 
                            || (piece.x + 1, piece.y - 1) == (king.x, king.y)
                        {
                            return true
                        }
                    } else {
                        if (piece.x - 1, piece.y + 1) == (king.x, king.y) 
                            || (piece.x - 1, piece.y - 1) == (king.x, king.y) 
                        {
                            return true
                        }
                    }
                },
                PieceType::Rook => {
                    if (king.x == piece.x || king.y == piece.y)
                        && is_path_empty(
                            (piece.x, piece.y), 
                            (king.x, king.y), &pieces_after_move) {
                            return true
                        }
                }
                PieceType::Bishop => {
                    if ((king.x - piece.x).abs() 
                        == (king.y - piece.y).abs()) 
                        &&  is_path_empty(
                            (piece.x, piece.y), 
                            (king.x, king.y), &pieces_after_move) {
                            return true
                        }
                }
                PieceType::Queen => {
                    if (((king.x - piece.x).abs() 
                        == (king.y - piece.y).abs()) 
                        && is_path_empty(
                            (piece.x, piece.y), 
                            (king.x, king.y), &pieces_after_move))
                        || (king.x == piece.x || king.y == piece.y) 
                        && is_path_empty(
                            (piece.x, piece.y), 
                            (king.x, king.y), &pieces_after_move) {
                            return true
                        }
                }
                PieceType::King => {
                    if ((king.x - piece.x).abs() == 1
                        && (king.y == piece.y))
                    ||  ((king.y - piece.y).abs() == 1
                        && (king.x == piece.x))
                    ||  ((king.x - piece.x).abs() == 1
                        && (king.y - piece.y).abs() == 1) {
                            return true
                        }
                }
                PieceType::Knight => {
                    if ((king.x - piece.x).abs() == 2
                        && (king.y - piece.y).abs() == 1)
                    ||  ((king.x - piece.x).abs() == 1
                        && (king.y - piece.y).abs() == 2) {
                            return true
                        }
                }
            };
        }


        false
    }
}