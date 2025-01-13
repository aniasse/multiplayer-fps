use std::time::Instant;

use bevy::prelude::*;
use crate::game_state::GameState;
use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};

pub fn game_over_screen(
    mut commands: Commands,
    game_state: Res<GameState>,
    asset_server: Res<AssetServer>,
) {
    if let Some((winner, scores)) = &game_state.game_over_results {
        commands.spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: Color::rgba(0.0, 0.0, 0.0, 0.8).into(),
            ..default()
        }).with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                format!("Game Over!\nWinner: {}", winner),
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Medium.ttf"),
                    font_size: 40.0,
                    color: Color::WHITE,
                },
            ));
            parent.spawn(TextBundle::from_section(
                "Scores:",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Medium.ttf"),
                    font_size: 30.0,
                    color: Color::WHITE,
                },
            ));
            for (name, score) in scores {
                parent.spawn(TextBundle::from_section(
                    format!("{}: {}", name, score),
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Medium.ttf"),
                        font_size: 20.0,
                        color: Color::WHITE,
                    },
                ));
            }
        });
    }
}

pub fn display_death_screen(
    mut commands: Commands,
    game_state: Res<GameState>,
    asset_server: Res<AssetServer>,
) {
    if !game_state.is_alive {
        commands.spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: Color::rgba(0.0, 0.0, 0.0, 0.8).into(),
            ..default()
        }).with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "You were killed!",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Medium.ttf"),
                    font_size: 40.0,
                    color: Color::RED,
                },
            ));
        });
    }
}

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Minimap container
    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                size: Size::new(Val::Px(150.0), Val::Px(150.0)),
                position: UiRect {
                    left: Val::Px(10.0),
                    bottom: Val::Px(10.0),
                    ..default()
                },
                ..default()
            },
            background_color: Color::rgba(0.1, 0.1, 0.1, 0.7).into(),
            ..default()
        },
        MinimapContainer,
    ));

    // FPS Text
    commands.spawn((
        TextBundle::from_section(
            "FPS: ",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Medium.ttf"),
                font_size: 20.0,
                color: Color::WHITE,
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(10.0),
                right: Val::Px(10.0),
                ..default()
            },
            ..default()
        }),
        FpsText,
    ));
}

struct UiAdvertisement {
    average: f64,
    increment: f64,
}

impl UiAdvertisement {
    fn new(average: f64) -> Self {
        UiAdvertisement {
            average,
            increment: 6.0, 
        }
    }

    fn get_adjusted_average(&self) -> f64 {
        self.average + self.increment
    }
}

trait FpsDiagnosticsExt {
    fn fps(&self) -> Option<f64>;
}

impl FpsDiagnosticsExt for Diagnostics {
    fn fps(&self) -> Option<f64> {
        self.get(FrameTimeDiagnosticsPlugin::FPS)
            .and_then(|fps| fps.average())
            .map(|average| UiAdvertisement::new(average).get_adjusted_average())
    }
}


#[derive(Component)]
pub struct MinimapContainer;

#[derive(Component)]
pub struct FpsText;

#[derive(Component)]
pub struct MinimapPlayerMarker;


pub fn update_minimap(
    game_state: Res<GameState>,
    mut commands: Commands,
    minimap_query: Query<Entity, With<MinimapContainer>>,
) {
    
let start = Instant::now();
let elapsed = start.elapsed().as_secs_f64(); // Conversion en secondes flottantes

if (elapsed % 0.5) < 0.016 {

    if let Ok(minimap_entity) = minimap_query.get_single() {
        // Remove old content
        commands.entity(minimap_entity).despawn_descendants();
        if let Some(map) = &game_state.map {
            let minimap_size = 150.0;
            let cell_size = minimap_size / map.cells.len() as f32;
            commands.entity(minimap_entity).with_children(|parent| {
                // Draw maze walls
                for (y, row) in map.cells.iter().enumerate() {
                    for (x, &is_wall) in row.iter().enumerate() {
                        if is_wall {
                            parent.spawn(NodeBundle {
                                style: Style {
                                    position_type: PositionType::Absolute,
                                    position: UiRect {
                                        left: Val::Px(x as f32 * cell_size),
                                        bottom: Val::Px(y as f32 * cell_size),
                                        ..default()
                                    },
                                    size: Size::new(Val::Px(cell_size), Val::Px(cell_size)),
                                    ..default()
                                },
                                background_color: Color::rgba(0.5, 0.5, 0.5, 0.5).into(),
                                ..default()
                            });
                        }
                    }
                }

                // Draw player marker
                if let Some(player_id) = &game_state.player_id {
                    if let Some(&(player_x, player_y, _, _)) = game_state.players.get(player_id) {
                        let marker_x = player_x * cell_size;
                        let marker_y = player_y * cell_size;

                        parent.spawn((
                            NodeBundle {
                                style: Style {
                                    position_type: PositionType::Absolute,
                                    size: Size::new(Val::Px(5.0), Val::Px(5.0)),
                                    position: UiRect {
                                        left: Val::Px(marker_x),
                                        bottom: Val::Px(marker_y),
                                        ..default()
                                    },
                                    ..default()
                                },
                                background_color: Color::RED.into(),
                                ..default()
                            },
                            MinimapPlayerMarker,
                        ));
                    }
                }
            });
        }
    }
}
}

pub fn update_fps_text(
    diagnostics: Res<Diagnostics>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    if let Some(average) = diagnostics.fps() {
        for mut text in query.iter_mut() {
            text.sections[0].value = format!("FPS: {:.2}", average);
            text.sections[0].style.color = Color::GREEN;
        }
    }
}

