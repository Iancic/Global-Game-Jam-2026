use crate::components::*;
use crate::constants::*;
use crate::troop_utilities::*;

use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;
use rand::Rng;

pub fn update_game_logic(
    keys: Res<ButtonInput<KeyCode>>,
    commands: Commands,
    mut query: Query<&mut GlobalTurnState>,
    asset_server: Res<AssetServer>,
    texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    colorstate: Query<&mut RoundColorState>,
    tilemap_q: Query<
        (
            &TilemapSize,
            &TilemapGridSize,
            &TilemapTileSize,
            &TilemapType,
            &TilemapAnchor,
        ),
        With<PlayZoneTilemap>,
    >,
    player_q2: Query<&mut Sprite, With<Player>>,
    enemy_q: Query<&mut Sprite, (With<Enemy>, Without<Player>)>,
    mut player_query: Query<&mut TilePos, With<Player>>,
    tilemap_query: Query<(&TileStorage, &TilemapSize), With<PlayZoneTilemap>>,
    tile_query: Query<&mut TileColor>,
) {
    // Retrieve the global turn state.
    // This component dictates what action is done next.
    // Enemy attack, Player attack, etc.
    for mut turn_state_entity in query.iter_mut() {
        match turn_state_entity.turn_state {
            TurnState::ColorPick => {
                color_pick_update(commands, asset_server, colorstate);
                
                turn_state_entity.modify_state(TurnState::PlayerChange);
                return;
            }
            TurnState::PlayerChange => {
                update_player_color(player_q2, &colorstate);
                update_enemy_color(enemy_q, &colorstate);

                turn_state_entity.modify_state(TurnState::EnemySpawn);
                return;
            }
            TurnState::EnemySpawn => {
                let mut rng = rand::rng();
                spawn_enemy(rng.random_range(0..15) as u32, rng.random_range(13..15) as u32, commands, asset_server, texture_atlas_layouts);
                turn_state_entity.modify_state(TurnState::MovePlayer);
                return;
            }
            TurnState::MovePlayer => {
                let Ok((map_size, _grid_size, _tile_size, _map_type, _anchor)) = tilemap_q.single()
                else {
                    return;
                };

                for mut tile_pos in player_query.iter_mut() {
                    let mut next = *tile_pos;

                    if keys.just_pressed(KeyCode::ArrowUp) {
                        next.y += 1;
                        turn_state_entity.modify_state(TurnState::AttackPlayer);
                    }
                    if keys.just_pressed(KeyCode::ArrowDown) {
                        next.y = next.y.saturating_sub(1);
                        turn_state_entity.modify_state(TurnState::AttackPlayer);
                    }
                    if keys.just_pressed(KeyCode::ArrowRight) {
                        next.x += 1;
                        turn_state_entity.modify_state(TurnState::AttackPlayer);
                    }
                    if keys.just_pressed(KeyCode::ArrowLeft) {
                        next.x = next.x.saturating_sub(1);
                        turn_state_entity.modify_state(TurnState::AttackPlayer);
                    }

                    let max_x = (map_size.x - 1) as i32;
                    let max_y = (map_size.y - 1) as i32;

                    next.x = (next.x as i32).clamp(0, max_x) as u32;
                    next.y = (next.y as i32).clamp(0, max_y) as u32;

                    *tile_pos = next;
                }
                return;
            }
            TurnState::AttackPlayer => {
                // Attack with player
                if keys.just_pressed(KeyCode::KeyQ) {
                    color_player_neighbors(
                        AttackPattern::Diagonal,
                        player_query,
                        tilemap_query,
                        tile_query,
                    );
                    turn_state_entity.modify_state(TurnState::MoveEnemy);
                } else if keys.just_pressed(KeyCode::KeyW) {
                    color_player_neighbors(
                        AttackPattern::Sides,
                        player_query,
                        tilemap_query,
                        tile_query,
                    );
                    turn_state_entity.modify_state(TurnState::MoveEnemy);
                } else if keys.just_pressed(KeyCode::KeyE) {
                    color_player_neighbors(
                        AttackPattern::Around,
                        player_query,
                        tilemap_query,
                        tile_query,
                    );
                    turn_state_entity.modify_state(TurnState::MoveEnemy);
                } else if keys.just_pressed(KeyCode::KeyR) {
                    color_player_neighbors(
                        AttackPattern::Ultimate,
                        player_query,
                        tilemap_query,
                        tile_query,
                    );
                    turn_state_entity.modify_state(TurnState::MoveEnemy);
                }

                return;
            }
            TurnState::MoveEnemy => {
                turn_state_entity.modify_state(TurnState::AttackEnemy);
                return;
            }
            TurnState::AttackEnemy => {
                turn_state_entity.modify_state(TurnState::ColorPick);
                return;
            }
        }
    }
}

