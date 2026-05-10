
use super::*;

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, init_next_move_text)
            .add_systems(Startup, init_winner_text)
            .add_systems(Update, next_move_text_update
                .run_if(resource_changed::<Player>))
            .add_systems(Update, show_winner_text);
    }
}