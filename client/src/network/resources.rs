// client/src/network.rs
use bevy::prelude::*;
use renet::{DefaultChannel, RenetClient};
use renet_netcode::{ClientAuthentication, NetcodeClientTransport};
use shared::network::connection_config;
use shared::{PieceColor, ServerMessage};
use std::net::{SocketAddr, UdpSocket};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::board::{Taken, render_taken_on_side};
use crate::game_state::ClientGameState;
use crate::pieces::{PieceComponent, PieceHandles, spawn_piece};

#[derive(Resource)]
pub struct ClientNetwork {
    pub client: RenetClient,
    pub transport: NetcodeClientTransport,
}

pub fn setup_network(mut commands: Commands) {
    let client = RenetClient::new(connection_config());

    let server_addr: SocketAddr = "127.0.0.1:5000".parse().unwrap();
    let socket = UdpSocket::bind("127.0.0.1:0").unwrap(); // :0 = OS picks a free local port
    let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let client_id = current_time.as_millis() as u64; // fine for 2 players, not a real auth scheme
    let authentication = ClientAuthentication::Unsecure {
        server_addr,
        client_id,
        user_data: None,
        protocol_id: 0,
    };
    let transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();

    commands.insert_resource(ClientNetwork { client, transport });
}

pub fn update_network(net: ResMut<ClientNetwork>, time: Res<Time>) {
    let ClientNetwork { client, transport } = net.into_inner();
    client.update(time.delta());
    let _ = transport.update(time.delta(), client);
    let _ = transport.send_packets(client); // <- add this
}

pub fn receive_server_messages(
    mut commands: Commands,
    mut net: ResMut<ClientNetwork>,
    mut game_state: ResMut<ClientGameState>,
    pieces_query: Query<Entity, With<PieceComponent>>,
    taken_pieces_query: Query<Entity, With<Taken>>,
    piece_handles: Res<PieceHandles>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    while let Some(bytes) = net.client.receive_message(DefaultChannel::ReliableOrdered) {
        let server_message: ServerMessage =
            bincode::deserialize(&bytes).expect("Expected server message.");

        match server_message {
            ServerMessage::SyncState(new_state) => {
                for entity in pieces_query.iter() {
                    commands.entity(entity).despawn();
                }

                for entity in taken_pieces_query.iter() {
                    commands.entity(entity).despawn();
                }

                let white_material = materials.add(Color::linear_rgb(1.0, 0.9, 0.9));
                let black_material = materials.add(Color::linear_rgb(0.0, 0.1, 0.1));

                let mut removed_counts = (0u8, 0u8);

                for piece in &new_state.pieces {
                    let material = if piece.color == PieceColor::White {
                        white_material.clone()
                    } else {
                        black_material.clone()
                    };

                    if piece.taken {
                        if piece.color == PieceColor::White {
                            removed_counts.0 += 1
                        } else {
                            removed_counts.1 += 1
                        }
                        render_taken_on_side(
                            commands.reborrow(),
                            *piece,
                            material,
                            removed_counts,
                            piece_handles.clone(),
                        );
                    } else {
                        spawn_piece(commands.reborrow(), material, piece, piece_handles.clone());
                    }
                }
                game_state.removed_counts = removed_counts;
                game_state.shared = new_state;
            }
            ServerMessage::AssignColor(color) => {
                eprintln!("assigned color: {color:?}"); // needs Debug on PieceColor — add the derive if missing
                commands.insert_resource(MyColor(color));
            }
            ServerMessage::SyncTimer(white, black) => {
                game_state.timer = (white, black);
            }
        }
    }
}

#[derive(Resource, Default)]
pub struct MyColor(pub PieceColor);
