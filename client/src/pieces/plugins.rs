use super::*;

pub struct PiecePlugin;
impl Plugin for PiecePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PieceHandles>()
            .add_systems(Startup, create_pieces)
            .add_systems(Update, move_pieces);
    }
}
