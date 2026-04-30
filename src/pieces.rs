use bevy::prelude::*;


pub fn spawn_king(
    mut commands: Commands,
    material: Handle<StandardMaterial>,
    mesh: Handle<Mesh>,
    mesh_cross: Handle<Mesh>,
    position: Vec3
) {
    commands
        .spawn((
            Transform::from_translation(position),
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


pub fn spawn_knight(
    mut commands: Commands,
    material: Handle<StandardMaterial>,
    mesh_1: Handle<Mesh>,
    mesh_2: Handle<Mesh>,
    position: Vec3,
    rotate: bool
) {
    commands
        .spawn(
            Transform::from_translation(position)
                .with_rotation(
                    if rotate {
                        Quat::from_xyzw(0.0, 1.0, 0.0, 0.0)
                            .normalize()
                    } else {
                        Quat::from_xyzw(0.0, 0.0, 0.0, 0.0)
                    }
                )
        )
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

pub fn spawn_queen(
    mut commands: Commands,
    material: Handle<StandardMaterial>,
    mesh: Handle<Mesh>,
    position: Vec3
) {
    commands
        .spawn(
            Transform::from_translation(position)
        )
        .with_children(|parent| {
            parent.spawn((
                Mesh3d(mesh),
                MeshMaterial3d(material),
                Transform::from_translation(vec3(-0.2, 0.0, -0.95))
                    .with_scale(vec3(0.2, 0.2, 0.2))
            ));
        });
}


pub fn spawn_bishop(
    mut commands: Commands,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    position: Vec3
) {
    commands
        .spawn(
            Transform::from_translation(position)
        )
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

pub fn spawn_rook(
    mut commands: Commands,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    position: Vec3
) {
    commands
        .spawn(
            Transform::from_translation(position)
        )
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

pub fn spawn_pawn(
    mut commands: Commands,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    position: Vec3
) {
    commands
        .spawn(
            Transform::from_translation(position)
        )
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