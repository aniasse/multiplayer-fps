use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use bevy::window::CursorGrabMode;
use crate::game_state::GameState;
use crate::camera::{MouseSensitivity, PlayerCamera, PlayerRotation};
use crate::network::NetworkSender;
use crate::messages::ClientMessage;
use crate::player::{Bullet, PLAYER_SPEED, SHOOT_COOLDOWN};

#[derive(Resource)]
pub struct CursorState {
    pub captured: bool,
}

#[derive(Resource)]
pub struct MovementTimer(pub Timer);

pub fn player_input(
    keyboard_input: Res<Input<KeyCode>>,
    mouse_input: Res<Input<MouseButton>>,
    windows: Query<&Window>,
    network_sender: Res<NetworkSender>,
    mut game_state: ResMut<GameState>,
    time: Res<Time>,
    mut timer: ResMut<MovementTimer>,
    player_rotation: Res<PlayerRotation>,
    mut query_set: ParamSet<(
        Query<&Transform, With<PlayerCamera>>,
        Query<&mut Transform, With<crate::render::WeaponModel>>,
    )>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let _ = windows;
    timer.0.tick(time.delta());

    if mouse_input.just_pressed(MouseButton::Left) {
        let current_time = time.elapsed_seconds();
        if current_time - game_state.last_shoot_time >= SHOOT_COOLDOWN {
            game_state.last_shoot_time = current_time;

            // Obtenir la transformation de la caméra
            let camera_transform_query = query_set.p0();
            let camera_transform = camera_transform_query
                .get_single()
                .expect("Player camera not found");
            let shoot_direction = camera_transform.forward();

            let shoot_message = ClientMessage::Shoot { direction: (shoot_direction.x, shoot_direction.z) };
            if let Err(e) = network_sender.0.send(shoot_message) {
                panic!("Failed to send shoot message: {}", e);
            }
            println!("Player shot in direction: {:?}", (shoot_direction.x, shoot_direction.z));

            // Obtenir et modifier la transformation du modèle d'arme
            let mut weapon_transform_query = query_set.p1();
            let mut weapon_transform = weapon_transform_query
                .get_single_mut()
                .expect("Weapon model not found");

            let weapon_tip = weapon_transform.translation + shoot_direction * 0.5;
            let bullet_rotation = Quat::from_rotation_arc(Vec3::Z, shoot_direction);

            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Capsule { 
                        radius: 0.009,
                        rings: 0,
                        depth: 0.2,
                        latitudes: 8,
                        longitudes: 18,
                        uv_profile: shape::CapsuleUvProfile::Uniform,
                    })),
                    material: materials.add(Color::RED.into()),
                    transform: Transform::from_translation(weapon_tip)
                        .with_rotation(bullet_rotation * Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
                    ..default()
                },
                Bullet {
                    lifetime: Timer::from_seconds(0.2, TimerMode::Once),
                },
            ));

            // Effet de secousse de l'arme
            let shake_amount = 0.008;
            let shake_offset = Vec3::new(
                rand::random::<f32>() * shake_amount,
                rand::random::<f32>() * shake_amount,
                0.0,
            );
            weapon_transform.translation += shake_offset;
        }
    }

    if !timer.0.finished() {
        return;
    }

    let mut direction = Vec3::ZERO;
    if keyboard_input.pressed(KeyCode::S) {//arriere
        direction += Vec3::new(player_rotation.yaw.sin(), 0.0, player_rotation.yaw.cos());
    }
    if keyboard_input.pressed(KeyCode::Z) {// avancer
        direction += Vec3::new(-player_rotation.yaw.sin(), 0.0, -player_rotation.yaw.cos());
    }
    if keyboard_input.pressed(KeyCode::Q) {//gauche 
        direction += Vec3::new(-player_rotation.yaw.cos(), 0.0, player_rotation.yaw.sin());
    }
    if keyboard_input.pressed(KeyCode::D) {//droite
        direction += Vec3::new(player_rotation.yaw.cos(), 0.0, -player_rotation.yaw.sin());
    }

    if direction != Vec3::ZERO {
        direction = direction.normalize();
        let move_message = ClientMessage::Move { direction: (direction.x, direction.z) };
        if let Err(e) = network_sender.0.send(move_message) {
            panic!("Failed to send move message: {}", e);
        }
        let player_id = game_state.player_id.clone();
        if let Some(player_id) = player_id {
            if let Some(position) = game_state.players.get_mut(&player_id) {
                position.0 += direction.x * PLAYER_SPEED;
                position.1 += direction.z * PLAYER_SPEED;
            }
        }
    }
}

pub fn player_look(
    mut motion_evr: EventReader<MouseMotion>,
    mut player_rotation: ResMut<PlayerRotation>,
    sensitivity: Res<MouseSensitivity>,
    cursor_state: Res<CursorState>,
) {
    if cursor_state.captured {
        for ev in motion_evr.iter() {
            player_rotation.yaw -= ev.delta.x * sensitivity.0;
            player_rotation.pitch -= ev.delta.y * sensitivity.0;
        }
        player_rotation.pitch = player_rotation.pitch.clamp(-0.7, 0.7);
    }
}

pub fn toggle_cursor_capture(
    keyboard_input: Res<Input<KeyCode>>,
    mut cursor_state: ResMut<CursorState>,
    mut windows: Query<&mut Window>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        cursor_state.captured = !cursor_state.captured;
        if let Ok(mut window) = windows.get_single_mut() {
            if cursor_state.captured {
                window.cursor.grab_mode = CursorGrabMode::Locked;
                window.cursor.visible = false;
            } else {
                window.cursor.grab_mode = CursorGrabMode::None;
                window.cursor.visible = true;
            }
        }
    }
}


