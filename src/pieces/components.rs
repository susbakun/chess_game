use super::*;

use crate::player::*;



#[derive(Component, Clone, Copy, Debug)]
pub struct Piece {
    pub color: PieceColor,
    pub piece_type: PieceType,
    // current position
    pub x: i8,
    pub y: i8,
    pub taken: bool
}


impl Piece {
    pub fn is_move_valid(&self, new_pos: (i8, i8), player: &Player, pieces: &Vec<Piece>) -> bool {
        // If there's a piece of the same color in the same square, it can't move
        if color_of_square(new_pos, &pieces) == Some(self.color) {
            return false
        }


        let pieces_after_move= self.
            simulate_next_step(new_pos, &pieces);

        if player.is_check(&pieces_after_move) {
            println!("Move from ({}, {}) to ({}, {}) leaves king in check", self.x, self.y, new_pos.0, new_pos.1);
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
                // checking castling rule
                ||  self.castling_rule(new_pos, &player, pieces)
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

    pub fn simulate_next_step(&self, new_pos: (i8, i8), pieces: &Vec<Piece>) -> Vec<Piece> {
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

        pieces_after_move
    }

    pub fn castling_rule(
        &self, 
        new_pos: (i8, i8), 
        player: &Player, 
        pieces: &Vec<Piece>
    ) -> bool {
        if player.is_check(&pieces) {
            return false
        }

        
        if !is_path_empty((self.x, self.y), new_pos, &pieces) {
            return false
        }
        
        if self.color == PieceColor::White {
            if (self.x, self.y) == 
                (INITIAL_WHITE_KING_POS.0, INITIAL_WHITE_KING_POS.1){
                    if new_pos == WHITE_KING_POS_QUEEN_SIDE_CASTLING
                    && self.piece_at(
                        INITIAL_WHITE_ROOK_POS1, 
                        &pieces).is_some() {
                            return true
                } else if new_pos == WHITE_KING_POS_KING_SIDE_CASTLING
                    && self.piece_at(
                        INITIAL_WHITE_ROOK_POS2, 
                        &pieces).is_some() {
                            return true
                }
            }
        } else {
            if (self.x, self.y) == 
                (INITIAL_BLACK_KING_POS.0, INITIAL_BLACK_KING_POS.1){
                    if new_pos == BLACK_KING_POS_QUEEN_SIDE_CASTLING
                        && self.piece_at(
                            INITIAL_BLACK_ROOK_POS1, 
                            &pieces).is_some() {
                                return true
                    } else if new_pos == BLACK_KING_POS_KING_SIDE_CASTLING
                        && self.piece_at(
                            INITIAL_BLACK_ROOK_POS2, 
                            &pieces).is_some() {
                                return true
                    }
            }
        }


        false
    }


    fn piece_at(
        &self, 
        pos: (i8, i8), 
        pieces: &Vec<Piece>
    ) -> Option<Piece> {

        let piece = pieces
            .iter()
            .find(|p| p.color == self.color 
                && (p.x, p.y) == pos)
            .map(|p| *p);

        piece
    }

    pub fn find_castle_in_castling_move(
        &self, 
        pos: (i8, i8), 
        pieces: &Vec<Piece>
    ) -> Option<Piece> {
        let rook_pos: (i8, i8);

        if self.color == PieceColor::White {
            rook_pos = if is_king_side_castling(pos) {
                INITIAL_WHITE_ROOK_POS2
            } else {
                INITIAL_WHITE_ROOK_POS1
            };
        } else {
            rook_pos = if is_king_side_castling(pos) {
                INITIAL_BLACK_ROOK_POS2
            } else {
                INITIAL_BLACK_ROOK_POS1
            };
        }

        match self.piece_at(rook_pos, pieces) {
            Some(piece) 
                if piece.piece_type == PieceType::Rook => 
                    Some(piece),
            _ => None
        }
    }

    pub fn find_pos_for_castle(&self, new_pos: (i8, i8)) -> (i8, i8) {
        let is_king_side = is_king_side_castling(new_pos);

        if self.color == PieceColor::White {
            if is_king_side {
                WHITE_ROOK_POS_KING_SIDE_CASTLING
            } else {
                WHITE_ROOK_POS_QUEEN_SIDE_CASTLING
            }
        } else {
            if is_king_side {
                BLACK_ROOK_POS_KING_SIDE_CASTLING
            } else {
                BLACK_ROOK_POS_QUEEN_SIDE_CASTLING
            }
        }
    }
}