use crate::{game_state::timer::change_timer, not_multiplayer};

use super::*;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ClientGameState>()
            .add_systems(Update, change_timer.run_if(not_multiplayer));
    }
}
