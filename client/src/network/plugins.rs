use crate::{
    game_state::{ClientGameState, GameType},
    network::resources::{receive_server_messages, setup_network, update_network},
};

use super::*;

pub struct NetworkPlugin;
impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, maybe_setup_network).add_systems(
            Update,
            (update_network, receive_server_messages).run_if(resource_exists::<ClientNetwork>),
        );
    }
}

fn maybe_setup_network(
    commands: Commands,
    game_state: ResMut<ClientGameState>,
    existing_network: Option<Res<ClientNetwork>>,
) {
    if game_state.game_type != Some(GameType::Multiplayer) {
        return;
    }

    if existing_network.is_some() {
        return;
    }

    setup_network(commands);
}
