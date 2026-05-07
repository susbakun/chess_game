use bevy::prelude::*;

use crate::pieces::*;

mod components;
mod plugins;
mod resources;
mod messages;
mod event_handlers;
mod utils;


pub use components::*;
pub use plugins::*;
pub use resources::*;
pub use messages::*;
pub use event_handlers::*;
pub use utils::*;

fn create_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    materials: Res<SquareMaterials>
) {
    let mesh = meshes.add(Plane3d::new(
        Vec3::new(0.0, 1.0, 0.0), 
        Vec2::new(0.5, 0.5)));

    for i in 0..8 {
        for j in 0..8 {
            let is_white = (i + j + 1) % 2 == 0;
            let material = if is_white {
                materials.white_color.clone()
            } else {
                materials.black_color.clone()
            };

            commands
                .spawn((
                    Mesh3d(mesh.clone()),
                    MeshMaterial3d(material),
                    Transform::from_translation(Vec3::new(i as f32, 0.0, j as f32)),
                    Pickable::default(),
                    Square { x: i, y: j },
                ))
                .observe(on_square_hover)
                .observe(on_square_hover_end)
                .observe(on_sqaure_click);
        }
    }
}

fn select_piece(
    selected_square: Res<SelectedSquare>,
    mut selected_piece: ResMut<SelectedPiece>,
    squares_query: Query<&Square>,
    piece_query: Query<(Entity, &Piece)>,
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

    if selected_piece.entity.is_none() {
        for (piece_entity, piece) in piece_query.iter() {
            if piece.x == square.x && piece.y == square.y {
                selected_piece.entity = Some(piece_entity);
                break;
            }
        }
    }
    
}

fn move_piece(
    mut commands: Commands,
    selected_square: Res<SelectedSquare>,
    selected_piece: Res<SelectedPiece>,
    mut turn: ResMut<PlayerTurn>,
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

    if let Some(selected_piece) = 
        selected_piece.entity {
            let pieces_vec: Vec<Piece> = piece_query
                .iter_mut()
                .map(|(_, piece)| *piece)
                .collect();

            let pieces_entity_vec: Vec<(Entity, Piece)> = piece_query
                .iter_mut()
                .map(|(entity, piece)| (entity, *piece))
                .collect();

            let mut piece = if let Ok((_, piece)) = 
                piece_query.get_mut(selected_piece) {
                    piece
            } else {
                    return;
            };

            if piece.is_move_valid((square.x, square.y), &pieces_vec) 
                && piece.color == turn.0
            {
                for (other_entity, other_piece) in pieces_entity_vec {
                    if other_piece.x == square.x
                        && other_piece.y == square.y 
                        && other_piece.color != piece.color
                        {
                            commands.entity(other_entity).insert(Taken);
                        }
                    }

                piece.x = square.x;
                piece.y = square.y;

                turn.change();
            }
            reset_selected_event.write(ResetSelectedEvent);
    }
}

fn reset_selected(
    mut message_reader: MessageReader<ResetSelectedEvent>,
    mut selected_square: ResMut<SelectedSquare>,
    mut selected_piece: ResMut<SelectedPiece>
) {
    for _message in message_reader.read() {
        selected_piece.entity = None;
        selected_square.entity = None;
    }
}
