use bevy::prelude::*;

use crate::pieces::*;

pub struct SquarePlugin;
impl Plugin for SquarePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<PlayerTurn>()
            .init_resource::<SelectedSquare>()
            .init_resource::<SelectedPiece>()
            .add_message::<ResetSelectedEvent>()
            .add_systems(Startup, create_board)
            .add_systems(Update, move_piece
                .run_if(resource_changed::<SelectedSquare>))
            .add_systems(Update, select_piece
                .run_if(resource_changed::<SelectedSquare>))
            .add_systems(Update, despawn_taken_pieces)
            .add_systems(Update, reset_selected);

    }
}


#[derive(Component)]
pub struct Square {
    pub x: u8,
    pub y: u8
}

impl Square {
    fn is_white(&self) -> bool {
        (self.x + self.y + 1) % 2 == 0
    }
}

#[derive(Resource)]
pub struct PlayerTurn(pub PieceColor);

impl PlayerTurn {
    fn change(&mut self) {
        self.0 = match self.0 {
            PieceColor::White => PieceColor::Black,
            PieceColor::Black => PieceColor::White
        }
    }
}

impl Default for PlayerTurn {
    fn default() -> Self {
        PlayerTurn(PieceColor::White)
    }
}

#[derive(Default, Resource)]
pub struct SelectedSquare {
    entity: Option<Entity>
}

#[derive(Default, Resource)]
pub struct SelectedPiece {
    entity: Option<Entity>
}

pub fn create_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    let mesh = meshes.add(Plane3d::new(
        Vec3::new(0.0, 1.0, 0.0), 
        Vec2::new(0.5, 0.5)));

    for i in 0..8 {
        for j in 0..8 {
            let is_white = (i + j + 1) % 2 == 0;
            let color = if is_white {
                Color::linear_rgb(1.0, 0.9, 0.9)
            } else {
                Color::linear_rgb(0.0, 0.1, 0.1)
            };

            let material = materials.add(color);

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

fn on_square_hover(
    _over: On<Pointer<Over>>,
    selected_square: Res<SelectedSquare>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(&MeshMaterial3d<StandardMaterial>, &Square)>,
) {
    // Don't reset color if this square is selected
    if let Some(selected_square_entity) = 
        selected_square.entity {
        if selected_square_entity == _over.entity {
            return;
        }
    }

    if let Ok((material_handle, _square)) = 
        query.get(_over.entity) {
        if let Some(material) = 
            materials.get_mut(material_handle) {
                // highlight color
                material.base_color = Color::linear_rgb(
                    0.8, 
                    0.3, 
                    0.3
                );
        }
    }
}

fn on_square_hover_end(
    _out: On<Pointer<Out>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(&MeshMaterial3d<StandardMaterial>, &Square)>,
    selected_square: Res<SelectedSquare>
) {
    // Don't reset color if this square is selected
    if let Some(selected_square_entity) = 
        selected_square.entity {
        if selected_square_entity == _out.entity {
            return;
        }
    }

    if let Ok((material_handle, square)) = 
        query.get(_out.entity) {
        if let Some(material) = 
            materials.get_mut(material_handle) {
                material.base_color = if square.is_white() {
                    Color::linear_rgb(
                        1.0, 
                        0.9, 
                        0.9)
                } else {
                    Color::linear_rgb(
                        0.0, 
                        0.1, 
                        0.1)
                };
        }
    }
}

fn on_sqaure_click(
    _click: On<Pointer<Click>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut selected_square: ResMut<SelectedSquare>,
    mut selected_piece: ResMut<SelectedPiece>,
    squares_query: Query<(&MeshMaterial3d<StandardMaterial>, &Square)>
) {
    if _click.button != PointerButton::Primary {
        return
    }   

    if let Ok((material_handle, _square)) = 
        squares_query.get(_click.entity) {
            if let Some(prev_square_entity) = 
                selected_square.entity {
                if prev_square_entity != _click.entity {
                    if let Ok((prev_material, prev_square)) = 
                        squares_query.get(prev_square_entity) {
                        if let Some(material) = 
                            materials.get_mut(prev_material) {
                                material.base_color = if prev_square.is_white() {
                                    Color::linear_rgb(
                                        1.0, 
                                        0.9, 
                                        0.9)
                                } else {
                                    Color::linear_rgb(
                                        0.0, 
                                        0.1, 
                                        0.1)
                                };
                            }
                    }
                }
            }
        // Highlight clicked square
        if let Some(material) = 
            materials.get_mut(material_handle) {
                material.base_color = Color::linear_rgb(
                    0.9, 
                    0.1, 
                    0.1);
        }
        selected_square.entity = Some(_click.entity);

    } else {
        // Player clicked outside the board, deselect everything
        selected_piece.entity = None;
        selected_square.entity = None;
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

            if piece.is_move_valid((square.x, square.y), pieces_vec) {
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


#[derive(Message)]
struct ResetSelectedEvent;
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

#[derive(Component)]
struct Taken;
fn despawn_taken_pieces(
    mut commands: Commands,
    query: Query<(Entity, &Piece, &Taken)>
) {
    for (entity, piece, _taken) in query.iter() {
        if piece.piece_type == PieceType::King {
            println!(
                "{} won! Thanks for playing!",
                match piece.color {
                    PieceColor::White => "Black",
                    PieceColor::Black => "White",
                }
            );
            std::process::exit(0);
        }
        commands.entity(entity).despawn();
    }
}