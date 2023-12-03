use eframe::NativeOptions;
use emulator_8085::gui::application::Application;

fn main() -> eframe::Result<()> {
    env_logger::init();
    let native_options = NativeOptions {
        initial_window_size: Some(eframe::egui::vec2(800., 800.)),
        resizable: false,
        ..Default::default()
    };
    eframe::run_native(
        "8085 Emulator",
        native_options,
        Box::new(|_| Box::new(Application::new())),
    )?;
    Ok(())
}
