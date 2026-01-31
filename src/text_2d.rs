use bevy::{math::ops, prelude::*, sprite::Text2dShadow};

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
        font_size: 15.0,
        ..default()
    };
    let text_justification = Justify::Center;

    commands.spawn((
        Text2d::new(" Enemy Attacks "),
        text_font,
        TextLayout::new_with_justify(text_justification),
        Transform::from_translation(Vec3::new(0.0, 165.0, 4.0f32)),
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

pub fn render_rotated_text(
    time: Res<Time>,
    mut query: Query<&mut Transform, (With<Text2d>, With<AnimateRotation>)>,
) {
    for mut transform in &mut query {
        transform.rotation = Quat::from_rotation_z(ops::cos(time.elapsed_secs()));
    }
}

pub fn render_scaled_text(mut query: Query<&mut Transform, (With<Text2d>, With<AnimateScale>)>) {
    for mut transform in &mut query {
        let scale = 1.4;
        transform.scale.x = scale;
        transform.scale.y = scale;
    }
}
