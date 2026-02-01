use crate::components::*;
use crate::constants::*;
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;

pub fn spawn_enemy(
    tile_pos_x: u32,
    tile_pos_y: u32,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("Enemy.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 3, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_indices = AnimationIndices { first: 0, last: 2 };

    commands.spawn((
        Troop,
        Enemy,
        TilePos {
            x: tile_pos_x,
            y: tile_pos_y,
        },
        Transform::from_xyz(0., 0., LAYER_PLAYER as f32),
        Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_indices.first,
            },
        ),
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));
}

// Credit: snapping idea to center in world from Codex 5.2
pub fn update_player_troop_to_tilemap(
    mut player_q: Query<(&TilePos, &mut Transform), (With<Player>, Without<Troop>)>, // All entities with transform, player and position
    tilemap_q: Query<
        (
            &TilemapSize,
            &TilemapGridSize,
            &TilemapTileSize,
            &TilemapType,
            &TilemapAnchor,
        ),
        With<PlayZoneTilemap>,
    >, // Necessary for center in world
) {
    // Tries to query for an entity with the data required to snap.
    // If there is none we have nothing to store, doesn't pass the Ok check move on.
    let Ok((map_size, grid_size, tile_size, map_type, anchor)) = tilemap_q.single() else {
        println!("No tilemap.");
        return;
    };

    for (tile_pos, mut transform) in player_q.iter_mut() {
        // Retrieve center from the tilemap:
        let center = tile_pos.center_in_world(map_size, grid_size, tile_size, map_type, anchor);

        // Change the transform from the player
        transform.translation.x = center.x;
        transform.translation.y = center.y;
        transform.translation.z = LAYER_PLAYER as f32;
    }
}

pub fn update_enemy_troop_to_tilemap(
    mut enemy_q: Query<(&TilePos, &mut Transform), (With<Troop>, Without<Player>)>, // All entities with transform, enemy and position
    tilemap_q: Query<
        (
            &TilemapSize,
            &TilemapGridSize,
            &TilemapTileSize,
            &TilemapType,
            &TilemapAnchor,
        ),
        With<PlayZoneTilemap>,
    >, // Necessary for center in world
) {
    // Tries to query for an entity with the data required to snap.
    // If there is none we have nothing to store, doesn't pass the Ok check move on.
    let Ok((map_size, grid_size, tile_size, map_type, anchor)) = tilemap_q.single() else {
        println!("No tilemap.");
        return;
    };

    for (tile_pos, mut transform) in enemy_q.iter_mut() {
        // Retrieve center from the tilemap:
        let center = tile_pos.center_in_world(map_size, grid_size, tile_size, map_type, anchor);

        // Change the transform from the player
        transform.translation.x = center.x;
        transform.translation.y = center.y;
        transform.translation.z = LAYER_PLAYER as f32;
    }
}

#[derive(Clone, Copy)]
// Avoid long if checks inside the coloring functions for player, q is diagonal, etc
pub enum AttackPattern {
    Diagonal,
    Sides,
    Around,
    Ultimate,
}

fn masked_color_from_index(index: i32) -> Color {
    match index {
        0 => Color::srgba(1.0, 0.0, 0.0, 1.0),
        1 => Color::srgba(0.0, 1.0, 0.0, 1.0),
        2 => Color::srgba(0.0, 0.0, 1.0, 1.0),
        _ => Color::srgba(1.0, 1.0, 1.0, 1.0),
    }
}

fn player_color_from_index(index: i32) -> Color {
    match index {
        0 => Color::srgba(0.0, 1.0, 1.0, 1.0),
        1 => Color::srgba(1.0, 0.0, 1.0, 1.0),
        2 => Color::srgba(1.0, 1.0, 0.0, 1.0),
        _ => Color::srgba(1.0, 1.0, 1.0, 1.0),
    }
}

// Helper function to safely create a tile position with bounds checking
fn try_tile_pos(x: i32, y: i32, map_size: &TilemapSize) -> Option<TilePos> {
    if x >= 0 && y >= 0 && (x as u32) < map_size.x && (y as u32) < map_size.y {
        Some(TilePos {
            x: x as u32,
            y: y as u32,
        })
    } else {
        None
    }
}

