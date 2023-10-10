#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]
#[macro_use]
extern crate log;

mod error;
mod logger;
mod scripting;
mod threading;
mod time;
mod ui;

fn main() {
    logger::init(None);

    let options = eframe::NativeOptions {
        initial_window_size: Some(eframe::egui::vec2(1000.0, 750.0)), /*x800y450 is 16:9*/
        resizable: true,
        centered: true,
        vsync: true,
        decorated: false,
        transparent: true,
        always_on_top: false,
        default_theme: eframe::Theme::Dark,
        // icon_data: Some(
        //     eframe::IconData::try_from_png_bytes(include_bytes!(
        //         "..\\assets\\icons8-robot-64_w.png"
        //     ))
        //     .unwrap(),
        // ),
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
