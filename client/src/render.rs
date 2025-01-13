use core::panic;

use bevy::prelude::*;
use crate::game_state::GameState;
use crate::player::{Player, OtherPlayer};
use crate::camera::{PlayerCamera, PlayerRotation};


pub fn setup_3d(mut commands: Commands) {
    // Ajout d'une lumière directionnelle
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: false,
            illuminance: 10000.0,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4)),
        ..default()
    });
    // Ajout d'une lumière ambiante
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.7,
    });
}


pub fn render_map(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    wall_query: Query<Entity, With<Wall>>,
) {
    if let Some(map) = &game_state.map {
        if !game_state.map_rendered {
            // Supprimer les anciens murs
            for entity in wall_query.iter() {
                commands.entity(entity).despawn();
            }

            let wall_mesh = meshes.add(Mesh::from(shape::Box::new(1.0, 3.0, 1.0)));
            let wall_material = materials.add(StandardMaterial {
                base_color: Color::rgb(0.8, 0.7, 0.6),
                ..default()
            });

            // Créer les nouveaux murs
            for (y, row) in map.cells.iter().enumerate() {
                for (x, &is_wall) in row.iter().enumerate() {
                    if is_wall {
                        commands.spawn((
                            PbrBundle {
                                mesh: wall_mesh.clone(),
                                material: wall_material.clone(),
                                transform: Transform::from_xyz(x as f32, 1.5, y as f32),
                                ..default()
                            },
                            Wall,
                            Renderable,
                        ));
                    }
                }
            }

            // Ajouter le sol
            let floor_size = map.map_width as f32;
            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Plane { size: floor_size, subdivisions: 1 })),
                    material: materials.add(Color::rgb(0.3, 0.3, 0.3).into()),
                    transform: Transform::from_xyz(floor_size / 2.0 - 0.5, 0.0, floor_size / 2.0 - 0.5),
                    ..default()
                },
                Renderable,
            ));

            game_state.map_rendered = true;
        }
    }
}

#[derive(Component)]
pub struct Wall;

// Ajoutez ce composant
#[derive(Component)]
pub struct Walls {
    transforms: Vec<Transform>,
}

// Ajoutez ce système pour rendre les murs
pub fn render_walls(
    walls_query: Query<&Walls>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for walls in walls_query.iter() {
        for wall_transform in &walls.transforms {
            commands.spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Box::new(1.0, 3.0, 1.0))),
                material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                transform: *wall_transform,
                ..default()
            });
        }
    }
}

#[derive(Component)]
pub struct WeaponModel;

pub fn update_player_positions(

    mut commands: Commands,
    game_state: Res<GameState>,
    player_rotation: Res<PlayerRotation>,
    mut query_set: ParamSet<(
        Query<(Entity, &mut Transform), With<Player>>,
        Query<(Entity, &mut Transform, &OtherPlayer)>,
        Query<(Entity, &mut Transform), With<PlayerCamera>>,
        Query<(Entity, &mut Transform), With<WeaponModel>>,
    )>,
    asset_server: Res<AssetServer>,
) {
    if game_state.players.len()==0{
        panic!()
    }
    if let Some(player_id) = &game_state.player_id {
        if let Some(&(position_x, position_y, _, is_alive)) = game_state.players.get(player_id) {
            if is_alive {
                let eye_height = 1.6;
                let forward_offset = 0.01;
                let mut camera_query = query_set.p2();
                let new_camera_position = Vec3::new(
                    position_x - forward_offset * player_rotation.yaw.sin(),
                    eye_height,
                    position_y - forward_offset * player_rotation.yaw.cos()
                );
                let new_camera_rotation = Quat::from_euler(EulerRot::YXZ, player_rotation.yaw, player_rotation.pitch, 0.0);

                if let Ok((_entity, mut camera_transform)) = camera_query.get_single_mut() {
                    camera_transform.translation = new_camera_position;
                    camera_transform.rotation = new_camera_rotation;
                } else {
                    let _ = commands.spawn((
                        Camera3dBundle {
                            transform: Transform::from_translation(new_camera_position)
                                .with_rotation(new_camera_rotation),
                            ..default()
                        },
                        PlayerCamera,
                    )).id();
                }

                // Gestion du modèle de l'arme
                let weapon_offset = Vec3::new(0.04, -0.15, -0.3); // Pousser l'arme plus vers le centre
                let mut weapon_query = query_set.p3();

                if let Ok((_, mut weapon_transform)) = weapon_query.get_single_mut() {
                    weapon_transform.translation = new_camera_position + new_camera_rotation * weapon_offset;
                    weapon_transform.rotation = new_camera_rotation;
                } else {
                    commands.spawn((
                        SceneBundle {
                            scene: asset_server.load("models/player/ak.glb#Scene0"),
                            transform: Transform {
                                translation: new_camera_position + new_camera_rotation * weapon_offset,
                                rotation: new_camera_rotation,
                                scale: Vec3::splat(0.15), // Réduire la taille de l'arme
                            },
                            ..default()
                        },
                        WeaponModel,
                    ));
                }

                // Remove player model for current player
                let mut player_query = query_set.p0();
                if let Ok((player_entity, _)) = player_query.get_single_mut() {
                    commands.entity(player_entity).despawn_recursive();
                }
            }
        }
    }

    // Update other players
    let mut other_player_query = query_set.p1();
    for (entity, mut transform, other_player) in other_player_query.iter_mut() {
        if let Some(&(x, y, rotation, is_alive)) = game_state.players.get(&other_player.name) {
            if is_alive {
                transform.translation = Vec3::new(x, 0.0, y);
                transform.rotation = Quat::from_rotation_y(rotation);
            } else {
                commands.entity(entity).despawn_recursive();
            }
        } else {
            commands.entity(entity).despawn_recursive();
        }
    }

    // Add new other players
    for (name, &(x, y, rotation, is_alive)) in game_state.players.iter() {
        if Some(name) != game_state.player_id.as_ref() && is_alive {
            let other_player_query = query_set.p1();
            if !other_player_query.iter().any(|(_, _, op)| &op.name == name) {
                commands.spawn((
                    SceneBundle {
                        scene: asset_server.load("models/player/Soldier.glb#Scene0"),
                        transform: Transform::from_xyz(x, 0.0, y)
                            .with_rotation(Quat::from_rotation_y(rotation))
                            .with_scale(Vec3::splat(0.06)),
                        ..default()
                    },
                    OtherPlayer { name: name.clone() },
                ));
            }
        }
    }
}

#[derive(Component)]
pub struct Renderable;

pub fn update_visibility(
    mut renderable_query: Query<(&mut Visibility, &GlobalTransform), With<Renderable>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    if let Ok((_camera, camera_transform)) = camera_query.get_single() {
        let camera_position = camera_transform.translation();
        let forward = camera_transform.forward();

        for (mut visibility, transform) in renderable_query.iter_mut() {
            let to_object = transform.translation() - camera_position;
            let distance = to_object.length();
            
            // Ajustez ces valeurs selon vos besoins
            let render_distance = 30.0;
            let fov_cos = 0.5; // Approximativement 60 degrés de champ de vision
            
            if distance <= render_distance && to_object.normalize().dot(forward) > fov_cos {
                *visibility = Visibility::Inherited;
            } else {
                *visibility = Visibility::Hidden;
            }
        }
    }
}