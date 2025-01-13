mod game_state;
mod map;
mod player;
mod messages;
mod network;
mod handlers;

use tokio::net::UdpSocket;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Choose difficulty level (1: Easy, 2: Medium, 3: Hard):");
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let difficulty: u8 = input.trim().parse().unwrap_or(2);

    let socket = UdpSocket::bind("0.0.0.0:5000").await?;
    let socket = Arc::new(socket);
    let game_state = Arc::new(Mutex::new(game_state::GameState::new(difficulty)));

    println!("Server listening on {}", socket.local_addr()?);

    network::start_server(socket, game_state).await
}