// This goes around the player tile and gets the neighbours
// Spits out vector used when calling color player/enemy neighbour
fn gather_neighbors(
    pattern: AttackPattern,
    pos: TilePos,
    map_size: &TilemapSize,
    attack_lenght: i32
) -> Vec<TilePos> {
    let mut neighbors = Vec::new();
    let px = pos.x as i32;
    let py = pos.y as i32;

    // This is where the unit is.
    // From a gameplay perspective looks better on the map.
    if let Some(tile) = try_tile_pos(px, py, map_size) {
        neighbors.push(tile);
    }

    match pattern {
        AttackPattern::Diagonal => {
            for offset in 1..=attack_lenght {
                if let Some(tile) = try_tile_pos(px + offset, py + offset, map_size) {
                    neighbors.push(tile);
                }
                if let Some(tile) = try_tile_pos(px - offset, py - offset, map_size) {
                    neighbors.push(tile);
                }
                if let Some(tile) = try_tile_pos(px - offset, py + offset, map_size) {
                    neighbors.push(tile);
                }
                if let Some(tile) = try_tile_pos(px + offset, py - offset, map_size) {
                    neighbors.push(tile);
                }
            }
        }
        AttackPattern::Sides => {
            for offset in 1..=attack_lenght {
                if let Some(tile) = try_tile_pos(px + offset, py, map_size) {
                    neighbors.push(tile);
                }
                if let Some(tile) = try_tile_pos(px - offset, py, map_size) {
                    neighbors.push(tile);
                }
                if let Some(tile) = try_tile_pos(px, py + offset, map_size) {
                    neighbors.push(tile);
                }
                if let Some(tile) = try_tile_pos(px, py - offset, map_size) {
                    neighbors.push(tile);
                }
            }
        }
        AttackPattern::Around => {
            for offset in 1..=attack_lenght {
                if let Some(tile) = try_tile_pos(px + offset, py + offset, map_size) {
                    neighbors.push(tile);
                }
                if let Some(tile) = try_tile_pos(px - offset, py - offset, map_size) {
                    neighbors.push(tile);
                }
                if let Some(tile) = try_tile_pos(px - offset, py + offset, map_size) {
                    neighbors.push(tile);
                }
                if let Some(tile) = try_tile_pos(px + offset, py - offset, map_size) {
                    neighbors.push(tile);
                }
                if let Some(tile) = try_tile_pos(px + offset, py, map_size) {
                    neighbors.push(tile);
                }
                if let Some(tile) = try_tile_pos(px - offset, py, map_size) {
                    neighbors.push(tile);
                }
                if let Some(tile) = try_tile_pos(px, py + offset, map_size) {
                    neighbors.push(tile);
                }
                if let Some(tile) = try_tile_pos(px, py - offset, map_size) {
                    neighbors.push(tile);
                }
            }
        }
        AttackPattern::Ultimate => {
            for offset in 1..=attack_lenght {
                if let Some(tile) = try_tile_pos(px + offset, py + offset, map_size) {
                    neighbors.push(tile);
                }
                if let Some(tile) = try_tile_pos(px - offset, py - offset, map_size) {
                    neighbors.push(tile);
                }
                if let Some(tile) = try_tile_pos(px - offset, py + offset, map_size) {
                    neighbors.push(tile);
                }
                if let Some(tile) = try_tile_pos(px + offset, py - offset, map_size) {
                    neighbors.push(tile);
                }
                if let Some(tile) = try_tile_pos(px + offset, py, map_size) {
                    neighbors.push(tile);
                }
                if let Some(tile) = try_tile_pos(px - offset, py, map_size) {
                    neighbors.push(tile);
                }
                if let Some(tile) = try_tile_pos(px, py + offset, map_size) {
                    neighbors.push(tile);
                }
                if let Some(tile) = try_tile_pos(px, py - offset, map_size) {
                    neighbors.push(tile);
                }
            }
        }
    }

    neighbors
}

fn apply_tile_color(
    neighbors: Vec<TilePos>,
    storage: &TileStorage,
    tile_q: &mut Query<&mut TileColor>,
    color: Color,
) {
    for tile in neighbors {
        if let Some(tile_entity) = storage.get(&tile) {
            if let Ok(mut tile_color) = tile_q.get_mut(tile_entity) {
                *tile_color = TileColor(color);
            }
        }
    }
}

// Retrieving the neighboring tiles and coloring them
pub fn color_player_neighbors(
    pattern: AttackPattern,
    player_q: &mut Query<(Entity, &mut TilePos), (With<Player>, Without<Enemy>)>, // We want the player
    tilemap_q: Query<(&TileStorage, &TilemapSize), With<PlayZoneTilemap>>, // Retrieve the tilemap and it's bundaries for safe writing
    tile_q: &mut Query<&mut TileColor>,
    color_state: &Query<&mut RoundColorState>,
) {
    let Ok((storage, map_size)) = tilemap_q.single() else {
        println!("No tilemap.");
        return;
    };
    let Ok(color_state) = color_state.single() else {
        return;
    };
    let attack_color = masked_color_from_index(color_state.index);

    for (_, pos) in player_q.iter_mut() {
        let pos = TilePos { x: pos.x, y: pos.y };
        let neighbors = gather_neighbors(pattern, pos, map_size, 5);
        apply_tile_color(neighbors, storage, tile_q, attack_color);
    }
}

