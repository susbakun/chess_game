// server/src/main.rs
use renet::{DefaultChannel, RenetServer, ServerEvent};
use renet_netcode::{NetcodeServerTransport, ServerAuthentication, ServerConfig};
use shared::network::{connection_config, ClientMessage, ServerMessage};
use shared::ClientMessage::SendEvent;
use shared::ServerMessage::{AssignColor, SyncState, SyncTimer};
use shared::{GameEvent, GameState, PieceColor};
use std::collections::HashMap;
use std::net::{SocketAddr, UdpSocket};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

fn main() {
    let mut server = RenetServer::new(connection_config());

    let server_addr: SocketAddr = "127.0.0.1:5000".parse().unwrap();
    let socket = UdpSocket::bind(server_addr).unwrap();
    let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let config = ServerConfig {
        current_time,
        max_clients: 2,
        protocol_id: 0,
        public_addresses: vec![server_addr],
        authentication: ServerAuthentication::Unsecure,
    };
    let mut transport = NetcodeServerTransport::new(config, socket).unwrap();

    let mut game_state = GameState::default();
    let dt = Duration::from_millis(16);

    let mut client_colors: HashMap<u64, PieceColor> = HashMap::new();

    let mut timer_accumulator = Duration::ZERO;

    loop {
        server.update(dt);
        transport.update(dt, &mut server).unwrap();

        timer_accumulator += dt;
        if timer_accumulator >= Duration::from_secs(1) {
            timer_accumulator -= Duration::from_secs(1);
            let server_message = if game_state.tick_timer(1) {
                SyncState(game_state.clone())
            } else {
                SyncTimer(game_state.timer.0, game_state.timer.1)
            };

            let serialized_message =
                bincode::serialize(&server_message).expect("Expected server message");
            server.broadcast_message(DefaultChannel::ReliableOrdered, serialized_message);
        }

        while let Some(event) = server.get_event() {
            match event {
                ServerEvent::ClientConnected { client_id } => {
                    println!("client {client_id} connected");

                    let color = if !client_colors.values().any(|c| *c == PieceColor::White) {
                        PieceColor::White
                    } else if !client_colors.values().any(|c| *c == PieceColor::Black) {
                        PieceColor::Black
                    } else {
                        eprintln!("game full, ignoring client {client_id}");
                        continue;
                    };
                    client_colors.insert(client_id, color);

                    let server_message: ServerMessage = AssignColor(color);
                    let serialized_message = bincode::serialize(&server_message).unwrap();
                    server.send_message(
                        client_id,
                        DefaultChannel::ReliableOrdered,
                        serialized_message,
                    );

                    let player_joined_event = GameEvent::PlayerJoined {
                        player_id: client_id,
                        color,
                    };

                    game_state.consume(&player_joined_event);

                    if client_colors.len() == 2 {
                        let begin_game_event = GameEvent::BeginGame;
                        game_state.consume(&begin_game_event);

                        let server_message = ServerMessage::SyncState(game_state.clone());
                        let serialized_message =
                            bincode::serialize(&server_message).expect("Expected server message");
                        server
                            .broadcast_message(DefaultChannel::ReliableOrdered, serialized_message);
                    }
                }
                ServerEvent::ClientDisconnected { client_id, reason } => {
                    println!("client {client_id} disconnected: {reason:?}");
                    client_colors.remove(&client_id);

                    let player_disconnected_event = GameEvent::PlayerDisconnected {
                        player_id: client_id,
                    };
                    game_state.consume(&player_disconnected_event);

                    let server_message = ServerMessage::SyncState(game_state.clone());
                    let serialized_message =
                        bincode::serialize(&server_message).expect("Expected server message");
                    server.broadcast_message(DefaultChannel::ReliableOrdered, serialized_message);
                }
            }
        }

        for client_id in server.clients_id() {
            while let Some(bytes) =
                server.receive_message(client_id, DefaultChannel::ReliableOrdered)
            {
                let client_message: ClientMessage =
                    bincode::deserialize(&bytes).expect("Expected client message");

                match client_message {
                    SendEvent(event) => {
                        if game_state.validate(&event) {
                            game_state.consume(&event);
                            let server_message = ServerMessage::SyncState(game_state.clone());
                            let serialized_message = bincode::serialize(&server_message)
                                .expect("Expected server message");
                            server.broadcast_message(
                                DefaultChannel::ReliableOrdered,
                                serialized_message,
                            );
                        }
                    }
                }
            }
        }

        transport.send_packets(&mut server);
        std::thread::sleep(dt);
    }
}
