use crate::GameOver;

use super::*;

pub fn move_piece(
    mut commands: Commands,
    selected_square: Res<SelectedSquare>,
    selected_piece: Res<SelectedPiece>,
    mut player: ResMut<Player>,
    mut winner: ResMut<Winner>,
    mut game_over: ResMut<GameOver>,
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

                // we store the current color before changing it
                // so we would have access to the player who might
                // have won the game
                let winner_player = player.0; 

                player.change();

                if player.is_check_mate(&mut updated_pieces_vec) {
                    winner.set(winner_player);
                    game_over.toggle();
                }
                reset_selected_event.write(ResetSelectedEvent);
            }
    }
}