// Credit: Codex 5.2, inspired by my intial player coloring
pub fn color_enemy_neighbors(
    enemy_q: &mut Query<(Entity, &mut TilePos), (With<Enemy>, Without<Player>)>,
    tilemap_q: Query<(&TileStorage, &TilemapSize), With<PlayZoneTilemap>>,
    tile_q: &mut Query<&mut TileColor>,
    color_state: &Query<&mut RoundColorState>,
) {
    let Ok((storage, map_size)) = tilemap_q.single() else {
        println!("No tilemap.");
        return;
    };
    let Ok(color_state) = color_state.single() else {
        return;
    };
    let attack_color = player_color_from_index(color_state.index);

    for (_, pos) in enemy_q.iter_mut() {
        let pos = TilePos { x: pos.x, y: pos.y };
        let neighbors = gather_neighbors(AttackPattern::Sides, pos, map_size, 2);
        apply_tile_color(neighbors, storage, tile_q, attack_color);
    }
}

// AI-generated (Codex): helper for kill checks based on tile color.
fn tile_matches_color(
    storage: &TileStorage,
    tile_q: &mut Query<&mut TileColor>,
    pos: &TilePos,
    target: Color,
) -> bool {
    if let Some(tile_entity) = storage.get(pos) {
        if let Ok(tile_color) = tile_q.get_mut(tile_entity) {
            return tile_color.0 == target;
        }
    }
    false
}

// AI-generated (Codex): despawn enemies standing on tiles matching their color.
pub fn despawn_enemies_on_matching_tile_color(
    mut commands: Commands,
    enemy_q: &mut Query<(Entity, &mut TilePos), (With<Enemy>, Without<Player>)>,
    tilemap_q: Query<(&TileStorage, &TilemapSize), With<PlayZoneTilemap>>,
    tile_q: &mut Query<&mut TileColor>,
    color_state: &Query<&mut RoundColorState>,
) {
    let Ok((storage, _map_size)) = tilemap_q.single() else {
        return;
    };
    let Ok(color_state) = color_state.single() else {
        return;
    };
    let enemy_color = masked_color_from_index(color_state.index);

    for (entity, pos) in enemy_q.iter_mut() {
        let pos = TilePos { x: pos.x, y: pos.y };
        if tile_matches_color(storage, tile_q, &pos, enemy_color) {
            commands.entity(entity).despawn();
        }
    }
}

// AI-generated (Codex): despawn the player if standing on a tile matching the player color.
pub fn despawn_player_on_matching_tile_color(
    mut commands: Commands,
    player_q: &mut Query<(Entity, &mut TilePos), (With<Player>, Without<Enemy>)>,
    tilemap_q: Query<(&TileStorage, &TilemapSize), With<PlayZoneTilemap>>,
    tile_q: &mut Query<&mut TileColor>,
    color_state: &Query<&mut RoundColorState>,
) {
    let Ok((storage, _map_size)) = tilemap_q.single() else {
        return;
    };
    let Ok(color_state) = color_state.single() else {
        return;
    };
    let player_color = player_color_from_index(color_state.index);

    for (entity, pos) in player_q.iter_mut() {
        let pos = TilePos { x: pos.x, y: pos.y };
        if tile_matches_color(storage, tile_q, &pos, player_color) {
            commands.entity(entity).despawn();
        }
    }
}

// Player uses the secondary ones
// Credit to Claude AI: Sonnet 4.5
// I needed a quick functional utility similar to what I did for the arrows.
pub fn update_player_color(
    mut player_q: Query<&mut Sprite, With<Player>>,
    color_state: &Query<&mut RoundColorState>,
) {
    if let Ok(color_state) = color_state.single() {
        let player_color = player_color_from_index(color_state.index);
        for mut sprite in player_q.iter_mut() {
            sprite.color = player_color;
        }
    }
}

// Enemy uses primary colors
// Same as before
pub fn update_enemy_color(
    mut enemy_q: Query<&mut Sprite, (With<Enemy>, Without<Player>)>,
    color_state: &Query<&mut RoundColorState>,
) {
    if let Ok(color_state) = color_state.single() {
        let masked_color = masked_color_from_index(color_state.index);
        for mut sprite in enemy_q.iter_mut() {
            sprite.color = masked_color;
        }
    }
}
