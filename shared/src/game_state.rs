use serde::{Deserialize, Serialize};

use crate::game_event::GameEvent;
use crate::{constants::*, Piece, PieceType};
use crate::{PieceColor, Player};

#[derive(Serialize, Deserialize, Clone)]
pub struct GameState {
    pub pieces: Vec<Piece>,
    pub game_over: bool,
    pub winner: Option<PieceColor>,
    pub player: Player,
    pub timer: (u16, u16),
    pub waiting_for_opponent: bool,
}

fn initial_pieces() -> Vec<Piece> {
    let mut pieces = vec![
        // kings
        Piece {
            color: PieceColor::White,
            piece_type: PieceType::King,
            x: INITIAL_WHITE_KING_POS.0,
            y: INITIAL_WHITE_KING_POS.1,
            taken: false,
        },
        Piece {
            color: PieceColor::Black,
            piece_type: PieceType::King,
            x: INITIAL_BLACK_KING_POS.0,
            y: INITIAL_BLACK_KING_POS.1,
            taken: false,
        },
        // queens
        Piece {
            color: PieceColor::White,
            piece_type: PieceType::Queen,
            x: 0,
            y: 3,
            taken: false,
        },
        Piece {
            color: PieceColor::Black,
            piece_type: PieceType::Queen,
            x: 7,
            y: 3,
            taken: false,
        },
        // rooks
        Piece {
            color: PieceColor::White,
            piece_type: PieceType::Rook,
            x: INITIAL_WHITE_ROOK_POS1.0,
            y: INITIAL_WHITE_ROOK_POS1.1,
            taken: false,
        },
        Piece {
            color: PieceColor::White,
            piece_type: PieceType::Rook,
            x: INITIAL_WHITE_ROOK_POS2.0,
            y: INITIAL_WHITE_ROOK_POS2.1,
            taken: false,
        },
        Piece {
            color: PieceColor::Black,
            piece_type: PieceType::Rook,
            x: INITIAL_BLACK_ROOK_POS1.0,
            y: INITIAL_BLACK_ROOK_POS1.1,
            taken: false,
        },
        Piece {
            color: PieceColor::Black,
            piece_type: PieceType::Rook,
            x: INITIAL_BLACK_ROOK_POS2.0,
            y: INITIAL_BLACK_ROOK_POS2.1,
            taken: false,
        },
        // bishops
        Piece {
            color: PieceColor::White,
            piece_type: PieceType::Bishop,
            x: 0,
            y: 2,
            taken: false,
        },
        Piece {
            color: PieceColor::White,
            piece_type: PieceType::Bishop,
            x: 0,
            y: 5,
            taken: false,
        },
        Piece {
            color: PieceColor::Black,
            piece_type: PieceType::Bishop,
            x: 7,
            y: 2,
            taken: false,
        },
        Piece {
            color: PieceColor::Black,
            piece_type: PieceType::Bishop,
            x: 7,
            y: 5,
            taken: false,
        },
        // knights
        Piece {
            color: PieceColor::White,
            piece_type: PieceType::Knight,
            x: 0,
            y: 1,
            taken: false,
        },
        Piece {
            color: PieceColor::White,
            piece_type: PieceType::Knight,
            x: 0,
            y: 6,
            taken: false,
        },
        Piece {
            color: PieceColor::Black,
            piece_type: PieceType::Knight,
            x: 7,
            y: 1,
            taken: false,
        },
        Piece {
            color: PieceColor::Black,
            piece_type: PieceType::Knight,
            x: 7,
            y: 6,
            taken: false,
        },
    ];

    // pawns — this is the loop that saves you from writing 16 near-identical lines by hand
    for y in 0..8 {
        pieces.push(Piece {
            color: PieceColor::White,
            piece_type: PieceType::Pawn,
            x: 1,
            y,
            taken: false,
        });
        pieces.push(Piece {
            color: PieceColor::Black,
            piece_type: PieceType::Pawn,
            x: 6,
            y,
            taken: false,
        });
    }

    pieces
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            pieces: initial_pieces(),
            game_over: false,
            winner: None,
            player: Player::default(),
            // 20 minutes each
            timer: (TIMER_DURATION_SECS, TIMER_DURATION_SECS),
            waiting_for_opponent: true,
        }
    }
}

