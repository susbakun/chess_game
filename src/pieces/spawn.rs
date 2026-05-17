use super::*;

fn spawn_king(
    mut commands: Commands,
    material: Handle<StandardMaterial>,
    mesh: Handle<Mesh>,
    mesh_cross: Handle<Mesh>,
    piece_color: PieceColor,
    position: (i8, i8),
    taken: bool,
) {
    commands
        .spawn((
            Transform::from_translation(vec3(position.0 as f32, 0.0, position.1 as f32)),
            Piece {
                color: piece_color,
                piece_type: PieceType::King,
                x: position.0,
                y: position.1,
                taken,
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Mesh3d(mesh.clone()),
                MeshMaterial3d(material.clone()),
                Transform::from_translation(Vec3::new(-0.2, 0.0, -1.9))
                    .with_scale(Vec3::new(0.2, 0.2, 0.2)),
                Pickable::IGNORE,
            ));

            parent.spawn((
                Mesh3d(mesh_cross.clone()),
                MeshMaterial3d(material.clone()),
                Transform::from_translation(Vec3::new(-0.2, 0.0, -1.9))
                    .with_scale(Vec3::new(0.2, 0.2, 0.2)),
                Pickable::IGNORE,
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
    rotate: bool,
    taken: bool,
) {
    commands
        .spawn((
            Transform::from_translation(vec3(position.0 as f32, 0.0, position.1 as f32))
                .with_rotation(if rotate {
                    Quat::from_xyzw(0.0, 1.0, 0.0, 0.0).normalize()
                } else {
                    Quat::from_xyzw(0.0, 0.0, 0.0, 0.0)
                }),
            Piece {
                color: piece_color,
                piece_type: PieceType::Knight,
                x: position.0,
                y: position.1,
                taken,
            },
            Pickable::IGNORE,
        ))
        .with_children(|parent| {
            parent.spawn((
                Mesh3d(mesh_1),
                MeshMaterial3d(material.clone()),
                Transform::from_translation(vec3(-0.15, 0.0, 0.85)).with_scale(vec3(0.2, 0.2, 0.2)),
                Pickable::IGNORE,
            ));
            parent.spawn((
                Mesh3d(mesh_2),
                MeshMaterial3d(material.clone()),
                Transform::from_translation(vec3(-0.15, 0.0, 0.85)).with_scale(vec3(0.2, 0.2, 0.2)),
                Pickable::IGNORE,
            ));
        });
}

fn spawn_queen(
    mut commands: Commands,
    material: Handle<StandardMaterial>,
    mesh: Handle<Mesh>,
    piece_color: PieceColor,
    position: (i8, i8),
    taken: bool,
) {
    commands
        .spawn((
            Transform::from_translation(vec3(position.0 as f32, 0.0, position.1 as f32)),
            Piece {
                color: piece_color,
                piece_type: PieceType::Queen,
                x: position.0,
                y: position.1,
                taken,
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Mesh3d(mesh),
                MeshMaterial3d(material),
                Transform::from_translation(vec3(-0.2, 0.0, -0.95)).with_scale(vec3(0.2, 0.2, 0.2)),
                Pickable::IGNORE,
            ));
        });
}

fn spawn_bishop(
    mut commands: Commands,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    piece_color: PieceColor,
    position: (i8, i8),
    taken: bool,
) {
    commands
        .spawn((
            Transform::from_translation(vec3(position.0 as f32, 0.0, position.1 as f32)),
            Piece {
                color: piece_color,
                piece_type: PieceType::Bishop,
                x: position.0,
                y: position.1,
                taken,
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Mesh3d(mesh),
                MeshMaterial3d(material),
                Transform::from_translation(vec3(-0.1, 0.0, 0.0)).with_scale(vec3(0.2, 0.2, 0.2)),
                Pickable::IGNORE,
            ));
        });
}

fn spawn_rook(
    mut commands: Commands,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    piece_color: PieceColor,
    position: (i8, i8),
    taken: bool,
) {
    commands
        .spawn((
            Transform::from_translation(vec3(position.0 as f32, 0.0, position.1 as f32)),
            Piece {
                color: piece_color,
                piece_type: PieceType::Rook,
                x: position.0,
                y: position.1,
                taken,
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Mesh3d(mesh),
                MeshMaterial3d(material),
                Transform::from_translation(vec3(-0.05, 0.0, 1.8)).with_scale(vec3(0.2, 0.2, 0.2)),
                Pickable::IGNORE,
            ));
        });
}

fn spawn_pawn(
    mut commands: Commands,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    piece_color: PieceColor,
    position: (i8, i8),
    taken: bool,
) {
    commands
        .spawn((
            Transform::from_translation(vec3(position.0 as f32, 0.0, position.1 as f32)),
            Piece {
                color: piece_color,
                piece_type: PieceType::Pawn,
                x: position.0,
                y: position.1,
                taken,
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Mesh3d(mesh),
                MeshMaterial3d(material),
                Transform::from_translation(vec3(-0.2, 0.0, 2.6)).with_scale(vec3(0.2, 0.2, 0.2)),
                Pickable::IGNORE,
            ));
        });
}

pub fn spawn_piece(
    mut commands: Commands,
    material: Handle<StandardMaterial>,
    piece: Piece,
    piece_handles: PieceHandles,
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
                position,
                piece.taken,
            );
        }
        PieceType::Queen => {
            spawn_queen(
                commands.reborrow(),
                material,
                piece_handles.queen_handle.clone(),
                piece.color,
                position,
                piece.taken,
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
                rotate,
                piece.taken,
            );
        }
        PieceType::Bishop => {
            spawn_bishop(
                commands.reborrow(),
                piece_handles.bishop_handle.clone(),
                material,
                piece.color,
                position,
                piece.taken,
            );
        }
        PieceType::Rook => {
            spawn_rook(
                commands.reborrow(),
                piece_handles.rook_handle.clone(),
                material,
                piece.color,
                position,
                piece.taken,
            );
        }
        PieceType::Pawn => {
            spawn_pawn(
                commands.reborrow(),
                piece_handles.pawn_handle.clone(),
                material,
                piece.color,
                position,
                piece.taken,
            );
        }
    }
}
