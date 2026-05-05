use bevy::prelude::*;

pub struct PiecePlugin;
impl Plugin for PiecePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<PieceHandles>()
            .add_systems(Startup, create_pieces)
            .add_systems(Update, move_pieces);
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum PieceColor {
    White,
    Black
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PieceType {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn
}

#[derive(Component, Clone, Copy)]
pub struct Piece {
    pub color: PieceColor,
    pub piece_type: PieceType,
    // current position
    pub x: i8,
    pub y: i8
}


impl Piece {
    pub fn is_move_valid(&self, new_pos: (i8, i8), pieces: Vec<Piece>) -> bool {
        // If there's a piece of the same color in the same square, it can't move
        if color_of_square(new_pos, &pieces) == Some(self.color) {
            return false
        }

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
}


#[derive(Resource, Clone)]
pub struct PieceHandles {
    king_handle: Handle<Mesh>,
    king_cross_handle: Handle<Mesh>,
    queen_handle: Handle<Mesh>,
    knight_1_handle: Handle<Mesh>,
    knight_2_handle: Handle<Mesh>,
    bishop_handle: Handle<Mesh>,
    rook_handle: Handle<Mesh>,
    pawn_handle: Handle<Mesh>
}

impl FromWorld for PieceHandles {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();

        let king_handle: Handle<Mesh> = 
            asset_server.load("models/pieces.glb#Mesh0/Primitive0");
        let king_cross_handle: Handle<Mesh> = 
            asset_server.load("models/pieces.glb#Mesh1/Primitive0");
        let pawn_handle: Handle<Mesh> = 
            asset_server.load("models/pieces.glb#Mesh2/Primitive0");
        let knight_1_handle: Handle<Mesh> = 
            asset_server.load("models/pieces.glb#Mesh3/Primitive0");
        let knight_2_handle: Handle<Mesh> =
                asset_server.load("models/pieces.glb#Mesh4/Primitive0");
        let rook_handle: Handle<Mesh> =
                asset_server.load("models/pieces.glb#Mesh5/Primitive0");
        let bishop_handle: Handle<Mesh> =
                asset_server.load("models/pieces.glb#Mesh6/Primitive0");
        let queen_handle: Handle<Mesh> =
                asset_server.load("models/pieces.glb#Mesh7/Primitive0");

        Self {
            king_handle,
            king_cross_handle,
            queen_handle,
            knight_1_handle,
            knight_2_handle,
            rook_handle,
            bishop_handle,
            pawn_handle
        }
    }
}

fn spawn_king(
    mut commands: Commands,
    material: Handle<StandardMaterial>,
    mesh: Handle<Mesh>,
    mesh_cross: Handle<Mesh>,
    piece_color: PieceColor,
    position: (i8, i8)
) {
    commands
        .spawn((
            Transform::from_translation(
                vec3(position.0 as f32, 0.0, position.1 as f32)
            ),
            Piece {
                color: piece_color,
                piece_type: PieceType::King,
                x: position.0,
                y: position.1
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Mesh3d(mesh.clone()),
                MeshMaterial3d(material.clone()),
                Transform::from_translation(Vec3::new(-0.2, 0.0, -1.9))
                    .with_scale(Vec3::new(0.2, 0.2, 0.2)),
                Pickable::IGNORE
            ));

            parent.spawn((
                Mesh3d(mesh_cross.clone()),
                MeshMaterial3d(material.clone()),
                Transform::from_translation(Vec3::new(-0.2, 0.0, -1.9))
                    .with_scale(Vec3::new(0.2, 0.2, 0.2)),
                Pickable::IGNORE
            ));
        });
}

