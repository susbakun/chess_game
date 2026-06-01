#![warn(clippy::all, clippy::pedantic)]
use bevy::dev_tools::picking_debug::{DebugPickingMode, DebugPickingPlugin};
use bevy::input_focus::InputFocus;
use bevy::prelude::*;
use bevy::window::WindowResolution;

mod board;
use board::*;
mod pieces;
use pieces::*;
mod ui;
use ui::*;
mod constants;
mod game_state;
mod player;
use game_state::*;
mod replay;
use replay::*;
mod engine;

fn main() {
    App::new()
        .init_resource::<InputFocus>()
        .insert_resource(DebugPickingMode::Disabled)
        .add_systems(Startup, setup)
        .add_systems(
            PreUpdate,
            (|mut mode: ResMut<DebugPickingMode>| {
                *mode = match *mode {
                    DebugPickingMode::Disabled => DebugPickingMode::Normal,
                    DebugPickingMode::Normal => DebugPickingMode::Noisy,
                    DebugPickingMode::Noisy => DebugPickingMode::Disabled,
                }
            })
            .distributive_run_if(bevy::input::common_conditions::input_just_pressed(
                KeyCode::F3,
            )),
        )
        .add_systems(Update, replay)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(1600, 1600).with_scale_factor_override(1.0),
                title: "Chess!".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(GamePlugin)
        .add_plugins((MeshPickingPlugin, DebugPickingPlugin))
        .add_plugins(SquarePlugin)
        .add_plugins(PiecePlugin)
        .add_plugins(UIPlugin)
        .add_message::<BackToMenuEvent>()
        .run();
}

pub fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Msaa::Sample4,
        Transform::from_matrix(Mat4::from_rotation_translation(
            Quat::from_xyzw(-0.3, -0.5, -0.3, 0.5).normalize(),
            Vec3::new(-7.0, 20.0, 4.0),
        )),
    ));

    commands.spawn((
        DirectionalLight {
            illuminance: 20000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::default().looking_to(Vec3::new(-1.0, -1.0, -1.0), Vec3::Y),
    ));
}
