use renet::DefaultChannel;

#[cfg(not(target_arch = "wasm32"))]
use crate::game_state::GameType;
use crate::network::ClientNetwork;
use crate::{game_state::ClientGameState, network::MyColor};

use super::*;

pub fn send_move_system(
    selected_square: Res<SelectedSquare>,
    selected_piece: Res<SelectedPiece>,
    squares_query: Query<&Square>,
    piece_query: Query<&PieceComponent>,
    mut net: ResMut<ClientNetwork>,
    mut reset_selected_event: MessageWriter<ResetSelectedEvent>,
    my_color: Option<Res<MyColor>>,
) {
    let Some(selected_entity) = selected_piece.entity else {
        return;
    };
    let Some(square_entity) = selected_square.entity else {
        return;
    };
    let Ok(square) = squares_query.get(square_entity) else {
        return;
    };
    let Ok(piece) = piece_query.get(selected_entity) else {
        return;
    };

    let Some(my_color) = my_color else {
        return;
    }; // no color assigned yet, can't move
    if piece.color != my_color.0 {
        reset_selected_event.write(ResetSelectedEvent);
        return;
    }

    let from = (piece.x, piece.y);
    let to = (square.x, square.y);
    if from == to {
        return;
    }

    let event = GameEvent::MovePiece { from, to };
    let message = ClientMessage::SendEvent(event);
    let bytes = bincode::serialize(&message).expect("failed to serialize client message");
    net.client
        .send_message(DefaultChannel::ReliableOrdered, bytes);

    reset_selected_event.write(ResetSelectedEvent);
}

pub fn process_move_system(
    mut commands: Commands,
    selected_square: Res<SelectedSquare>,
    selected_piece: Res<SelectedPiece>,
    mut game_state: ResMut<ClientGameState>,
    squares_query: Query<&Square>,
    mut piece_query: Query<(Entity, &mut PieceComponent)>,
    mut reset_selected_event: MessageWriter<ResetSelectedEvent>,
) {
    let mut pieces_vec: Vec<Piece> = piece_query
        .iter()
        .map(|(_, piece)| *piece)
        .filter(|piece| !piece.taken)
        .map(|piece| piece.0)
        .collect();

    let pieces_entity_vec: Vec<(Entity, Piece)> = piece_query
        .iter()
        .map(|(entity, piece)| (entity, *piece))
        .filter(|(_, piece)| !piece.taken)
        .map(|(entity, piece)| (entity, piece.0))
        .collect();

    if let Some(selected_piece) = selected_piece.entity {
        let square_entity = if let Some(entity) = selected_square.entity {
            entity
        } else {
            return;
        };

        let square = if let Ok(square) = squares_query.get(square_entity) {
            square
        } else {
            return;
        };

        let new_pos = (square.x, square.y);
        let player = game_state.player.clone();

        #[cfg(not(target_arch = "wasm32"))]
        if game_state.engine.is_some() && player.0 == PieceColor::Black {
            return;
        }

        move_piece(
            &mut commands,
            Some(selected_piece),
            new_pos,
            &player,
            &mut pieces_vec,
            &pieces_entity_vec,
            &mut piece_query,
            &mut game_state,
            &mut reset_selected_event,
        );
    } else if let Some(game_type) = &game_state.game_type {
        let player = game_state.player.clone();

        #[cfg(not(target_arch = "wasm32"))]
        if *game_type == GameType::PlayWithAi && game_state.player.0 == PieceColor::Black {
            let fen = convert_to_fen(&pieces_vec, player.0);
            if let Some(engine) = &mut game_state.engine {
                engine.set_position(&fen);
                if let Some(best_move) = engine.get_best_move() {
                    if let Some((current_pos, new_pos)) = fen_to_piece_pos(best_move.clone()) {
                        let selected_entity = pieces_entity_vec
                            .iter()
                            .find(|(_, p)| (p.x, p.y) == current_pos)
                            .map(|(e, _)| *e);

                        move_piece(
                            &mut commands,
                            selected_entity,
                            new_pos,
                            &player,
                            &mut pieces_vec,
                            &pieces_entity_vec,
                            &mut piece_query,
                            &mut game_state,
                            &mut reset_selected_event,
                        );
                    }
                }
            }
        }
    }
}

fn move_piece(
    commands: &mut Commands,
    selected_piece: Option<Entity>,
    new_pos: (i8, i8),
    player: &Player,
    pieces_vec: &mut Vec<Piece>,
    pieces_entity_vec: &Vec<(Entity, Piece)>,
    piece_query: &mut Query<(Entity, &mut PieceComponent)>,
    game_state: &mut ClientGameState,
    reset_selected_event: &mut MessageWriter<ResetSelectedEvent>,
) {
    let p: &mut Piece;
    if let Some(selected_piece) = selected_piece {
        let piece = if let Ok((_, piece)) = piece_query.get_mut(selected_piece) {
            piece
        } else {
            return;
        };
        p = piece.into_inner()
    } else {
        return;
    }

    if p.is_move_valid(new_pos, &player, pieces_vec) && p.color == player.0 {
        for (other_entity, other_piece) in pieces_entity_vec.iter() {
            if other_piece.x == new_pos.0
                && other_piece.y == new_pos.1
                && other_piece.color != p.color
            {
                commands.entity(*other_entity).insert(Taken);
            }
        }

        let mut updated_pieces_vec: Vec<Piece> = p.simulate_next_step(new_pos, &pieces_vec);

        let rook_entity_to_move = if p.castling_rule(new_pos, &player, &pieces_vec) {
            p.find_castle_in_castling_move(new_pos, &pieces_vec)
                .and_then(|rook| {
                    pieces_entity_vec
                        .iter()
                        .find(|(_, p)| {
                            p.piece_type == rook.piece_type
                                && (p.x, p.y) == (rook.x, rook.y)
                                && p.color == rook.color
                        })
                        .map(|(entity, _)| (*entity, p.find_pos_for_castle(new_pos)))
                })
        } else {
            None
        };

        p.x = new_pos.0;
        p.y = new_pos.1;

        let _ = p;

        if let Some((rook_entity, pos)) = rook_entity_to_move {
            if let Ok((_, mut moving_rook)) = piece_query.get_mut(rook_entity) {
                moving_rook.x = pos.0;
                moving_rook.y = pos.1;
            }
        }

        let winner_player = player.0;

        game_state.change_turn();

        let player = &game_state.player;

        if player.is_check_mate(&mut updated_pieces_vec) {
            game_state.set_winner(winner_player);
            game_state.toggle_game_over();
        }
        reset_selected_event.write(ResetSelectedEvent);
    } else if new_pos != (p.x, p.y) {
        reset_selected_event.write(ResetSelectedEvent);
    }
}

pub fn move_pieces(time: Res<Time>, mut query: Query<(&mut Transform, &PieceComponent)>) {
    for (mut transform, piece) in query.iter_mut() {
        let direction = vec3(piece.x as f32, 0.0, piece.y as f32) - transform.translation;

        if direction.length() > 0.1 {
            transform.translation +=
                direction.normalize() * time.delta_secs() * vec3(2.0, 2.0, 2.0);
        }
    }
}
