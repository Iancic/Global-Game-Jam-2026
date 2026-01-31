use crate::components::*;
use bevy::prelude::*;

pub fn animate_sprites(
    time: Res<Time>,
    mut query_anim: Query<(&AnimationIndices, &mut AnimationTimer, &mut Sprite), (Without<Player>, Without<Enemy>)>,
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
