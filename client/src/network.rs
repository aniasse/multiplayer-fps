use std::net::UdpSocket;
use bevy::prelude::*;
use bevy::window::WindowClosed;
use crate::messages::{ClientMessage, ServerMessage};
use crate::game_state::{GameState, AppState};
use crossbeam_channel::{unbounded, Receiver, Sender};
use tokio::signal;

#[derive(Resource)]
pub struct NetworkReceiver(pub Receiver<ServerMessage>);
#[derive(Resource)]
pub struct NetworkSender(pub Sender<ClientMessage>);
use std::sync::Arc;
pub async fn setup_network(server_addr: &str, player_name: &str) -> Result<(Sender<ServerMessage>, Receiver<ServerMessage>, Sender<ClientMessage>), Box<dyn std::error::Error>> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    if let Err(e)=socket.connect(server_addr){
        panic!("le server ne reponds pas <==== {} ====>",e);
    };
    let ping=serde_json::to_string(&ClientMessage::Ping)?;
    println!("Connected to server at {}", server_addr);
    let (network_sender, network_receiver) = unbounded::<ServerMessage>();
    let (client_sender, client_receiver) = unbounded::<ClientMessage>();
    let socket = Arc::new(socket);
    let send_socket = Arc::clone(&socket);
    // Envoyer le message de connexion
    let join_message = ClientMessage::Join { name: player_name.to_string() };
    let serialized = serde_json::to_string(&join_message)?;
    socket.send(serialized.as_bytes())?;
    println!("Join message sent to server");
    // Clone network_sender pour l'utiliser dans la boucle de réception
    let network_sender_clone = network_sender.clone();
    
    // Lancer la boucle de réception
    tokio::spawn(async move {
        let mut buf = vec![0u8; 4096];
        loop {
            match socket.recv(&mut buf) {
                Ok(n) => {
                    if let Ok(message) = serde_json::from_slice::<ServerMessage>(&buf[..n]) {
                        if let Err(e) = network_sender_clone.send(message) {
                            eprintln!("Failed to send message to main thread: {}", e);
                        }
                    } else {
                        eprintln!("Failed to parse server message");
                    }
                }
                Err(e) => panic!("Failed to receive data: {}", e),
            }
        }
    });
    // Lancer la boucle d'envoi
    tokio::spawn(async move {
        loop {
            if let Ok(message) = client_receiver.try_recv() {
                let serialized = serde_json::to_string(&message).unwrap();
                if let Err(e) = send_socket.send(serialized.as_bytes()) {
                    panic!("Failed to send message: {}", e);
                }
            }
            let _=send_socket.send(ping.as_bytes());
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        }
    });
    Ok((network_sender, network_receiver, client_sender))
}

pub(crate) fn handle_network_messages(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    network_receiver: Res<NetworkReceiver>,
    mut app_state: ResMut<NextState<AppState>>,
){
    for message in network_receiver.0.try_iter() {
        println!("Received message: {:?}", message);
        match message {
            ServerMessage::Welcome { map, player_id } => {
                println!("Received Welcome message with map");
                game_state.map = Some(map);
                game_state.player_id = Some(player_id);
                game_state.map_rendered = false;  // Force map re-render
                
                // Trigger map rendering
                commands.insert_resource(NextState(Some(AppState::RenderMap)));
            }
            ServerMessage::GameState { players } => {
                game_state.players = players;
            }
            ServerMessage::PlayerShot { shooter, target } => {
                if Some(target.clone()) == game_state.player_id {
                    println!("You were shot by {}!", shooter);
                } else {
                    println!("Player {} was shot by {}!", target, shooter);
                }
            }
            ServerMessage::PlayerDied { player } => {
                if Some(player.clone()) == game_state.player_id {
                    game_state.is_alive = false;
                    println!("You died!");
                } else {
                    println!("Player {} died!", player);
                }
            }
            ServerMessage::GameOver { winner, scores } => {
                println!("Game Over! Winner: {}", winner);
                println!("Scores:");
                for (name, score) in &scores {
                    println!("{}: {}", name, score);
                }
                game_state.game_over_results = Some((winner, scores));
                app_state.set(AppState::GameOver);
            },
            ServerMessage::Pong=>{
                println!("Pong from server");
            }
        }
    }
}


pub fn disconnected(mut closed: EventReader<WindowClosed>){
   
}