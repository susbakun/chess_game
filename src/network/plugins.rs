use super::*;
use crate::game_state::*;

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            connect_to_spacetimedb
                .run_if(is_loading_multiplayer)
                .run_if(not(resource_exists::<SpacetimeConnection>)),
        )
        .add_systems(
            Update,
            process_spacetimedb_messages.run_if(resource_exists::<SpacetimeConnection>),
        )
        .add_systems(
            Update,
            cleanup_network
                .run_if(back_to_main_menu)
                .run_if(resource_exists::<SpacetimeConnection>),
        );
    }
}
