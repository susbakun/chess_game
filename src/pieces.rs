use bevy::prelude::*;


fn spawn_king(
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


fn spawn_knight(
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

fn spawn_queen(
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


fn spawn_bishop(
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

fn spawn_rook(
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

fn spawn_pawn(
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
        vec3(0.0, 0.0, 0.0)
    );
    spawn_knight(
        commands.reborrow(), 
        white_material.clone(), 
        knight_1_handle.clone(),
        knight_2_handle.clone(),
        vec3(0.0, 0.0, 1.0),
        false
    );
    spawn_bishop(
        commands.reborrow(), 
        bishop_handle.clone(), 
        white_material.clone(), 
        vec3(0.0, 0.0, 2.0)
    );
    spawn_queen(
        commands.reborrow(), 
        white_material.clone(), 
        queen_handle.clone(), 
        vec3(0.0, 0.0, 3.0)
    );
    spawn_king(
        commands.reborrow(), 
        white_material.clone(), 
        king_handle.clone(), 
        king_cross_handle.clone(), 
        Vec3::new(0.0, 0.0, 4.0)
    );
    spawn_bishop(
        commands.reborrow(), 
        bishop_handle.clone(), 
        white_material.clone(), 
        vec3(0.0, 0.0, 5.0)
    );
    spawn_knight(
        commands.reborrow(), 
        white_material.clone(), 
        knight_1_handle.clone(),
        knight_2_handle.clone(),
        vec3(0.0, 0.0, 6.0),
        false
    );
    spawn_rook(
        commands.reborrow(), 
        rook_handle.clone(), 
        white_material.clone(), 
        vec3(0.0, 0.0, 7.0)
    );

    for i in 0..8 {
        spawn_pawn(
            commands.reborrow(), 
            pawn_handle.clone(), 
            white_material.clone(), 
            vec3(1., 0., i as f32)
        );
    }

    spawn_rook(
        commands.reborrow(),
        rook_handle.clone(),
        black_material.clone(),
        Vec3::new(7., 0., 0.),
    );
    spawn_knight(
        commands.reborrow(),
        black_material.clone(),
        knight_1_handle.clone(),
        knight_2_handle.clone(),
        Vec3::new(7., 0., 1.),
        true
    );
    spawn_bishop(
        commands.reborrow(),
        bishop_handle.clone(),
        black_material.clone(),
        Vec3::new(7., 0., 2.),
    );
    spawn_queen(
        commands.reborrow(),
        black_material.clone(),
        queen_handle.clone(),
        Vec3::new(7., 0., 3.),
    );
    spawn_king(
        commands.reborrow(),
        black_material.clone(),
        king_handle.clone(),
        king_cross_handle.clone(),
        Vec3::new(7., 0., 4.),
    );
    spawn_bishop(
        commands.reborrow(),
        bishop_handle.clone(),
        black_material.clone(),
        Vec3::new(7., 0., 5.),
    );
    spawn_knight(
        commands.reborrow(),
        black_material.clone(),
        knight_1_handle.clone(),
        knight_2_handle.clone(),
        Vec3::new(7., 0., 6.),
        true
    );
    spawn_rook(
        commands.reborrow(),
        rook_handle.clone(),
        black_material.clone(),
        Vec3::new(7., 0., 7.),
    );

    for i in 0..8 {
        spawn_pawn(
            commands.reborrow(),
            pawn_handle.clone(),
            black_material.clone(),
            Vec3::new(6., 0., i as f32),
        );
    }
}