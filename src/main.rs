mod error;
mod input;
mod scripting;
mod threading;
mod time;
mod ui;

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(eframe::egui::vec2(1000.0, 750.0)), /*x800y450 is 16:9*/
        resizable: true,
        centered: true,
        vsync: true,
        decorated: false,
        transparent: true,
        always_on_top: true,
        default_theme: eframe::Theme::Dark,

        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| {
            use eframe::egui::{
                FontFamily::{Monospace, Proportional},
                FontId, TextStyle,
            };

            let mut style = (*cc.egui_ctx.style()).clone();
            style.text_styles = [
                (TextStyle::Heading, FontId::new(25.0, Proportional)),
                (TextStyle::Body, FontId::new(16.0, Proportional)),
                (TextStyle::Monospace, FontId::new(16.0, Monospace)),
                (TextStyle::Button, FontId::new(16.0, Proportional)),
                (TextStyle::Small, FontId::new(8.0, Proportional)),
            ]
            .into();
            cc.egui_ctx.set_style(style);
            std::boxed::Box::new(ui::Ui::new())
        }),
    )
    .unwrap();
}
