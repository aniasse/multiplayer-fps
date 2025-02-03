mod camera;
mod game_state;
mod input;
mod map;
mod messages;
mod network;
mod player;
mod render;
mod ui;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use camera::{MouseSensitivity, PlayerRotation};
use game_state::{AppState, GameState};
use input::CursorState;
use network::{setup_network, NetworkReceiver, NetworkSender};
use std::io::{self, Write};
use tokio::runtime::Runtime;

fn main() -> io::Result<()> {
    println!("Choose server address:");
    println!("1. Use default (0.0.0.0:5000)");
    println!("2. Enter manually");  
    io::stdout().flush()?;
    
    let mut choice = String::new();
    io::stdin().read_line(&mut choice)?;

    let server_addr = match choice.trim() {
        "1" => String::from("0.0.0.0:5000"),
        "2" => {
            println!("Enter server IP:port (e.g., 127.0.0.1:5000): ");
            io::stdout().flush()?;
            let mut addr = String::new();
            io::stdin().read_line(&mut addr)?;
            addr.trim().to_string()
        }
        _ => {
            println!("Invalid choice. Using default (0.0.0.0:5000)");
            String::from("0.0.0.0:5000")
        }
    };
    
    println!("Enter UserName: ");
    io::stdout().flush()?;
    let mut player_name = String::new();
    io::stdin().read_line(&mut player_name)?;
    let player_name = player_name.trim().to_string();
    let rt = Runtime::new().unwrap();
    let (_network_sender, network_receiver, client_sender) =
        rt.block_on(async { setup_network(&server_addr, &player_name).await.unwrap() });
    App::new()
        .insert_resource(ClearColor(Color::hex("#87CEEB").unwrap()))
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_state::<AppState>()
        .insert_resource(GameState::new(player_name))
        .insert_resource(NetworkReceiver(network_receiver))
        .insert_resource(NetworkSender(client_sender))
        .add_startup_system(render::setup_3d)
        .insert_resource(input::MovementTimer(Timer::from_seconds(
            0.08,
            TimerMode::Repeating,
        )))
        .add_startup_system(ui::setup_ui.after(render::setup_3d))
        .add_system(network::handle_network_messages)
        .add_system(input::player_input)
        .add_system(render::update_player_positions)
        .add_system(render::render_map.in_schedule(OnEnter(AppState::RenderMap)))
        .add_system(ui::update_minimap)
        .add_system(render::render_walls)
        .add_system(ui::update_fps_text)
        .insert_resource(MouseSensitivity(0.005))
        .insert_resource(PlayerRotation::default())
        .add_system(input::player_look)
        .add_startup_system(camera::setup_fps_camera)
        .insert_resource(CursorState { captured: true })
        .add_system(input::toggle_cursor_capture)
        .add_system(ui::game_over_screen.in_schedule(OnEnter(AppState::GameOver)))
        .add_system(ui::display_death_screen)
        .add_system(player::update_bullets)
        // .add_system(render::update_visibility)
       .add_system(network::disconnected)
        .run();
    Ok(())
}
