use bevy::prelude::*;

use crate::pieces::*;
use crate::game_state::*;

mod components;
mod plugins;
mod resources;
mod messages;
mod event_handlers;
mod utils;
mod select;


pub use components::*;
pub use plugins::*;
pub use resources::*;
pub use messages::*;
pub use event_handlers::*;
use utils::*;
use select::*;

pub fn create_board(
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


