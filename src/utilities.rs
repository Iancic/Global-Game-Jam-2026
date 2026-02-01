use crate::components::*;
use bevy::prelude::*;
use std::thread::sleep;
use std::time::Duration;

pub fn update_animated_sprites(
    time: Res<Time>,
    mut query_anim: Query<
        (&AnimationIndices, &mut AnimationTimer, &mut Sprite),
    >,
) {
    for (indices, mut timer, mut sprite) in &mut query_anim {
        timer.tick(time.delta());

        if timer.just_finished()
            && let Some(atlas) = &mut sprite.texture_atlas
        {
            atlas.index = if atlas.index == indices.last {
                indices.first
            } else {
                atlas.index + 1
            };
        }
    }
}

// Utility generated with Codex, just stops the game for a bit, will be used in turns
pub fn sleep_seconds(seconds: f32) {
    if seconds <= 0.0 {
        return;
    }
    sleep(Duration::from_secs_f32(seconds));
}
