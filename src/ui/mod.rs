mod tab;
mod utils;

pub struct Ui {
    tabs: Vec<tab::Tab>,
    current_tab_index: usize,
}

impl Ui {
    pub fn new() -> Self {
        let tabs = vec![
            tab::Tab::new(String::from("Tab1")),
            tab::Tab::new(String::from("Tab2")),
        ];

        Self {
            tabs,
            current_tab_index: 0,
        }
    }
    fn draw_title_bar(
        &mut self,
        ui: &mut eframe::egui::Ui,
        frame: &mut eframe::Frame,
        title_bar_rect: eframe::epaint::Rect,
        title: &str,
    ) {
        let painter = ui.painter();

        let title_bar_response = ui.interact(
            title_bar_rect,
            eframe::egui::Id::new("title_bar"),
            eframe::egui::Sense::click(),
        );

        // Paint the title:
        painter.text(
            title_bar_rect.center(),
            eframe::emath::Align2::CENTER_CENTER,
            title,
            eframe::epaint::FontId::proportional(20.0),
            ui.style().visuals.text_color(),
        );

        // Paint the line under the title:
        painter.line_segment(
            [
                title_bar_rect.left_bottom() + eframe::epaint::vec2(1.0, 0.0),
                title_bar_rect.right_bottom() + eframe::epaint::vec2(-1.0, 0.0),
            ],
            ui.visuals().widgets.noninteractive.bg_stroke,
        );

        // Interact with the title bar (drag to move window):
        if title_bar_response.double_clicked() {
            // frame.set_maximized(!frame.info().window_info.maximized);
        } else if title_bar_response.is_pointer_button_down_on() {
            frame.drag_window();
        }

        // Show toggle button for light/dark mode
        ui.allocate_ui_at_rect(title_bar_rect, |ui| {
            ui.with_layout(
                eframe::egui::Layout::left_to_right(eframe::egui::Align::Center),
                |ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.visuals_mut().button_frame = false;
                    ui.add_space(8.0);
                    eframe::egui::widgets::global_dark_light_mode_switch(ui);
                },
            );
        });

        // Show some close/maximize/minimize buttons for the native window.
        ui.allocate_ui_at_rect(title_bar_rect, |ui| {
            ui.with_layout(
                eframe::egui::Layout::right_to_left(eframe::egui::Align::Center),
                |ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.visuals_mut().button_frame = false;
                    ui.add_space(8.0);

                    let button_height = 12.0;

                    if ui
                        .add(eframe::egui::Button::new(
                            eframe::egui::RichText::new("❌").size(button_height),
                        ))
                        .on_hover_text("Close the window")
                        .clicked()
                    {
                        frame.close();
                    }

                    let (hover_text, clicked_state) = if frame.info().window_info.maximized {
                        ("Restore window", false)
                    } else {
                        ("Maximize window", true)
                    };

                    if ui
                        .add(eframe::egui::Button::new(
                            eframe::egui::RichText::new("🗗").size(button_height),
                        ))
                        .on_hover_text(hover_text)
                        .clicked()
                    {
                        frame.set_maximized(clicked_state);
                    }

                    if ui
                        .add(eframe::egui::Button::new(
                            eframe::egui::RichText::new("🗕").size(button_height),
                        ))
                        .on_hover_text("Minimize the window")
                        .clicked()
                    {
                        frame.set_minimized(true);
                    }
                },
            );
        });
    }
    fn draw_tabs(&mut self, ui: &mut eframe::egui::Ui, rect: eframe::egui::Rect) {
        utils::centerer(ui, |ui| {
            for (index, tab) in self.tabs.iter().enumerate() {
                if ui
                    .button(eframe::egui::RichText::new(tab.name()).size(20.).strong())
                    .clicked()
                {
                    self.current_tab_index = index
                }
            }
        });

        let current_tab = self.tabs.get_mut(self.current_tab_index).unwrap();

        current_tab.draw(ui);
    }
}

impl eframe::App for Ui {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        eframe::egui::containers::CentralPanel::default()
            .frame(
                eframe::egui::Frame::none()
                    .fill(ctx.style().visuals.window_fill())
                    .rounding(10.0)
                    .stroke(ctx.style().visuals.widgets.noninteractive.fg_stroke)
                    .outer_margin(0.5),
            )
            .show(ctx, |ui| {
                let app_rect = ui.max_rect();

                // draw the title bar

                let title_bar_rect = {
                    let mut rect = app_rect;
                    rect.max.y = rect.min.y + 32.0;
                    rect
                };
                self.draw_title_bar(ui, frame, title_bar_rect, "Bwrc Input Sim");

                let tab_rect = {
                    let mut rect = title_bar_rect;
                    rect.min.y = title_bar_rect.max.y + 10.;
                    rect
                };

                // ui.allocate_ui_at_rect(tab_rect, |ui| {
                self.draw_tabs(ui, tab_rect);
                // });
            });
    }
}
