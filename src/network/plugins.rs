use super::*;
use crate::game_state::*;

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<OnlineSession>()
            .add_systems(
                Update,
                connect_to_spacetimedb
                    .run_if(is_loading_multiplayer)
                    .run_if(not(resource_exists::<SpacetimeConnection>)),
            )
            .add_systems(
                Update,
                (
                    process_spacetimedb_messages,
                    sync_online_session.after(process_spacetimedb_messages),
                    end_multiplayer_loading.after(sync_online_session),
                )
                    .run_if(resource_exists::<SpacetimeConnection>),
            )
            .add_systems(
                Update,
                cleanup_network
                    .run_if(back_to_main_menu)
                    .run_if(resource_exists::<SpacetimeConnection>),
            )
            .add_systems(
                Update,
                reset_online_session.run_if(back_to_main_menu),
            );
    }
}