impl GameState {
    pub fn tick_timer(&mut self, elapsed_secs_whole: u16) -> bool {
        // returns true if the tick caused a game-over, so the caller knows to broadcast
        if self.game_over || self.waiting_for_opponent {
            return false;
        }
        match self.player.0 {
            PieceColor::White => {
                if self.timer.0 == 0 {
                    self.game_over = true;
                    self.winner = Some(PieceColor::Black);
                    return true;
                }
                self.timer.0 = self.timer.0.saturating_sub(elapsed_secs_whole);
            }
            PieceColor::Black => {
                if self.timer.1 == 0 {
                    self.game_over = true;
                    self.winner = Some(PieceColor::White);
                    return true;
                }
                self.timer.1 = self.timer.1.saturating_sub(elapsed_secs_whole);
            }
        }
        false
    }

    fn active_pieces(&self) -> Vec<Piece> {
        self.pieces.iter().filter(|p| !p.taken).copied().collect()
    }

    pub fn validate(&self, event: &GameEvent) -> bool {
        match event {
            GameEvent::MovePiece { from, to } => {
                let active = self.active_pieces();
                let piece = active.iter().find(|p| (p.x, p.y) == *from);
                let Some(piece) = piece else {
                    return false;
                };
                if piece.color != self.player.0 {
                    return false;
                }
                piece.is_move_valid(*to, &self.player, &active)
            }
            _ => false,
        }
    }

    pub fn consume(&mut self, event: &GameEvent) {
        match event {
            GameEvent::PlayerJoined { .. } => {
                // nothing to do here
            }
            GameEvent::MovePiece { from, to } => {
                let enemy = self.pieces.iter_mut().find(|p| (p.x, p.y) == *to);
                if let Some(enemy) = enemy {
                    enemy.taken = true;
                }

                let active = self.active_pieces();

                let piece = self.pieces.iter_mut().find(|p| (p.x, p.y) == *from);
                let Some(piece) = piece else {
                    return;
                };

                let mut updated_pieces_vec: Vec<Piece> = piece.simulate_next_step(*to, &active);

                // special move for castling rule
                let rook_entity_to_move = if piece.castling_rule(*to, &self.player, &active) {
                    piece
                        .find_castle_in_castling_move(*to, &active)
                        .and_then(|rook| {
                            active
                                .iter()
                                .find(|p| {
                                    p.piece_type == rook.piece_type
                                        && (p.x, p.y) == (rook.x, rook.y)
                                        && p.color == rook.color
                                })
                                .map(|castle| (castle, piece.find_pos_for_castle(*to)))
                        })
                } else {
                    None
                };

                (piece.x, piece.y) = *to;

                // Now mutate the rook
                if let Some((rook_ref, pos)) = rook_entity_to_move {
                    let rook_ref_mut = self
                        .pieces
                        .iter_mut()
                        .find(|p| (p.x, p.y) == (rook_ref.x, rook_ref.y))
                        .expect("rook must exist");

                    (rook_ref_mut.x, rook_ref_mut.y) = (pos.0, pos.1);
                }

                // we store the current color before changing it
                // so we would have access to the player who might
                // have won the game
                let winner_player = self.player.0;

                self.change_turn();

                // retrieve the next player
                let player = &self.player;

                if player.is_check_mate(&mut updated_pieces_vec) {
                    self.set_winner(winner_player);
                    self.toggle_game_over();
                }
            }
            GameEvent::BeginGame => {
                self.waiting_for_opponent = false;
            }
            GameEvent::EndGame { winner } => {
                self.winner = *winner;
                self.game_over = true;
            }
            GameEvent::PlayerDisconnected { .. } => {
                self.waiting_for_opponent = true;
                self.game_over = true;
            }
        }
    }

    pub fn change_turn(&mut self) {
        self.player.0 = match self.player.0 {
            PieceColor::White => PieceColor::Black,
            PieceColor::Black => PieceColor::White,
        }
    }

    pub fn set_winner(&mut self, winner: PieceColor) {
        self.winner = Some(winner);
    }

    pub fn toggle_game_over(&mut self) {
        self.game_over = !self.game_over;
    }
}
