use bevy::prelude::*;

pub const TILEMAP_SIDE_LENGHT: u32 = 16;
pub const LAYER_PLAYER: u8 = 2;
pub const LAYER_TILEMAP: u8 = 1;
pub const LAYER_UI: u8 = 3;

pub const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

pub const MOVE_DELAY_SECONDS: f32 = 0.4;
pub const ENEMY_ATTACK_WINDUP_SECONDS: f32 = 1.3;
pub const ENEMY_ATTACK_COOLDOWN_SECONDS: f32 = 1.3;