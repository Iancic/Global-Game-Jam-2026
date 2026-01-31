use bevy::input_focus::InputFocus;
use bevy::prelude::*;
use bevy::window::WindowMode;
use bevy_ecs_tiled::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_egui::EguiPrimaryContextPass;

pub mod button_2d;
pub mod camera;
pub mod components;
pub mod constants;
pub mod editor;
pub mod input;
pub mod startup;
pub mod text_2d;
pub mod troop_utilities;
pub mod update;
pub mod utilities;
pub mod post_processing;

use crate::camera::*;
use crate::editor::*;
use crate::input::*;
use crate::startup::*;
use crate::text_2d::*;
use crate::update::*;

use crate::post_processing::*;

fn main() {
    App::new()
        // PLUGINS
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Color Wizard"),
                        mode: WindowMode::Fullscreen(
                            MonitorSelection::Primary,
                            VideoModeSelection::Current,
                        ),
                        resolution: (1920, 1080).into(),
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        ..Default::default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()), PostProcessPlugin)
        )
        .init_resource::<InputFocus>()
        .add_plugins(TilemapPlugin)
        .add_plugins(TiledPlugin::default())
        .add_plugins(EguiPlugin::default())
        // STARTUP
        .add_systems(Startup, (setup_scene, setup_font))
        // UPDATE
        .add_systems(
            Update,
            (
                process_keyboard,
                update_camera,
                update_game_logic,
                render_scaled_text,
                render_post_processing,
            ),
        )
        // EGUI
        .add_systems(EguiPrimaryContextPass, render_egui)
        .run();
}