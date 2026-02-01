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
        font_size: 12.0,
        ..default()
    };
    let text_justification = Justify::Center;

    commands.spawn((
        Text2d::new(""),
        text_font,
        TextLayout::new_with_justify(text_justification).with_no_wrap(),
        TextColor(Color::WHITE),
        Transform::from_translation(Vec3::new(0.0, -120.0, 4.0)),
        TextBackgroundColor(Color::BLACK.with_alpha(0.0)),
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

// Credit: ChatGPT Codex cause I needed to convert the state of the game fast
pub fn render_rotated_text(
    mut query: Query<(&mut Transform, &mut Text2d, &mut TextColor), With<Text2d>>,
    color_state: Query<&RoundColorState>,
    turn_state: Query<&GlobalTurnState>,
) {
    
    let color = Color::BLACK;

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

    let text_value = status_label.to_string();

    for (mut transform, mut text, mut text_color) in &mut query {
        transform.rotation = Quat::IDENTITY;
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
