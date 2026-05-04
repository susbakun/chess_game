use bevy::prelude::*;

use crate::pieces::*;

pub struct SquarePlugin;
impl Plugin for SquarePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<PlayerTurn>()
            .init_resource::<SelectedSquare>()
            .init_resource::<SelectedPiece>()
            .init_resource::<SquareMaterials>()
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

#[derive(Resource)]
struct SquareMaterials {
    highlight_color: Handle<StandardMaterial>,
    selected_color: Handle<StandardMaterial>,
    black_color: Handle<StandardMaterial>,
    white_color: Handle<StandardMaterial>,
}

impl FromWorld for SquareMaterials {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world.resource_mut::<Assets<StandardMaterial>>();

        Self {
            highlight_color: materials.add(
                Color::linear_rgb(0.8, 0.3, 0.3)
            ),
            selected_color: materials.add(
                Color::linear_rgb(0.9, 0.1, 0.1)
            ),
            black_color: materials.add(
                Color::linear_rgb(0., 0.1, 0.1)
            ),
            white_color: materials.add(
                Color::linear_rgb(1., 0.9, 0.9)
            ),
        }
    }
}

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

fn on_square_hover(
    _over: On<Pointer<Over>>,
    selected_square: Res<SelectedSquare>,
    materials: Res<SquareMaterials>,
    mut query: Query<(&mut MeshMaterial3d<StandardMaterial>, &Square)>,
) {
    // Don't reset color if this square is selected
    if let Some(selected_square_entity) = 
        selected_square.entity {
        if selected_square_entity == _over.entity {
            return;
        }
    }

    if let Ok((mut material_handle, _square)) = 
        query.get_mut(_over.entity) {
            material_handle.0 = materials.highlight_color.clone();
    }
}

fn on_square_hover_end(
    _out: On<Pointer<Out>>,
    materials: ResMut<SquareMaterials>,
    mut query: Query<(&mut MeshMaterial3d<StandardMaterial>, &Square)>,
    selected_square: Res<SelectedSquare>
) {
    // Don't reset color if this square is selected
    if let Some(selected_square_entity) = 
        selected_square.entity {
        if selected_square_entity == _out.entity {
            return;
        }
    }

    if let Ok((mut material_handle, square)) = 
        query.get_mut(_out.entity) {
        material_handle.0 = if square.is_white() {
            materials.white_color.clone()
        } else {
            materials.black_color.clone()
        };
    }
}

fn on_sqaure_click(
    _click: On<Pointer<Click>>,
    materials: Res<SquareMaterials>,
    mut selected_square: ResMut<SelectedSquare>,
    mut selected_piece: ResMut<SelectedPiece>,
    mut squares_query: Query<(&mut MeshMaterial3d<StandardMaterial>, &Square)>
) {
    if _click.button != PointerButton::Primary {
        return
    }   

    // reset the previous selected square
    if let Some(prev_square_entity) = 
        selected_square.entity {
        if prev_square_entity != _click.entity {
            if let Ok((mut prev_material, prev_square)) = 
                squares_query.get_mut(prev_square_entity) {
                prev_material.0 = if prev_square.is_white() {
                    materials.white_color.clone()
                } else {
                    materials.black_color.clone()
                };
            }
        }
    }

    if let Ok((mut material_handle, _square)) = 
        squares_query.get_mut(_click.entity) {
        // Highlight clicked square
        material_handle.0 = materials.selected_color.clone();
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