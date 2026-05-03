use bevy::prelude::*;

pub struct PiecePlugin;
impl Plugin for PiecePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, create_pieces)
            .add_systems(Update, move_pieces);
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum PieceColor {
    White,
    Black
}

#[derive(Clone, Copy, PartialEq)]
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
    pub x: u8,
    pub y: u8
}


impl Piece {
    pub fn is_move_valid(&self, new_pos: (u8, u8), pieces: Vec<Piece>) -> bool {
        // If there's a piece of the same color in the same square, it can't move
        if color_of_square(new_pos, &pieces) == Some(self.color) {
            return false
        }

        match self.piece_type {
            PieceType::King => {
                // Horizontal
                ((self.x as i8 - new_pos.0 as i8).abs() == 1
                    && (self.y == new_pos.1))
                // Vertical
                ||  ((self.y as i8 - new_pos.1 as i8).abs() == 1
                    && (self.x == new_pos.0))
                // Diagonal
                ||  ((self.x as i8 - new_pos.0 as i8).abs() == 1
                    && (self.y as i8 - new_pos.1 as i8).abs() == 1)
            }
            PieceType::Queen => {
                is_path_empty((self.x, self.y), new_pos, &pieces)
                    && ((self.x as i8 - new_pos.0 as i8).abs()
                        == (self.y as i8 - new_pos.1 as i8).abs()
                        ||  ((self.x == new_pos.0 && self.y != new_pos.1)
                            ||  (self.y == new_pos.1 && self.x != new_pos.0)))
            },
            PieceType::Bishop => {
                is_path_empty((self.x, self.y), new_pos, &pieces)
                    && (self.x as i8 - new_pos.0 as i8).abs()
                        == (self.y as i8 - new_pos.1 as i8).abs()
            },
            PieceType::Knight => {
                ((self.x as i8 - new_pos.0 as i8).abs() == 1 
                    && (self.y as i8 - new_pos.1 as i8).abs() == 2)
                ||  ((self.x as i8 - new_pos.0 as i8).abs() == 2
                    && (self.y as i8 - new_pos.1 as i8).abs() == 1)
            },
            PieceType::Rook => {
                is_path_empty((self.x, self.y), new_pos, &pieces)
                    && ((self.x == new_pos.0 && self.y != new_pos.1)
                        || (self.y == new_pos.1 && self.x != new_pos.0))
            },
            PieceType::Pawn => {
                if self.color == PieceColor::White {
                    if new_pos.0 as i8 - self.x as i8 == 1 && (self.y == new_pos.1) {
                        if color_of_square(new_pos, &pieces).is_none() {
                            return true
                        }
                    }
    
                    if self.x == 1
                        && new_pos.0 as i8 - self.x as i8 == 2
                        && (self.y == new_pos.1)
                        && is_path_empty((self.x, self.y), new_pos, &pieces) {
                            if color_of_square(new_pos, &pieces).is_none() {
                                return true
                            }
                    }
    
                    if new_pos.0 as i8 - self.x as i8 == 1
                        && (self.y as i8 - new_pos.1 as i8).abs() == 1 {
                            if color_of_square(new_pos, &pieces) == Some(PieceColor::Black) {
                                return true
                            }
                    }
                } else {
                    if new_pos.0 as i8 - self.x as i8 == -1 && (self.y == new_pos.1) {
                        if color_of_square(new_pos, &pieces).is_none() {
                            return true
                        }
                    }
    
                    if self.x == 6
                        && new_pos.0 as i8 - self.x as i8 == -2
                        && (self.y == new_pos.1)
                        && is_path_empty((self.x, self.y), new_pos, &pieces) {
                            if color_of_square(new_pos, &pieces).is_none() {
                                return true
                            }
                    }
    
                    if new_pos.0 as i8 - self.x as i8 == -1
                        && (self.y as i8 - new_pos.1 as i8).abs() == 1 {
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



fn spawn_king(
    mut commands: Commands,
    material: Handle<StandardMaterial>,
    mesh: Handle<Mesh>,
    mesh_cross: Handle<Mesh>,
    piece_color: PieceColor,
    position: (u8, u8)
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
    position: (u8, u8),
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
    position: (u8, u8)
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
    position: (u8, u8)
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
    position: (u8, u8)
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
    position: (u8, u8)
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


pub fn create_pieces(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>
) {
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

    let white_material = materials.add(
        Color::linear_rgb(1.0, 0.9, 0.9));

    let black_material = materials.add(
        Color::linear_rgb(0.0, 0.1, 0.1)
    );

    spawn_rook(
        commands.reborrow(), 
        rook_handle.clone(), 
        white_material.clone(), 
        PieceColor::White,
        (0, 0)
    );
    spawn_knight(
        commands.reborrow(), 
        white_material.clone(), 
        knight_1_handle.clone(),
        knight_2_handle.clone(),
        PieceColor::White,
        (0, 1),
        false
    );
    spawn_bishop(
        commands.reborrow(), 
        bishop_handle.clone(), 
        white_material.clone(),
        PieceColor::White,
        (0, 2)
    );
    spawn_queen(
        commands.reborrow(), 
        white_material.clone(), 
        queen_handle.clone(),
        PieceColor::White,
        (0, 3)
    );
    spawn_king(
        commands.reborrow(), 
        white_material.clone(), 
        king_handle.clone(), 
        king_cross_handle.clone(),
        PieceColor::White,
        (0, 4)
    );
    spawn_bishop(
        commands.reborrow(), 
        bishop_handle.clone(), 
        white_material.clone(),
        PieceColor::White,
        (0, 5)
    );
    spawn_knight(
        commands.reborrow(), 
        white_material.clone(), 
        knight_1_handle.clone(),
        knight_2_handle.clone(),
        PieceColor::White,
        (0, 6),
        false
    );
    spawn_rook(
        commands.reborrow(), 
        rook_handle.clone(), 
        white_material.clone(),
        PieceColor::White,
        (0, 7)
    );

    for i in 0..8 {
        spawn_pawn(
            commands.reborrow(), 
            pawn_handle.clone(), 
            white_material.clone(),
            PieceColor::White,
            (1, i)
        );
    }

    spawn_rook(
        commands.reborrow(),
        rook_handle.clone(),
        black_material.clone(),
        PieceColor::Black,
        (7, 0),
    );
    spawn_knight(
        commands.reborrow(),
        black_material.clone(),
        knight_1_handle.clone(),
        knight_2_handle.clone(),
        PieceColor::Black,
        (7, 1),
        true
    );
    spawn_bishop(
        commands.reborrow(),
        bishop_handle.clone(),
        black_material.clone(),
        PieceColor::Black,
        (7, 2),
    );
    spawn_queen(
        commands.reborrow(),
        black_material.clone(),
        queen_handle.clone(),
        PieceColor::Black,
        (7, 3),
    );
    spawn_king(
        commands.reborrow(),
        black_material.clone(),
        king_handle.clone(),
        king_cross_handle.clone(),
        PieceColor::Black,
        (7, 4),
    );
    spawn_bishop(
        commands.reborrow(),
        bishop_handle.clone(),
        black_material.clone(),
        PieceColor::Black,
        (7, 5),
    );
    spawn_knight(
        commands.reborrow(),
        black_material.clone(),
        knight_1_handle.clone(),
        knight_2_handle.clone(),
        PieceColor::Black,
        (7, 6),
        true
    );
    spawn_rook(
        commands.reborrow(),
        rook_handle.clone(),
        black_material.clone(),
        PieceColor::Black,
        (7, 7),
    );

    for i in 0..8 {
        spawn_pawn(
            commands.reborrow(),
            pawn_handle.clone(),
            black_material.clone(),
            PieceColor::Black,
            (6, i),
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

fn color_of_square(pos: (u8, u8), pieces: &Vec<Piece>) -> Option<PieceColor> {
    for piece in pieces {
        if piece.x == pos.0 && piece.y == pos.1 {
            return Some(piece.color)
        }
    }

    None
}

fn is_path_empty(begin: (u8, u8), end: (u8, u8), pieces: &Vec<Piece>) -> bool {
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
                (begin.0 + i as u8, begin.1 + i as u8)
            } else if begin.0 > end.0 && begin.1 < end.1 {
                (begin.0 - i as u8, begin.1 + i as u8)
            } else if begin.0 < end.0 && begin.1 > end.1 {
                (begin.0 + i as u8, begin.0 - i as u8)
            } else {
                (begin.0 - i as u8, begin.1 - i as u8)
            };

            if color_of_square(pos, pieces).is_some() {
                return false
            }
        }
    }

    true
}