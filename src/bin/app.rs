use eframe::NativeOptions;
use emulator_8085::gui::application::Application;

fn main() -> eframe::Result<()> {
    env_logger::init();

    let mut native_options = NativeOptions {
        resizable: false,
        ..Default::default()
    };
    native_options.maximized = true;
    eframe::run_native(
        "8085 Emulator",
        native_options,
        Box::new(|cc| {
            cc.egui_ctx.style_mut(|style| {
                for (_text_style, font_id) in style.text_styles.iter_mut() {
                    font_id.size = 18.;
                }
            });
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::new(Application::new())
        }),
    )?;
    Ok(())
}
