use crate::pieces::*;
use bevy::prelude::*;

#[derive(Resource)]
pub struct Player(pub PieceColor);


impl Player {
    pub fn change(&mut self) {
        self.0 = match self.0 {
            PieceColor::White => PieceColor::Black,
            PieceColor::Black => PieceColor::White
        }
    }


    pub fn is_check(&self, pieces: &Vec<Piece>) -> bool {
        let enemy_pieces = pieces
            .iter()
            .filter(|piece| piece.color != self.0);

        let king = pieces
            .iter()
            .find(|piece| piece.piece_type == PieceType::King 
                && piece.color == self.0)
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
                            (king.x, king.y), &pieces) {
                            return true
                        }
                }
                PieceType::Bishop => {
                    if ((king.x - piece.x).abs() 
                        == (king.y - piece.y).abs()) 
                        &&  is_path_empty(
                            (piece.x, piece.y), 
                            (king.x, king.y), &pieces) {
                            return true
                        }
                }
                PieceType::Queen => {
                    if (((king.x - piece.x).abs() 
                        == (king.y - piece.y).abs()) 
                        && is_path_empty(
                            (piece.x, piece.y), 
                            (king.x, king.y), &pieces))
                        || (king.x == piece.x || king.y == piece.y) 
                        && is_path_empty(
                            (piece.x, piece.y), 
                            (king.x, king.y), &pieces) {
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


    pub fn is_check_mate(&self, pieces: &mut Vec<Piece>) -> bool {
        if !self.is_check(pieces) {
            return false
        }

        let mut pieces_clone = pieces.clone();

        for piece in pieces {
            if piece.color != self.0{
                continue;
            }


            for x in 0..8 {
                for y in 0..8 {
                    let new_pos = (x, y);

                    if piece.x == x && piece.y == y {
                        continue;
                    }
                    
                    if piece.is_move_valid(new_pos, self, &mut pieces_clone) {
                        return false
                    }
    
                }
            }
        }

        true
    }
}


impl Default for Player {
    fn default() -> Self {
        Player(PieceColor::White)
    }
}