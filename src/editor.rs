use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};

pub fn render_egui(mut contexts: EguiContexts) -> Result {
    egui::Window::new("Hello").show(contexts.ctx_mut()?, |ui| {
        ui.label("Hello World");
    });
    Ok(())
}