fn spawn_knight(
    mut commands: Commands,
    material: Handle<StandardMaterial>,
    mesh_1: Handle<Mesh>,
    mesh_2: Handle<Mesh>,
    piece_color: PieceColor,
    position: (i8, i8),
    rotate: bool
) {
    commands
        .spawn((
            Transform::from_translation(
                vec3(position.0 as f32, 0.0, position.1 as f32),
            )
                .with_rotation(
                    if rotate {
                        Quat::from_xyzw(0.0, 1.0, 0.0, 0.0)
                            .normalize()
                    } else {
                        Quat::from_xyzw(0.0, 0.0, 0.0, 0.0)
                    }
                ),
                Piece {
                    color: piece_color,
                    piece_type: PieceType::Knight,
                    x: position.0,
                    y: position.1
                },
                Pickable::IGNORE
            ))
        .with_children(|parent| {
            parent.spawn((
                Mesh3d(mesh_1),
                MeshMaterial3d(material.clone()),
                Transform::from_translation(vec3(-0.2, 0.0, 0.9))
                    .with_scale(vec3(0.2, 0.2, 0.2)),
                Pickable::IGNORE
            ));
            parent.spawn((
                Mesh3d(mesh_2),
                MeshMaterial3d(material.clone()),
                Transform::from_translation(vec3(-0.2, 0.0, 0.9))
                    .with_scale(vec3(0.2, 0.2, 0.2)),
                Pickable::IGNORE
            ));
        });
}

fn spawn_queen(
    mut commands: Commands,
    material: Handle<StandardMaterial>,
    mesh: Handle<Mesh>,
    piece_color: PieceColor,
    position: (i8, i8)
) {
    commands
        .spawn((
            Transform::from_translation(
                vec3(position.0 as f32, 0.0, position.1 as f32)
            ),
            Piece {
                color: piece_color,
                piece_type: PieceType::Queen,
                x: position.0,
                y: position.1
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Mesh3d(mesh),
                MeshMaterial3d(material),
                Transform::from_translation(vec3(-0.2, 0.0, -0.95))
                    .with_scale(vec3(0.2, 0.2, 0.2)),
                Pickable::IGNORE
            ));
        });
}

fn spawn_bishop(
    mut commands: Commands,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    piece_color: PieceColor,
    position: (i8, i8)
) {
    commands
        .spawn((
            Transform::from_translation(
                vec3(position.0 as f32, 0.0, position.1 as f32)
            ),
            Piece {
                color: piece_color,
                piece_type: PieceType::Bishop,
                x: position.0,
                y: position.1
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Mesh3d(mesh),
                MeshMaterial3d(material),
                Transform::from_translation(
                    vec3(-0.1, 0.0, 0.0)
                ).with_scale(vec3(0.2, 0.2, 0.2)),
                Pickable::IGNORE
            ));
        });
}

fn spawn_rook(
    mut commands: Commands,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    piece_color: PieceColor,
    position: (i8, i8)
) {
    commands
        .spawn((
            Transform::from_translation(
                vec3(position.0 as f32, 0.0, position.1 as f32)
            ),
            Piece {
                color: piece_color,
                piece_type: PieceType::Rook,
                x: position.0,
                y: position.1
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Mesh3d(mesh),
                MeshMaterial3d(material),
                Transform::from_translation(
                    vec3(-0.1, 0.0, 1.9)
                ).with_scale(vec3(0.2, 0.2, 0.2)),
                Pickable::IGNORE
            ));
        });
}

fn spawn_pawn(
    mut commands: Commands,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    piece_color: PieceColor,
    position: (i8, i8)
) {
    commands
        .spawn((
            Transform::from_translation(
                vec3(position.0 as f32, 0.0, position.1 as f32)
            ),
            Piece {
                color: piece_color,
                piece_type: PieceType::Pawn,
                x: position.0,
                y: position.1
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Mesh3d(mesh),
                MeshMaterial3d(material),
                Transform::from_translation(
                    vec3(-0.2, 0.0, 2.6)
                ).with_scale(vec3(0.2, 0.2, 0.2)),
                Pickable::IGNORE
            ));
        });
}


