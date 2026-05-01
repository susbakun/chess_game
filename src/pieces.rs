use bevy::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum PieceColor {
    White,
    Black
}

pub enum PieceType {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn
}

#[derive(Component)]
pub struct Piece {
    pub color: PieceColor,
    pub piece_type: PieceType,
    // current position
    pub x: u8,
    pub y: u8
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
                    .with_scale(Vec3::new(0.2, 0.2, 0.2))
            ));

            parent.spawn((
                Mesh3d(mesh_cross.clone()),
                MeshMaterial3d(material.clone()),
                Transform::from_translation(Vec3::new(-0.2, 0.0, -1.9))
                    .with_scale(Vec3::new(0.2, 0.2, 0.2))
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
                vec3(position.0 as f32, 0.0, position.1 as f32)
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
            ))
        .with_children(|parent| {
            parent.spawn((
                Mesh3d(mesh_1),
                MeshMaterial3d(material.clone()),
                Transform::from_translation(vec3(-0.2, 0.0, 0.9))
                    .with_scale(vec3(0.2, 0.2, 0.2))
            ));
            parent.spawn((
                Mesh3d(mesh_2),
                MeshMaterial3d(material.clone()),
                Transform::from_translation(vec3(-0.2, 0.0, 0.9))
                    .with_scale(vec3(0.2, 0.2, 0.2))
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
                    .with_scale(vec3(0.2, 0.2, 0.2))
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
                ).with_scale(vec3(0.2, 0.2, 0.2))
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
                ).with_scale(vec3(0.2, 0.2, 0.2))
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
                ).with_scale(vec3(0.2, 0.2, 0.2))
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