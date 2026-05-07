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

    fn is_check(&self, pieces: &Vec<Piece>) -> bool {
        let enemy_pieces = pieces
            .iter()
            .filter(|piece| piece.color != self.color);

        let king = pieces
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
                        && is_path_empty((piece.x, piece.y), (king.x, king.y), pieces) {
                            return true
                        }
                }
                PieceType::Bishop => {
                    if ((king.x - piece.x).abs() 
                        == (king.y - piece.y).abs()) 
                        &&  is_path_empty((piece.x, piece.y), (king.x, king.y), &pieces) {
                            return true
                        }
                }
                PieceType::Queen => {
                    if ((king.x - piece.x).abs() 
                        == (king.y - piece.y).abs()) 
                        && is_path_empty((piece.x, piece.y), (king.x, king.y), &pieces)
                        || (king.x == piece.x || king.y == piece.y) {
                            return true
                        }
                }
                PieceType::King => {
                    if (piece.x - king.x).abs() == 1
                        || (piece.y - king.y).abs() == 1 {
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



#[cfg(test)]
mod tests {
    use super::*;

    fn create_piece(piece_type: PieceType, color: PieceColor, x: i8, y: i8) -> Piece {
        Piece {
            piece_type,
            color,
            x,
            y,
        }
    }

    #[test]
    fn test_check_by_pawn_white() {
        let white_king = create_piece(PieceType::King, PieceColor::White, 3, 5);
        let black_pawn = create_piece(PieceType::Pawn, PieceColor::Black, 4, 4);
        
        let pieces = vec![white_king.clone(), black_pawn];
        
        assert!(white_king.is_check(&pieces));
    }

    #[test]
    fn test_check_by_pawn_black() {
        let black_king = create_piece(PieceType::King, PieceColor::Black, 5, 3);
        let white_pawn = create_piece(PieceType::Pawn, PieceColor::White, 4, 4);
        
        let pieces = vec![black_king.clone(), white_pawn];
        
        assert!(black_king.is_check(&pieces));
    }

    #[test]
    fn test_check_by_rook_horizontal() {
        let white_king = create_piece(PieceType::King, PieceColor::White, 4, 4);
        let black_rook = create_piece(PieceType::Rook, PieceColor::Black, 0, 4);
        
        let pieces = vec![white_king.clone(), black_rook];
        
        assert!(white_king.is_check(&pieces));
    }

    #[test]
    fn test_check_by_rook_vertical() {
        let white_king = create_piece(PieceType::King, PieceColor::White, 4, 4);
        let black_rook = create_piece(PieceType::Rook, PieceColor::Black, 4, 7);
        
        let pieces = vec![white_king.clone(), black_rook];
        
        assert!(white_king.is_check(&pieces));
    }

    #[test]
    fn test_rook_blocked_by_piece() {
        let white_king = create_piece(PieceType::King, PieceColor::White, 4, 4);
        let black_rook = create_piece(PieceType::Rook, PieceColor::Black, 0, 4);
        let blocking_piece = create_piece(PieceType::Bishop, PieceColor::Black, 2, 4);
        
        let pieces = vec![white_king.clone(), black_rook, blocking_piece];
        
        assert!(!white_king.is_check(&pieces));
    }

    #[test]
    fn test_check_by_bishop() {
        let white_king = create_piece(PieceType::King, PieceColor::White, 4, 4);
        let black_bishop = create_piece(PieceType::Bishop, PieceColor::Black, 1, 1);
        
        let pieces = vec![white_king.clone(), black_bishop];
        
        assert!(white_king.is_check(&pieces));
    }

    #[test]
    fn test_bishop_blocked_by_piece() {
        let white_king = create_piece(PieceType::King, PieceColor::White, 4, 4);
        let black_bishop = create_piece(PieceType::Bishop, PieceColor::Black, 1, 1);
        let blocking_piece = create_piece(PieceType::Pawn, PieceColor::Black, 2, 2);
        
        let pieces = vec![white_king.clone(), black_bishop, blocking_piece];
        
        assert!(!white_king.is_check(&pieces));
    }

    #[test]
    fn test_check_by_queen_horizontal() {
        let white_king = create_piece(PieceType::King, PieceColor::White, 4, 4);
        let black_queen = create_piece(PieceType::Queen, PieceColor::Black, 0, 4);
        
        let pieces = vec![white_king.clone(), black_queen];
        
        assert!(white_king.is_check(&pieces));
    }

    #[test]
    fn test_check_by_queen_diagonal() {
        let white_king = create_piece(PieceType::King, PieceColor::White, 4, 4);
        let black_queen = create_piece(PieceType::Queen, PieceColor::Black, 1, 1);
        
        let pieces = vec![white_king.clone(), black_queen];
        
        assert!(white_king.is_check(&pieces));
    }

    #[test]
    fn test_check_by_knight() {
        let white_king = create_piece(PieceType::King, PieceColor::White, 4, 4);
        let black_knight = create_piece(PieceType::Knight, PieceColor::Black, 5, 6);
        
        let pieces = vec![white_king.clone(), black_knight];
        
        assert!(white_king.is_check(&pieces));
    }

    #[test]
    fn test_knight_cannot_be_blocked() {
        let white_king = create_piece(PieceType::King, PieceColor::White, 4, 4);
        let black_knight = create_piece(PieceType::Knight, PieceColor::Black, 5, 6);
        let blocking_piece = create_piece(PieceType::Pawn, PieceColor::Black, 4, 5);
        
        let pieces = vec![white_king.clone(), black_knight, blocking_piece];
        
        assert!(white_king.is_check(&pieces));
    }

    #[test]
    fn test_check_by_adjacent_king() {
        let white_king = create_piece(PieceType::King, PieceColor::White, 4, 4);
        let black_king = create_piece(PieceType::King, PieceColor::Black, 5, 4);
        
        let pieces = vec![white_king.clone(), black_king];
        
        assert!(white_king.is_check(&pieces));
    }

    #[test]
    fn test_no_check_from_non_adjacent_king() {
        let white_king = create_piece(PieceType::King, PieceColor::White, 4, 4);
        let black_king = create_piece(PieceType::King, PieceColor::Black, 6, 4);
        
        let pieces = vec![white_king.clone(), black_king];
        
        assert!(!white_king.is_check(&pieces));
    }

    #[test]
    fn test_multiple_enemy_pieces_check() {
        let white_king = create_piece(PieceType::King, PieceColor::White, 4, 4);
        let black_rook = create_piece(PieceType::Rook, PieceColor::Black, 0, 0);
        let black_bishop = create_piece(PieceType::Bishop, PieceColor::Black, 1, 1);
        
        let pieces = vec![white_king.clone(), black_rook, black_bishop];
        
        assert!(white_king.is_check(&pieces));
    }
}
