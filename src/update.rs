use crate::components::*;
use crate::constants::*;
use crate::troop_utilities::*;

use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;
use rand::Rng;

#[derive(Default)]
pub(crate) struct StateEntryDelay {
    last_state: Option<TurnState>,
    // One-shot timer used to pause only once when a state is entered.
    timer: Option<Timer>,
}

impl StateEntryDelay {
    // Reset the delay whenever the turn state changes.
    fn on_state_change(&mut self, state: TurnState) {
        if self.last_state != Some(state) {
            self.last_state = Some(state);
            self.timer = None;
        }
    }

    // Non-blocking wait: returns true once the delay has elapsed.
    fn wait(&mut self, time: &Time, seconds: f32) -> bool {
        if seconds <= 0.0 {
            return true;
        }
        match self.timer.as_mut() {
            Some(timer) => {
                // Tick the timer until it finishes; stays finished until state changes.
                timer.tick(time.delta());
                timer.is_finished()
            }
            None => {
                // First call in this state: start the timer and pause this frame.
                self.timer = Some(Timer::from_seconds(seconds, TimerMode::Once));
                false
            }
        }
    }
}

#[derive(Default, PartialEq, Eq)]
enum EnemyAttackPhase {
    #[default]
    Idle,
    Windup,
    Cooldown,
}

#[derive(Default)]
pub(crate) struct EnemyAttackDelay {
    phase: EnemyAttackPhase,
    // Timer for the current enemy attack phase (windup or cooldown).
    timer: Option<Timer>,
}

impl EnemyAttackDelay {
    // Clear attack timing when leaving the enemy attack state.
    fn reset(&mut self) {
        self.phase = EnemyAttackPhase::Idle;
        self.timer = None;
    }

    // Non-blocking wait used by the enemy attack phases.
    fn wait(&mut self, time: &Time, seconds: f32) -> bool {
        if seconds <= 0.0 {
            return true;
        }
        match self.timer.as_mut() {
            Some(timer) => {
                // Tick current phase timer until done.
                timer.tick(time.delta());
                if timer.is_finished() {
                    self.timer = None;
                    true
                } else {
                    false
                }
            }
            None => {
                // Start a new phase timer and pause this frame.
                self.timer = Some(Timer::from_seconds(seconds, TimerMode::Once));
                false
            }
        }
    }
}

