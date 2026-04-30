#![warn(clippy::all, clippy::pedantic)]
use bevy::prelude::*;
use bevy::window::WindowResolution;

mod pieces;
use pieces::*;


fn main() {
    App::new()
        .add_systems(Startup, setup)
        .add_systems(Startup, create_board)
        .add_systems(Startup, create_pieces)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(
                    1600, 
                    1600)
                    .with_scale_factor_override(1.0),
                title: "Chess!".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .run();
}


fn setup(
    mut commands: Commands
) {
    commands
        .spawn((
            Camera3d::default(),
            Msaa::Sample4,
            Transform::from_matrix(
                // | R R R | Tx |
                // | R R R | Ty |
                // | R R R | Tz |
                // | 0 0 0 | 1  |
                // instead of (pitch, yaw, roll)
                Mat4::from_rotation_translation(
                    // around this axis (should be normalized)
                Quat::from_xyzw(-0.3, -0.5, -0.3, 0.5).normalize(), 
                Vec3::new(-7.0, 20.0, 4.0)
            ))
    ));


    commands.spawn((
        DirectionalLight {
            illuminance: 20000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::default().looking_to(
            Vec3::new(-1.0, -1.0, -1.0), Vec3::Y),
    ));
}

fn create_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    let mesh = meshes.add(Plane3d::new(
        Vec3::new(0.0, 1.0, 0.0), 
        Vec2::new(0.5, 0.5)));

    let white_material = materials.add(
        Color::linear_rgb(1.0, 0.9, 0.9));

    let black_material = materials.add(
        Color::linear_rgb(0.0, 0.1, 0.1)
    );

    for i in 0..8 {
        for j in 0..8 {
            if (i + j + 1) % 2 == 0 {
                commands.spawn((
                    Mesh3d(mesh.clone()),
                    MeshMaterial3d(white_material.clone()),
                    Transform::from_translation(Vec3::new(i as f32, 0.0, j as f32)),
                ));
            } else {
                commands.spawn((
                    Mesh3d(mesh.clone()),
                    MeshMaterial3d(black_material.clone()),
                    Transform::from_translation(Vec3::new(i as f32, 0.0, j as f32)),
                ));
            }
        }
    }
}

fn create_pieces(
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