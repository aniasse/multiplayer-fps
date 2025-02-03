use tokio::net::UdpSocket;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::game_state::GameState;
use crate::handlers;
use crate::messages::ClientMessage;

pub async fn start_server(
    socket: Arc<UdpSocket>,
    game_state: Arc<Mutex<GameState>>
) -> Result<(), Box<dyn std::error::Error>> {
    let game_state_clone = Arc::clone(&game_state);
    let socket_clone = Arc::clone(&socket);

    tokio::spawn(async move {
        if let Err(e) = handlers::check_game_over(game_state_clone, socket_clone).await {
            eprintln!("Error in game over check: {}", e);
        }
    }); 

    loop {
        let mut buf = vec![0u8; 4096];
        let (len, addr) = socket.recv_from(&mut buf).await?;
        let message: ClientMessage = serde_json::from_slice(&buf[..len])?;
        handlers::handle_message(message, addr, Arc::clone(&game_state), Arc::clone(&socket)).await?;
    }
}