pub(crate) fn update_game_logic(
    keys: Res<ButtonInput<KeyCode>>,
    commands: Commands,
    mut query: Query<&mut GlobalTurnState>,
    time: Res<Time>,
    mut state_entry_delay: Local<StateEntryDelay>,
    mut enemy_attack_delay: Local<EnemyAttackDelay>,
    asset_server: Res<AssetServer>,
    texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut colorstate: Query<&mut RoundColorState>,
    spectrum_q: Query<Entity, With<SpectrumElement>>,
    mut tilemap_set: ParamSet<(
        Query<
            (
                &TilemapSize,
                &TilemapGridSize,
                &TilemapTileSize,
                &TilemapType,
                &TilemapAnchor,
            ),
            With<PlayZoneTilemap>,
        >,
        Query<(&TileStorage, &TilemapSize), With<PlayZoneTilemap>>,
    )>,
    player_q2: Query<&mut Sprite, With<Player>>,
    enemy_q: Query<&mut Sprite, (With<Enemy>, Without<Player>)>,
    mut enemy_pos_q: Query<(Entity, &mut TilePos), (With<Enemy>, Without<Player>)>,
    mut player_query: Query<(Entity, &mut TilePos), (With<Player>, Without<Enemy>)>,
    mut tile_query: Query<&mut TileColor>,
) {
    // Retrieve the global turn state.
    // This component dictates what action is done next.
    // Enemy attack, Player attack, etc.
    for mut turn_state_entity in query.iter_mut() {
        // Track state transitions so per-state delays only run once.
        state_entry_delay.on_state_change(turn_state_entity.turn_state);
        // Reset enemy attack timing when we're not in the enemy attack state.
        if turn_state_entity.turn_state != TurnState::AttackEnemy {
            enemy_attack_delay.reset();
        }

        match turn_state_entity.turn_state {
            TurnState::ColorPick => {
                color_pick_update(commands, asset_server, &mut colorstate, &spectrum_q);
                
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
                let mut spawn_pos = None;
                for _ in 0..8 {
                    let x = rng.random_range(0..15) as u32;
                    let y = rng.random_range(13..15) as u32;
                    let occupied = enemy_pos_q
                        .iter_mut()
                        .any(|(_, pos)| pos.x == x && pos.y == y);
                    if !occupied {
                        spawn_pos = Some((x, y));
                        break;
                    }
                }

                if let Some((x, y)) = spawn_pos {
                    spawn_enemy(x, y, commands, asset_server, texture_atlas_layouts);
                }
                turn_state_entity.modify_state(TurnState::MovePlayer);
                return;
            }
            TurnState::MovePlayer => {
                // Short pause before allowing player movement.
                if !state_entry_delay.wait(&time, MOVE_DELAY_SECONDS) {
                    return;
                }
                let tilemap_q = tilemap_set.p0();
                let Ok((map_size, _grid_size, _tile_size, _map_type, _anchor)) =
                    tilemap_q.single()
                else {
                    return;
                };

                for (_, mut tile_pos) in player_query.iter_mut() {
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
                        &mut player_query,
                        tilemap_set.p1(),
                        &mut tile_query,
                        &colorstate,
                    );
                    despawn_enemies_on_matching_tile_color(
                        commands,
                        &mut enemy_pos_q,
                        tilemap_set.p1(),
                        &mut tile_query,
                        &colorstate,
                    );
                    turn_state_entity.modify_state(TurnState::MoveEnemy);
                } else if keys.just_pressed(KeyCode::KeyW) {
                    color_player_neighbors(
                        AttackPattern::Sides,
                        &mut player_query,
                        tilemap_set.p1(),
                        &mut tile_query,
                        &colorstate,
                    );
                    despawn_enemies_on_matching_tile_color(
                        commands,
                        &mut enemy_pos_q,
                        tilemap_set.p1(),
                        &mut tile_query,
                        &colorstate,
                    );
                    turn_state_entity.modify_state(TurnState::MoveEnemy);
                } else if keys.just_pressed(KeyCode::KeyE) {
                    color_player_neighbors(
                        AttackPattern::Around,
                        &mut player_query,
                        tilemap_set.p1(),
                        &mut tile_query,
                        &colorstate,
                    );
                    despawn_enemies_on_matching_tile_color(
                        commands,
                        &mut enemy_pos_q,
                        tilemap_set.p1(),
                        &mut tile_query,
                        &colorstate,
                    );
                    turn_state_entity.modify_state(TurnState::MoveEnemy);
                } else if keys.just_pressed(KeyCode::KeyR) {
                    color_player_neighbors(
                        AttackPattern::Ultimate,
                        &mut player_query,
                        tilemap_set.p1(),
                        &mut tile_query,
                        &colorstate,
                    );
                    despawn_enemies_on_matching_tile_color(
                        commands,
                        &mut enemy_pos_q,
                        tilemap_set.p1(),
                        &mut tile_query,
                        &colorstate,
                    );
                    turn_state_entity.modify_state(TurnState::MoveEnemy);
                }

                return;
            }
            TurnState::MoveEnemy => {
                // Short pause before enemy movement.
                if !state_entry_delay.wait(&time, MOVE_DELAY_SECONDS) {
                    return;
                }
                for (_, mut tile_pos) in enemy_pos_q.iter_mut() {
                    tile_pos.y = tile_pos.y.saturating_sub(1);
                }
                turn_state_entity.modify_state(TurnState::AttackEnemy);
                return;
            }
            TurnState::AttackEnemy => {
                match enemy_attack_delay.phase {
                    EnemyAttackPhase::Idle => {
                        // Start the windup phase on first entry.
                        enemy_attack_delay.phase = EnemyAttackPhase::Windup;
                        enemy_attack_delay.timer = None;
                        return;
                    }
                    EnemyAttackPhase::Windup => {
                        // Windup delay before the enemy attack happens.
                        if !enemy_attack_delay.wait(&time, ENEMY_ATTACK_WINDUP_SECONDS) {
                            return;
                        }
                        color_enemy_neighbors(
                            &mut enemy_pos_q,
                            tilemap_set.p1(),
                            &mut tile_query,
                            &colorstate,
                        );
                        despawn_player_on_matching_tile_color(
                            commands,
                            &mut player_query,
                            tilemap_set.p1(),
                            &mut tile_query,
                            &colorstate,
                        );
                        // After attacking, enter cooldown phase.
                        enemy_attack_delay.phase = EnemyAttackPhase::Cooldown;
                        enemy_attack_delay.timer = None;
                        return;
                    }
                    EnemyAttackPhase::Cooldown => {
                        // Cooldown delay after the attack, before the next turn.
                        if !enemy_attack_delay.wait(&time, ENEMY_ATTACK_COOLDOWN_SECONDS) {
                            return;
                        }
                        enemy_attack_delay.reset();
                        turn_state_entity.modify_state(TurnState::ColorPick);
                        return;
                    }
                }
            }
        }
    }
}

fn color_pick_update(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    color_state_q: &mut Query<&mut RoundColorState>,
    spectrum_q: &Query<Entity, With<SpectrumElement>>,
) {
    let offset_up = 178.0f32;
    let offset_right = 61.0f32;

    // Despawn the color sprites from before
    // There might be smarter ways to update existing sprites though
    for entity in spectrum_q.iter() {
        commands.entity(entity).despawn();
    }

    for mut color_state in color_state_q.iter_mut() {
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
