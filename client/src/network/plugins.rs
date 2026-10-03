use crate::network::resources::{receive_server_messages, setup_network, update_network};

use super::*;

pub struct NetworkPlugin;
impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MyColor>()
            .add_systems(Startup, setup_network)
            .add_systems(Update, update_network)
            .add_systems(Update, receive_server_messages);
    }
}
