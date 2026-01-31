use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;

use crate::components::*;
use crate::troop_utilities::*;

pub fn process_keyboard(
    keys: Res<ButtonInput<KeyCode>>,
    mut player_q: Query<&mut TilePos, With<Player>>,
    tilemap_q: Query<(&TileStorage, &TilemapSize), With<PlayZoneTilemap>>,
    tile_q: Query<&mut TileColor>,
) {
    // Exit the application.
    if keys.just_pressed(KeyCode::Escape) {
        std::process::exit(0);
    }

    // Move The Player
    // TODO: possible PS5 controller path
    let Ok((_storage, map_size)) = tilemap_q.single() else {
        return;
    };

    for mut pos in player_q.iter_mut() {
        let mut next = *pos;

        if keys.just_pressed(KeyCode::ArrowUp) {
            next.y += 1;
        }
        if keys.just_pressed(KeyCode::ArrowDown) {
            next.y = next.y.saturating_sub(1);
        }
        if keys.just_pressed(KeyCode::ArrowRight) {
            next.x += 1;
        }
        if keys.just_pressed(KeyCode::ArrowLeft) {
            next.x = next.x.saturating_sub(1);
        }

        let max_x = (map_size.x - 1) as i32;
        let max_y = (map_size.y - 1) as i32;

        let x = (next.x as i32).clamp(0, max_x) as u32;
        let y = (next.y as i32).clamp(0, max_y) as u32;

        *pos = TilePos { x, y };
    }

    // Attack with player
    if keys.just_pressed(KeyCode::KeyQ) {
        crate::troop_utilities::color_player_neighbors(
            AttackPattern::Diagonal,
            player_q,
            tilemap_q,
            tile_q,
        );
    } else if keys.just_pressed(KeyCode::KeyW) {
        crate::troop_utilities::color_player_neighbors(
            AttackPattern::Sides,
            player_q,
            tilemap_q,
            tile_q,
        );
    } else if keys.just_pressed(KeyCode::KeyE) {
        crate::troop_utilities::color_player_neighbors(
            AttackPattern::Around,
            player_q,
            tilemap_q,
            tile_q,
        );
    } else if keys.just_pressed(KeyCode::KeyR) {
        crate::troop_utilities::color_player_neighbors(
            AttackPattern::Ultimate,
            player_q,
            tilemap_q,
            tile_q,
        );
    }
}
