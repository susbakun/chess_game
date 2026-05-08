use super::*;

pub fn move_piece(
    mut commands: Commands,
    selected_square: Res<SelectedSquare>,
    selected_piece: Res<SelectedPiece>,
    mut player: ResMut<Player>,
    squares_query: Query<&Square>,
    mut piece_query: Query<(Entity, &mut Piece)>,
    mut reset_selected_event: MessageWriter<ResetSelectedEvent>
) {
    let square_entity = if let Some(entity) = 
        selected_square.entity {
            entity
    } else {
            return
    };

    let square = if let Ok(square) = 
        squares_query.get(square_entity) {
            square
    } else {
            return
    };

    let new_pos = (square.x, square.y);

    if let Some(selected_piece) = 
        selected_piece.entity {
            let mut pieces_vec: Vec<Piece> = piece_query
                .iter_mut()
                .map(|(_, piece)| *piece)
                .filter(|piece| !piece.taken)
                .collect();


            let pieces_entity_vec: Vec<(Entity, Piece)> = piece_query
                .iter_mut()
                .map(|(entity, piece)| (entity, *piece))
                .filter(|(_, piece)| !piece.taken)
                .collect();

            let mut piece = if let Ok((_, piece)) = piece_query
                .get_mut(selected_piece) {
                    piece
            } else {
                    return;
            };

            if piece.is_move_valid(new_pos, &player, &mut pieces_vec) 
                && piece.color == player.0
            {
                for (other_entity, other_piece) in pieces_entity_vec.iter() {
                    if other_piece.x == new_pos.0
                        && other_piece.y == new_pos.1
                        && other_piece.color != piece.color
                        {
                            commands.entity(*other_entity).insert(Taken);
                        }
                    }

                // Recreate pieces_vec AFTER moving the piece
                let mut updated_pieces_vec: Vec<Piece> = piece.simulate_next_step(
                    new_pos, 
                    &pieces_vec
                );

                // special move for castling rule
                let rook_entity_to_move = if piece
                    .castling_rule(new_pos, &player, &pieces_vec) {
                    piece.find_castle_in_castling_move(new_pos, &pieces_vec)
                        .and_then(|rook| {

                            pieces_entity_vec
                                .iter()
                                .find(|(_, p)| 
                                    p.piece_type == rook.piece_type 
                                    && (p.x, p.y) == (rook.x, rook.y)
                                    && p.color == rook.color
                                )
                                .map(|(entity, _)| 
                                    (*entity, piece.find_pos_for_castle(new_pos)))
                        })
                } else {
                    None
                };

                piece.x = new_pos.0;
                piece.y = new_pos.1;
            
                // Drop the mutable borrow of piece
                drop(piece);
            
                // Now mutate the rook
                if let Some((rook_entity, pos)) = rook_entity_to_move {
                    if let Ok((_, mut moving_rook)) = piece_query.get_mut(rook_entity) {
                        moving_rook.x = pos.0;
                        moving_rook.y = pos.1;
                    }
                }            

                player.change();

                if player.is_check_mate(&mut updated_pieces_vec) {
                    println!(
                        "{} won! Thanks for playing!",
                        match player.0 {
                            PieceColor::White => "Black",
                            PieceColor::Black => "White",
                        }
                    );
                    std::process::exit(0);
                }
            }
            reset_selected_event.write(ResetSelectedEvent);
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_white_kingside_castling() {
        // Setup: King at e1 (4,0), Rook at h1 (7,0)
        let pieces = vec![
            Piece {
                piece_type: PieceType::King,
                x: 4,
                y: 0,
                color: PieceColor::White,
                taken: false
            },
            Piece {
                piece_type: PieceType::Rook,
                x: 7,
                y: 0,
                color: PieceColor::White,
                taken: false
            },
        ];

        let king = &pieces[0];
        let new_pos = (6, 0); // King moves to g1

        // Test castling_rule
        let player = Player(PieceColor::White);
        assert!(
            king.castling_rule(new_pos, &player, &pieces),
            "Castling should be valid"
        );

        // Test find_castle_in_castling_move
        let rook = king.find_castle_in_castling_move(new_pos, &pieces);
        assert!(rook.is_some(), "Should find the rook");
        let rook = rook.unwrap();
        assert_eq!(rook.x, 7);
        assert_eq!(rook.y, 0);

        // Test find_pos_for_castle
        let rook_new_pos = king.find_pos_for_castle(new_pos);
        assert_eq!(rook_new_pos, (5, 0), "Rook should move to f1");
    }

    #[test]
    fn test_white_queenside_castling() {
        // Setup: King at e1 (4,0), Rook at a1 (0,0)
        let pieces = vec![
            Piece {
                piece_type: PieceType::King,
                x: 4,
                y: 0,
                color: PieceColor::White,
                taken: false
            },
            Piece {
                piece_type: PieceType::Rook,
                x: 0,
                y: 0,
                color: PieceColor::White,
                taken: false
            },
        ];

        let king = &pieces[0];
        let new_pos = (2, 0); // King moves to c1

        let player = Player(PieceColor::White);
        assert!(king.castling_rule(new_pos, &player, &pieces));

        let rook = king.find_castle_in_castling_move(new_pos, &pieces);
        assert!(rook.is_some());
        
        let rook_new_pos = king.find_pos_for_castle(new_pos);
        assert_eq!(rook_new_pos, (3, 0), "Rook should move to d1");
    }

    #[test]
    fn test_black_kingside_castling() {
        let pieces = vec![
            Piece {
                piece_type: PieceType::King,
                x: 4,
                y: 7,
                color: PieceColor::Black,
                taken: false
            },
            Piece {
                piece_type: PieceType::Rook,
                x: 7,
                y: 7,
                color: PieceColor::Black,
                taken: false
            },
        ];

        let king = &pieces[0];
        let new_pos = (6, 7);

        let player = Player(PieceColor::Black);
        assert!(king.castling_rule(new_pos, &player, &pieces));

        let rook_new_pos = king.find_pos_for_castle(new_pos);
        assert_eq!(rook_new_pos, (5, 7), "Rook should move to f8");
    }
}