fn color_pick_update(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut color_state: Query<&mut RoundColorState>,
) {
    let offset_up = 178.0f32;
    let offset_right = 61.0f32;

    for mut color_state in color_state.iter_mut() {
        color_state.asign_random_color();

        // Load Spectrum Sprites
        let texture_handle_blue: bevy::prelude::Handle<Image> =
            asset_server.load("Spectrum-Blue-Sphere.png");
        let texture_handle_red: bevy::prelude::Handle<Image> =
            asset_server.load("Spectrum-Red-Sphere.png");
        let texture_handle_green: bevy::prelude::Handle<Image> =
            asset_server.load("Spectrum-Green-Sphere.png");
        let texture_handle_middle: bevy::prelude::Handle<Image> = asset_server.load("TEXT.png");
        let texture_handle_arrow: bevy::prelude::Handle<Image> =
            asset_server.load("Spectrum-Masked.png");

        if color_state.index != 0
        // Red gets masked
        {
            // The colors in the spectrum
            commands.spawn((
                Sprite {
                    image: texture_handle_red,
                    color: Color::srgba(1.0, 0.0, 0.0, 0.7),
                    ..default()
                },
                Transform::from_xyz(-offset_right, offset_up, LAYER_UI as f32),
                SpectrumElement,
            ));
        }
        if color_state.index != 1 {
            commands.spawn((
                Sprite {
                    image: texture_handle_green,
                    color: Color::srgba(0.0, 1.0, 0.0, 0.7),
                    ..default()
                },
                Transform::from_xyz(-offset_right, offset_up, LAYER_UI as f32),
                SpectrumElement,
            ));
        }
        if color_state.index != 2 {
            commands.spawn((
                Sprite {
                    image: texture_handle_blue.clone(),
                    color: Color::srgba(0.0, 0.0, 1.0, 0.7),
                    ..default()
                },
                Transform::from_xyz(-offset_right, offset_up, LAYER_UI as f32),
                SpectrumElement,
            ));
        }

        // Arrow that shows what gets masked
        let mut r = 0.0f32;
        let mut g = 0.0f32;
        let mut b = 0.0f32;
        let mut r2 = 1.0f32;
        let mut g2 = 1.0f32;
        let mut b2 = 1.0f32;

        if color_state.index == 0 {
            r = 1.0f32;
            r2 = 0.0f32;
        } else if color_state.index == 1 {
            g = 1.0f32;
            g2 = 0.0f32;
        } else {
            b = 1.0f32;
            b2 = 0.0f32;
        }

        commands.spawn((
            Sprite {
                image: texture_handle_middle.clone(),
                color: Color::srgba(r, g, b, 1.),
                ..default()
            },
            Transform::from_xyz(
                offset_right - 55 as f32,
                offset_up - 8 as f32,
                LAYER_UI as f32,
            ),
            SpectrumElement,
        ));

        // Round Resulting color
        commands.spawn((
            Transform::from_xyz(offset_right, offset_up - 8 as f32, LAYER_UI as f32),
            Sprite {
                image: texture_handle_arrow.clone(),
                color: Color::srgba(r2, g2, b2, 1.),
                ..default()
            },
            SpectrumElement,
        ));
    }
}
