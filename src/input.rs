use bevy::prelude::*;

pub fn process_keyboard(keys: Res<ButtonInput<KeyCode>>) {
    // Exit the application.
    if keys.just_pressed(KeyCode::Escape) {
        std::process::exit(0);
    }
}