pub fn spawn_piece(
    mut commands: Commands,
    material: Handle<StandardMaterial>,
    piece: Piece,
    piece_handles: PieceHandles
) {
    let position = (piece.x, piece.y);

    match piece.piece_type {
        PieceType::King => {
            spawn_king(
                commands.reborrow(), 
                material,
                piece_handles.king_handle.clone(),
                piece_handles.king_cross_handle.clone(), 
                piece.color, 
                position
            );
        }
        PieceType::Queen => {
            spawn_queen(
                commands.reborrow(), 
                material, 
                piece_handles.queen_handle.clone(), 
                piece.color, 
                position
            );
        }
        PieceType::Knight => {
            let rotate = if piece.color == PieceColor::White {
                false
            } else {
                true
            };

            spawn_knight(
                commands.reborrow(), 
                material, 
                piece_handles.knight_1_handle.clone(), 
                piece_handles.knight_2_handle.clone(), 
                piece.color, 
                position,
                rotate
            );
        }
        PieceType::Bishop => {
            spawn_bishop(
                commands.reborrow(), 
                piece_handles.bishop_handle.clone(), 
                material, 
                piece.color, 
                position
            );
        }
        PieceType::Rook => {
            spawn_rook(
                commands.reborrow(), 
                piece_handles.rook_handle.clone(), 
                material, 
                piece.color, 
                position
            );
        }
        PieceType::Pawn => {
            spawn_pawn(
                commands.reborrow(), 
                piece_handles.pawn_handle.clone(), 
                material, 
                piece.color, 
                position
            );
        }
    }
}

pub fn create_pieces(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    piece_handles: Res<PieceHandles>
) {
    let white_material = materials.add(
        Color::linear_rgb(1.0, 0.9, 0.9));

    let black_material = materials.add(
        Color::linear_rgb(0.0, 0.1, 0.1)
    );

    // white pieces
    spawn_piece(
        commands.reborrow(), 
        white_material.clone(), 
        Piece { 
            color: PieceColor::White, 
            piece_type: PieceType::Rook, 
            x: 0, 
            y: 0 
        },
        piece_handles.clone()
    );

    spawn_piece(
        commands.reborrow(), 
        white_material.clone(),
        Piece { 
            color: PieceColor::White, 
            piece_type: PieceType::Knight, 
            x: 0, 
            y: 1 
        },
        piece_handles.clone()
    );

    spawn_piece(
        commands.reborrow(), 
        white_material.clone(),
        Piece { 
            color: PieceColor::White, 
            piece_type: PieceType::Bishop, 
            x: 0, 
            y: 2 
        },
        piece_handles.clone()
    );

    spawn_piece(
        commands.reborrow(), 
        white_material.clone(),
        Piece { 
            color: PieceColor::White, 
            piece_type: PieceType::Queen, 
            x: 0, 
            y: 3 
        },
        piece_handles.clone()
    );

    spawn_piece(
        commands.reborrow(), 
        white_material.clone(),
        Piece { 
            color: PieceColor::White, 
            piece_type: PieceType::King, 
            x: 0, 
            y: 4
        },
        piece_handles.clone()
    );

    spawn_piece(
        commands.reborrow(), 
        white_material.clone(),
        Piece { 
            color: PieceColor::White, 
            piece_type: PieceType::Bishop, 
            x: 0, 
            y: 5
        },
        piece_handles.clone()
    );

    spawn_piece(
        commands.reborrow(), 
        white_material.clone(),
        Piece { 
            color: PieceColor::White, 
            piece_type: PieceType::Knight, 
            x: 0, 
            y: 6
        },
        piece_handles.clone()
    );

    spawn_piece(
        commands.reborrow(), 
        white_material.clone(), 
        Piece { 
            color: PieceColor::White, 
            piece_type: PieceType::Rook, 
            x: 0, 
            y: 7
        },
        piece_handles.clone()
    );

    for i in 0..8 {
        spawn_piece(
            commands.reborrow(), 
            white_material.clone(),
            Piece { 
                color: PieceColor::White, 
                piece_type: PieceType::Pawn,
                x: 1, 
                y: i 
            },
            piece_handles.clone()
        );
    }


    // black pieces
    spawn_piece(
        commands.reborrow(), 
        black_material.clone(), 
        Piece { 
            color: PieceColor::Black, 
            piece_type: PieceType::Rook, 
            x: 7,
            y: 0 
        },
        piece_handles.clone()
    );

    spawn_piece(
        commands.reborrow(), 
        black_material.clone(),
        Piece { 
            color: PieceColor::Black, 
            piece_type: PieceType::Knight, 
            x: 7, 
            y: 1 
        },
        piece_handles.clone()
    );

    spawn_piece(
        commands.reborrow(), 
        black_material.clone(),
        Piece { 
            color: PieceColor::Black, 
            piece_type: PieceType::Bishop, 
            x: 7, 
            y: 2 
        },
        piece_handles.clone()
    );

    spawn_piece(
        commands.reborrow(), 
        black_material.clone(),
        Piece { 
            color: PieceColor::Black, 
            piece_type: PieceType::Queen, 
            x: 7, 
            y: 3 
        },
        piece_handles.clone()
    );

    spawn_piece(
        commands.reborrow(), 
        black_material.clone(),
        Piece { 
            color: PieceColor::Black, 
            piece_type: PieceType::King, 
            x: 7, 
            y: 4
        },
        piece_handles.clone()
    );

    spawn_piece(
        commands.reborrow(), 
        black_material.clone(),
        Piece { 
            color: PieceColor::Black, 
            piece_type: PieceType::Bishop, 
            x: 7, 
            y: 5
        },
        piece_handles.clone()
    );

    spawn_piece(
        commands.reborrow(), 
        black_material.clone(),
        Piece { 
            color: PieceColor::Black, 
            piece_type: PieceType::Knight, 
            x: 7, 
            y: 6
        },
        piece_handles.clone()
    );

    spawn_piece(
        commands.reborrow(), 
        black_material.clone(), 
        Piece { 
            color: PieceColor::Black, 
            piece_type: PieceType::Rook, 
            x: 7, 
            y: 7
        },
        piece_handles.clone()
    );

    for i in 0..8 {
        spawn_piece(
            commands.reborrow(), 
            black_material.clone(),
            Piece { 
                color: PieceColor::Black, 
                piece_type: PieceType::Pawn,
                x: 6, 
                y: i 
            },
            piece_handles.clone()
        );
    }
}

