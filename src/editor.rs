use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};

pub fn ui_example_system(mut contexts: EguiContexts) -> Result {
    egui::Window::new("Hello").show(contexts.ctx_mut()?, |ui| {
        ui.label("Hello World");
    });
    Ok(())
}
