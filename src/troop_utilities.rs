use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;
use crate::components::*;
use crate::constants::*;

// Credit: snapping idea to center in world from Codex 5.2
pub fn snap_troop_to_tilemap(
    mut player_q: Query<(&TilePos, &mut Transform), With<Troop>>, // All entities with transform, player and position
    tilemap_q: Query<(
        &TilemapSize,
        &TilemapGridSize,
        &TilemapTileSize,
        &TilemapType,
        &TilemapAnchor,
    ), With<PlayZoneTilemap>>, // Necessary for center in world
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

pub enum AttackPattern
{
    Diagonal,
    Sides,
    Around,
    Ultimate,
}

// Helper function to safely create a tile position with bounds checking
fn try_tile_pos(x: i32, y: i32, map_size: &TilemapSize) -> Option<TilePos> {
    if x >= 0 && y >= 0 && (x as u32) < map_size.x && (y as u32) < map_size.y {
        Some(TilePos { x: x as u32, y: y as u32 })
    } else {
        None
    }
}

// Credit: inspiration for retrieving the neighboring tiles using Codex 5.2
pub fn color_player_neighbors(pattern:AttackPattern,
    player_q: Query<&mut TilePos, With<Player>>, // We want the player
    tilemap_q: Query<(&TileStorage, &TilemapSize), With<PlayZoneTilemap>>, // Retrieve the tilemap and it's bundaries for safe writing
    mut tile_q: Query<&mut TileColor>,
) {
    let Ok((storage, map_size)) = tilemap_q.single() else {
        println!("No tilemap.");
        return;
    };
    // Now that I have where the player is in tile position,
    // I make an array with all positions around him
    for pos in player_q.iter() {
        let mut neighbors = Vec::new();
        let px = pos.x as i32;
        let py = pos.y as i32;

        // Always add current position if valid
        if let Some(tile) = try_tile_pos(px, py, map_size) {
            neighbors.push(tile);
        }

        // Hardcoded patterns but could not care less for now
        match pattern {
            AttackPattern::Diagonal => {
                // Top Right
                if let Some(tile) = try_tile_pos(px + 1, py + 1, map_size) { neighbors.push(tile); }
                if let Some(tile) = try_tile_pos(px + 2, py + 2, map_size) { neighbors.push(tile); }
                // Bottom Left
                if let Some(tile) = try_tile_pos(px - 1, py - 1, map_size) { neighbors.push(tile); }
                if let Some(tile) = try_tile_pos(px - 2, py - 2, map_size) { neighbors.push(tile); }
                // Top Left
                if let Some(tile) = try_tile_pos(px - 1, py + 1, map_size) { neighbors.push(tile); }
                if let Some(tile) = try_tile_pos(px - 2, py + 2, map_size) { neighbors.push(tile); }
                // Bottom Right
                if let Some(tile) = try_tile_pos(px + 1, py - 1, map_size) { neighbors.push(tile); }
                if let Some(tile) = try_tile_pos(px + 2, py - 2, map_size) { neighbors.push(tile); }
            },
            AttackPattern::Sides => {
                // Right
                if let Some(tile) = try_tile_pos(px + 1, py, map_size) { neighbors.push(tile); }
                if let Some(tile) = try_tile_pos(px + 2, py, map_size) { neighbors.push(tile); }
                // Left
                if let Some(tile) = try_tile_pos(px - 1, py, map_size) { neighbors.push(tile); }
                if let Some(tile) = try_tile_pos(px - 2, py, map_size) { neighbors.push(tile); }
                // Up
                if let Some(tile) = try_tile_pos(px, py + 1, map_size) { neighbors.push(tile); }
                if let Some(tile) = try_tile_pos(px, py + 2, map_size) { neighbors.push(tile); }
                // Down
                if let Some(tile) = try_tile_pos(px, py - 1, map_size) { neighbors.push(tile); }
                if let Some(tile) = try_tile_pos(px, py - 2, map_size) { neighbors.push(tile); }
            },
            AttackPattern::Around => {
                // Diagonals
                if let Some(tile) = try_tile_pos(px + 1, py + 1, map_size) { neighbors.push(tile); }
                if let Some(tile) = try_tile_pos(px - 1, py - 1, map_size) { neighbors.push(tile); }
                if let Some(tile) = try_tile_pos(px - 1, py + 1, map_size) { neighbors.push(tile); }
                if let Some(tile) = try_tile_pos(px + 1, py - 1, map_size) { neighbors.push(tile); }
                // Cardinals
                if let Some(tile) = try_tile_pos(px + 1, py, map_size) { neighbors.push(tile); }
                if let Some(tile) = try_tile_pos(px - 1, py, map_size) { neighbors.push(tile); }
                if let Some(tile) = try_tile_pos(px, py + 1, map_size) { neighbors.push(tile); }
                if let Some(tile) = try_tile_pos(px, py - 1, map_size) { neighbors.push(tile); }
            },
            AttackPattern::Ultimate => {
                // Combine all patterns
                // Diagonals
                for offset in 1..=2 {
                    if let Some(tile) = try_tile_pos(px + offset, py + offset, map_size) { neighbors.push(tile); }
                    if let Some(tile) = try_tile_pos(px - offset, py - offset, map_size) { neighbors.push(tile); }
                    if let Some(tile) = try_tile_pos(px - offset, py + offset, map_size) { neighbors.push(tile); }
                    if let Some(tile) = try_tile_pos(px + offset, py - offset, map_size) { neighbors.push(tile); }
                }
                // Cardinals
                for offset in 1..=2 {
                    if let Some(tile) = try_tile_pos(px + offset, py, map_size) { neighbors.push(tile); }
                    if let Some(tile) = try_tile_pos(px - offset, py, map_size) { neighbors.push(tile); }
                    if let Some(tile) = try_tile_pos(px, py + offset, map_size) { neighbors.push(tile); }
                    if let Some(tile) = try_tile_pos(px, py - offset, map_size) { neighbors.push(tile); }
                }
            },
        }

        // With these new positions I can iterate them one by one and if not outside the bounds color them
        for tile in neighbors {
            if tile.x < map_size.x && tile.y < map_size.y { // the neighbouring tile can be outside that is invalid
                if let Some(tile_entity) = storage.get(&tile){ // retrieve entity
                    if let Ok(mut color) = tile_q.get_mut(tile_entity) // If there is a retrievable tile
                    {
                        *color = TileColor(Color::srgba(1.0, 0.0, 0.0, 1.0)); // TODO: red for now but make it based on
                    }
                }
            }
        }
    }
}
