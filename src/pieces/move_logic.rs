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
            let pieces_vec: Vec<Piece> = piece_query
                .iter_mut()
                .map(|(_, piece)| *piece)
                .filter(|piece| !piece.taken)
                .collect();


            let pieces_entity_vec: Vec<(Entity, Piece)> = piece_query
                .iter_mut()
                .map(|(entity, piece)| (entity, *piece))
                .filter(|(_, piece)| !piece.taken)
                .collect();

            let mut piece = if let Ok((_, piece)) = 
                piece_query.get_mut(selected_piece) {
                    piece
            } else {
                    return;
            };

            if piece.is_move_valid(new_pos, &player, &pieces_vec) 
                && piece.color == player.0
            {
                for (other_entity, other_piece) in pieces_entity_vec {
                    if other_piece.x == new_pos.0
                        && other_piece.y == new_pos.1
                        && other_piece.color != piece.color
                        {
                            commands.entity(other_entity).insert(Taken);
                        }
                    }

                // Recreate pieces_vec AFTER moving the piece
                let updated_pieces_vec: Vec<Piece> = piece.simulate_next_step(
                    new_pos, 
                    &pieces_vec
                );

                piece.x = new_pos.0;
                piece.y = new_pos.1;

                player.change();

                if player.is_check_mate(&updated_pieces_vec) {
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