fn move_pieces(time: Res<Time>, mut query: Query<(&mut Transform, &Piece)>) {
    for (mut transform, piece) in query.iter_mut() {
        let direction = vec3(piece.x as f32, 0.0, piece.y as f32) - transform.translation;

        // Only move if the piece isn't already there (distance is big)
        if direction.length() > 0.1 {
            transform.translation += 
                direction.normalize() * 
                time.delta_secs() * 
                vec3(2.0, 2.0, 2.0);
        }
    }
}

fn color_of_square(pos: (i8, i8), pieces: &Vec<Piece>) -> Option<PieceColor> {
    for piece in pieces {
        if piece.x == pos.0 && piece.y == pos.1 {
            return Some(piece.color)
        }
    }

    None
}

fn is_path_empty(begin: (i8, i8), end: (i8, i8), pieces: &Vec<Piece>) -> bool {
    // same col
    if begin.0 == end.0 {
        for piece in pieces {
            if piece.x == begin.0
                && ((piece.y > begin.1 && piece.y < end.1)
                    ||  (piece.y > end.1 && piece.y  < begin.1))
            {
                    return false
            }
        }
    }
    // same row
    if begin.1 == end.1 {
        for piece in pieces {
            if piece.y == begin.1
                && ((piece.x > begin.0 && piece.x < end.0)
                    ||  (piece.x > end.0 && piece.x  < begin.0))
            {
                    return false
            }
        }
    }

    // Diagnols
    let x_diff = (begin.0 as i8 - end.0 as i8).abs();
    let y_diff = (begin.1 as i8 - end.1 as i8).abs();

    if x_diff == y_diff {
        for i in 1..x_diff {
            let pos = if begin.0 < end.0 && begin.1 < end.1 {
                (begin.0 + i, begin.1 + i)
            } else if begin.0 > end.0 && begin.1 < end.1 {
                (begin.0 - i, begin.1 + i)
            } else if begin.0 < end.0 && begin.1 > end.1 {
                (begin.0 + i, begin.0 - i)
            } else {
                (begin.0 - i, begin.1 - i)
            };

            if color_of_square(pos, pieces).is_some() {
                return false
            }
        }
    }

    true
}