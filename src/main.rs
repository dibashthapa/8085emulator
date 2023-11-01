mod core;
mod gui;
mod parser;
mod token;

use core::application::Application;

use eframe::NativeOptions;

fn main() -> eframe::Result<()> {
    env_logger::init();
    let native_options = NativeOptions {
        initial_window_size: Some(eframe::egui::vec2(800., 800.)),
        ..Default::default()
    };
    eframe::run_native(
        "8085 Emulator",
        native_options,
        Box::new(|_| Box::new(Application::new())),
    )
}
