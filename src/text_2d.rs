use bevy::math::ops;
use bevy::prelude::*;

use crate::components::{GlobalTurnState, RoundColorState, TurnState};

#[derive(Component)]
pub struct AnimateTranslation;

#[derive(Component)]
pub struct AnimateRotation;

#[derive(Component)]
pub struct AnimateScale;

pub fn setup_font(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/SNPro-VariableFont_wght.ttf");
    let text_font = TextFont {
        font: font.clone(),
        font_size: 13.0,
        ..default()
    };
    let text_justification = Justify::Center;

    commands.spawn((
        Text2d::new(""),
        text_font,
        TextLayout::new_with_justify(text_justification),
        Transform::from_translation(Vec3::new(0.0, 135.0, 4.0f32)),
        TextBackgroundColor(Color::BLACK.with_alpha(0.0)),
        AnimateScale,
    ));
}

pub fn render_translated_text(
    time: Res<Time>,
    mut query: Query<&mut Transform, (With<Text2d>, With<AnimateTranslation>)>,
) {
    for mut transform in &mut query {
        transform.translation.x = 100.0 * ops::sin(time.elapsed_secs()) - 400.0;
        transform.translation.y = 100.0 * ops::cos(time.elapsed_secs());
    }
}

// Credit: ChatGPT Codex
pub fn render_rotated_text(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Text2d, &mut TextColor), With<AnimateRotation>>,
    color_state: Query<&RoundColorState>,
    turn_state: Query<&GlobalTurnState>,
) {
    // Render the color name and turn status in the text itself.
    let (color_label, color) = match color_state.single() {
        Ok(color_state) => match color_state.index {
            0 => ("Cyan", Color::srgba(0.0, 1.0, 1.0, 1.0)),
            1 => ("Magenta", Color::srgba(1.0, 0.0, 1.0, 1.0)),
            2 => ("Yellow", Color::srgba(1.0, 1.0, 0.0, 1.0)),
            _ => ("", Color::WHITE),
        },
        Err(_) => ("", Color::WHITE),
    };

    let status_label = match turn_state.single() {
        Ok(state) => match state.turn_state {
            TurnState::ColorPick => "Color Pick",
            TurnState::PlayerChange => "Player Change",
            TurnState::EnemySpawn => "Enemy Spawn",
            TurnState::MovePlayer => "Move Player",
            TurnState::AttackPlayer => "Player Attacks",
            TurnState::MoveEnemy => "Move Enemy",
            TurnState::AttackEnemy => "Enemy Attacks",
        },
        Err(_) => "Status",
    };

    let text_value = if color_label.is_empty() {
        status_label.to_string()
    } else {
        format!("{status_label} ({color_label})")
    };

    for (mut transform, mut text, mut text_color) in &mut query {
        transform.rotation = Quat::from_rotation_z(ops::cos(time.elapsed_secs()));
        text_color.0 = color;
        text.0 = text_value.clone();
    }
}

pub fn render_scaled_text(mut query: Query<&mut Transform, (With<Text2d>, With<AnimateScale>)>) {
    for mut transform in &mut query {
        let scale = 1.4;
        transform.scale.x = scale;
        transform.scale.y = scale;
    }
}
