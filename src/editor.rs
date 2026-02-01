use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};

pub fn render_egui(mut contexts: EguiContexts) -> Result {
    egui::Window::new("Color Wizard").show(contexts.ctx_mut()?, |ui| {
        ui.label("Esc - Exit");
        ui.label("QWER - Shoot");
        ui.label("Arrows - Move Around");
    });
    Ok(())
}
