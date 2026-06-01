use crate::game_state::timer::change_timer;

use super::*;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameState>()
            .add_systems(Update, change_timer);
    }
}
