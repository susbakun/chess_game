use bevy::prelude::*;

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

pub fn create_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    let mesh = meshes.add(Plane3d::new(
        Vec3::new(0.0, 1.0, 0.0), 
        Vec2::new(0.5, 0.5)));

    for i in 0..8 {
        for j in 0..8 {
            let is_white = (i + j + 1) % 2 == 0;
            let color = if is_white {
                Color::linear_rgb(1.0, 0.9, 0.9)
            } else {
                Color::linear_rgb(0.0, 0.1, 0.1)
            };

            let material = materials.add(color);

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
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(&MeshMaterial3d<StandardMaterial>, &Square)>,
) {
    if let Ok((material_handle, _square)) = 
        query.get(_over.entity) {
        if let Some(material) = 
            materials.get_mut(material_handle) {
                // highlight color
                material.base_color = Color::linear_rgb(
                    0.8, 
                    0.3, 
                    0.3
                );
        }
    }
}

fn on_square_hover_end(
    _out: On<Pointer<Out>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(&MeshMaterial3d<StandardMaterial>, &Square)>,
) {
    if let Ok((material_handle, square)) = 
        query.get(_out.entity) {
        if let Some(material) = 
            materials.get_mut(material_handle) {
                material.base_color = if square.is_white() {
                    Color::linear_rgb(
                        1.0, 
                        0.9, 
                        0.9
                    )
                } else {
                    Color::linear_rgb(
                        0.0, 
                        0.1, 
                        0.1
                    )
                };
        }
    }
}

fn on_sqaure_click(
    _click: On<Pointer<Click>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(&MeshMaterial3d<StandardMaterial>, &Square)>,
) {
    if _click.button != PointerButton::Primary {
        return
    }

    if let Ok((material_handle, _square)) = 
        query.get(_click.entity) {
        if let Some(material) = 
            materials.get_mut(material_handle) {
                material.base_color = Color::linear_rgb(
                    0.9, 
                    0.1, 
                    0.1
                )
            }
    }
}