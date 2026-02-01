use bevy::{math::Vec3, prelude::*};

// Code from: https://github.com/StarArawn/bevy_ecs_tilemap/blob/main/examples/helpers/camera.rs
// A simple camera system for moving and zooming the camera.
pub fn update_camera(mut query: Query<(&mut Transform, &mut Projection), With<Camera>>) {
    for (mut transform, mut projection) in query.iter_mut() {
        let Projection::Orthographic(ortho) = &mut *projection else {
            continue;
        };

        ortho.scale = 0.315;

        let z = transform.translation.z;

        transform.translation = Vec3::new(0.0, 25.0, 0.0);

        transform.translation.z = z;
    }
}
