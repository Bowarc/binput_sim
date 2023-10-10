// Formatting
const COMBO_BOX_TEXT_SPACING: &str = "                               ";

pub struct Tab {
    name: String,
    action_sequence: crate::scripting::ActionSequence,
    runner_handle: crate::scripting::runner::RunnerHandle,
    current_action_index: usize,
}

impl Tab {
    pub fn new(name: String) -> Self {
        let seq = crate::scripting::ActionSequence::new(vec![
            crate::scripting::Action::Wait(crate::time::Delay::new(10.)),
            crate::scripting::Action::Wait(crate::time::Delay::new(1.)),
            crate::scripting::Action::MouseMovement(
                crate::scripting::CursorMovementMode::Relative,
                (0, -200),
            ),
            crate::scripting::Action::KeyPress(inputbot::KeybdKey::SpaceKey),
            crate::scripting::Action::Wait(crate::time::Delay::new(0.1)),
            crate::scripting::Action::KeyRelease(inputbot::KeybdKey::SpaceKey),
            crate::scripting::Action::Wait(crate::time::Delay::new(1.5)),
            crate::scripting::Action::ButtonPress(inputbot::MouseButton::LeftButton),
            crate::scripting::Action::Wait(crate::time::Delay::new(0.1)),
            crate::scripting::Action::ButtonRelease(inputbot::MouseButton::LeftButton),
            crate::scripting::Action::Wait(crate::time::Delay::new(1.5)),
            crate::scripting::Action::Scroll(crate::scripting::ScrollDirection::Y, 10),
            crate::scripting::Action::Wait(crate::time::Delay::new(1.5)),
            crate::scripting::Action::KeySequence(String::from("Cool text written by a bot")),
            crate::scripting::Action::Stop,
        ]);

        let runner = crate::scripting::runner::RunnerHandle::new(name.clone());

        // runner
        //     .thread_channel
        //     .send(crate::scripting::runner::RunnerMessage::NewSequence(
        //         seq.clone(),
        //     ))
        //     .unwrap();

        Self {
            current_action_index: 0,
            runner_handle: runner,
            name,
            action_sequence: seq,
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
    pub fn runner_running(&self) -> bool {
        self.runner_handle.is_runner_running()
    }

    fn update_runner(&mut self) {
        loop {
            match self.runner_handle.try_recv() {
                Ok(msg) => {
                    debug!("tab: {} received a message {msg:?}", self.name);
                    match msg {
                        crate::scripting::runner::RunnerMessage::Goodbye => {
                            debug!("The runner thread {} exited", self.name)
                        }
                        crate::scripting::runner::RunnerMessage::CrusorUpdate(cursor) => {
                            trace!("Tab cursor updated to {cursor}");
                            self.current_action_index = cursor;
                        }
                        crate::scripting::runner::RunnerMessage::SequenceDeleted => {
                            self.current_action_index = 0
                        }

                        _ => {
                            warn!("Unexpected thread message: {msg:?}")
                        }
                    }
                }
                Err(e) => match e {
                    std::sync::mpsc::TryRecvError::Empty => {
                        break;
                    }
                    std::sync::mpsc::TryRecvError::Disconnected => {
                        break;
                        // self.runner_handle = crate::scripting::runner::RunnerHandle::new()
                    }
                },
            }
        }
    }

    pub fn update(&mut self) {
        self.update_runner()
    }

    pub fn draw(&mut self, ui: &mut eframe::egui::Ui) {
        ui.separator();

        /*
            vertical,
            horizontal
            truc a gauche
            vertical
            spacer with size of width -  truc a gauche size - list size
            liste
        */

        ui.horizontal(|ui| {
            ui.label("Runner state: ");
            ui.label(if self.runner_handle.is_runner_running() {
                eframe::egui::RichText::new("Running").color(eframe::egui::Color32::GREEN)
            } else {
                eframe::egui::RichText::new("Stopped").color(eframe::egui::Color32::RED)
            })
        });

        ui.add_space(20.);

        // for _ in 0..30 {
        //     ui.label("Salut");
        // }
        ui.label("Add actions to the sequence");
        ui.add_space(10.);

        {
            ui.horizontal(|ui| {
                if ui.button("Delay").clicked() {
                    self.action_sequence
                        .actions()
                        .push(crate::scripting::Action::Wait(crate::time::Delay::new(1.)));
                }

                if ui.button("Stop").clicked() {
                    self.action_sequence
                        .actions()
                        .push(crate::scripting::Action::Stop);
                }
            });

            ui.add_space(10.);

            ui.horizontal(|ui| {
                if ui.button("Mouse press").clicked() {
                    self.action_sequence
                        .actions()
                        .push(crate::scripting::Action::ButtonPress(
                            inputbot::MouseButton::LeftButton,
                        ));
                }

                if ui.button("Mouse release").clicked() {
                    self.action_sequence
                        .actions()
                        .push(crate::scripting::Action::ButtonPress(
                            inputbot::MouseButton::LeftButton,
                        ));
                }
            });
            ui.add_space(10.);

            ui.horizontal(|ui| {
                if ui.button("Key press").clicked() {
                    self.action_sequence
                        .actions()
                        .push(crate::scripting::Action::KeyPress(
                            inputbot::KeybdKey::SpaceKey,
                        ));
                }

                if ui.button("Key release").clicked() {
                    self.action_sequence
                        .actions()
                        .push(crate::scripting::Action::KeyRelease(
                            inputbot::KeybdKey::SpaceKey,
                        ));
                }
            });
            ui.add_space(10.);

            ui.horizontal(|ui| {
                if ui.button("Mouse movement").clicked() {
                    self.action_sequence
                        .actions()
                        .push(crate::scripting::Action::MouseMovement(
                            crate::scripting::CursorMovementMode::Absolute,
                            (0, 0),
                        ));
                }

                if ui.button("Mouse scroll").clicked() {
                    self.action_sequence
                        .actions()
                        .push(crate::scripting::Action::Scroll(
                            crate::scripting::ScrollDirection::Y,
                            -10,
                        ));
                }
            });

            ui.add_space(10.);

            ui.horizontal(|ui| {
                if ui.button("Key sequence").clicked() {
                    self.action_sequence
                        .actions()
                        .push(crate::scripting::Action::KeySequence(String::new()))
                }
            });
        }

        self.draw_current_sequence(ui);
    }

    fn draw_current_sequence(&mut self, ui: &mut eframe::egui::Ui) {
        let scrollbar_rect_id = ui.id().with("right_rect_scrollbar");

        let last_width: Option<f32> = ui.memory_mut(|mem| mem.data.get_temp(scrollbar_rect_id));
        ui.allocate_ui_at_rect(
            {
                let mut r = ui.max_rect();

                r.min.y += 100.;
                if let Some(w) = last_width {
                    let w = w.max(500.);
                    r.min.x = r.max.x - w;
                }
                r
            },
            |ui| {
                ui.horizontal(|ui| {
                    if ui.button("Run sequence").clicked() {
                        debug!("Sending a request to the runner");
                        self.runner_handle
                            .send(crate::scripting::runner::RunnerMessage::SetSequence(
                                self.action_sequence.clone(),
                            ))
                            .unwrap();

                        self.runner_handle
                            .send(crate::scripting::runner::RunnerMessage::StartSequence)
                            .unwrap()
                    }
                    if ui.button("Stop sequence").clicked() {
                        debug!("Sending a stop request to the runner");
                        self.runner_handle
                            .send(crate::scripting::runner::RunnerMessage::StopSequence)
                            .unwrap();
                        self.runner_handle
                            .send(crate::scripting::runner::RunnerMessage::CleanSequence)
                            .unwrap();
                    }

                    ui.with_layout(
                        eframe::egui::Layout::right_to_left(eframe::egui::Align::Min),
                        |ui| {
                            ui.add_space(10.);

                            let btn_response = ui
                                .add(eframe::egui::widgets::Button::new(
                                    eframe::egui::RichText::new("Release all")
                                        .color(eframe::egui::Color32::from_rgb(200, 0, 0)),
                                ))
                                .on_hover_ui(|ui| {
                                    ui.label("Releases all keys and mouse buttons");
                                });

                            if btn_response.clicked() {
                                crate::scripting::utils::release_all_kbkeys();
                                crate::scripting::utils::release_all_mouse_btns();
                            }
                        },
                    );
                });
                ui.horizontal(|ui| {
                    if ui.button("Pause sequence").clicked() {
                        debug!("Sending a request to pause the current sequence");
                        self.runner_handle
                            .send(crate::scripting::runner::RunnerMessage::StopSequence)
                            .unwrap();
                    }

                    if ui.button("Resume sequence").clicked() {
                        debug!("Sending a request to resume the current sequence");
                        self.runner_handle
                            .send(crate::scripting::runner::RunnerMessage::StartSequence)
                            .unwrap();
                    }
                });

                let res = ui
                    .group(|ui| {
                        eframe::egui::ScrollArea::both()
                            .max_height(500.)
                            // .max_width(150.)
                            //             x      y
                            .auto_shrink([true, true])
                            .show(ui, |ui| {
                                ui.label("Actions: (* => unsaved)");

                                let mut i = 0;

                                while let Some(action) = self.action_sequence.actions().get_mut(i) {
                                    let mut delete_requested = false;

                                    ui.horizontal(|ui| {
                                        let cursor = if i == self.current_action_index {
                                            eframe::egui::RichText::new("->")
                                                .background_color(eframe::egui::Color32::GREEN)
                                                .monospace()
                                        } else {
                                            eframe::egui::RichText::new("  ")
                                                .background_color(
                                                    eframe::egui::Color32::TRANSPARENT,
                                                )
                                                .monospace()
                                        };

                                        if ui
                                            .button(
                                                eframe::egui::RichText::new("X")
                                                    .color(eframe::egui::Color32::DARK_RED),
                                            )
                                            .clicked()
                                        {
                                            delete_requested = true;
                                        }

                                        ui.label(cursor);

                                        match action {
                                            crate::scripting::Action::Wait(d) => {
                                                draw_action_wait(ui, d, i, &self.name);
                                            }
                                            crate::scripting::Action::KeyPress(key) => {
                                                draw_action_keypress(ui, key, i, &self.name);
                                                // format!("Key({key:?}) press")
                                            }
                                            crate::scripting::Action::KeyRelease(key) => {
                                                draw_action_keyrelease(ui, key, i, &self.name);

                                                // format!("Key({key:?}) release")
                                            }
                                            crate::scripting::Action::MouseMovement(
                                                mode,
                                                amount,
                                            ) => {
                                                draw_action_mouse_movement(
                                                    ui, mode, amount, i, &self.name,
                                                );
                                                // format!("Mouse movement {mode:?} {amount:?}")
                                            }
                                            crate::scripting::Action::ButtonPress(btn) => {
                                                draw_action_buttonpress(ui, btn, i, &self.name)
                                            }
                                            crate::scripting::Action::ButtonRelease(btn) => {
                                                draw_action_buttonrelease(ui, btn, i, &self.name)
                                            }
                                            crate::scripting::Action::Scroll(dir, amount) => {
                                                draw_action_scroll(ui, dir, amount, i, &self.name)

                                                // format!("Mouse scroll {dir:?} {amount}")
                                            }
                                            crate::scripting::Action::KeySequence(s) => {
                                                draw_action_keysequence(ui, s, i, &self.name)
                                            }
                                            crate::scripting::Action::Stop => {
                                                ui.label("Stop");
                                            }
                                        };
                                    });

                                    if delete_requested {
                                        self.action_sequence.actions().remove(i);
                                    } else {
                                        i += 1;
                                    }
                                }
                            });
                    })
                    .response;
                let width = res.rect.width();

                let width = width + 20.;

                ui.memory_mut(|mem| {
                    mem.data.insert_temp(
                        scrollbar_rect_id,
                        if last_width.is_none() {
                            width * 2.
                        } else {
                            width + 30.
                        },
                    )
                });

                // Repaint if width changed
                match last_width {
                    None => ui.ctx().request_repaint(),
                    Some(last_width) if last_width != width => ui.ctx().request_repaint(),
                    Some(_) => {}
                }
            },
        );
    }
}

fn draw_action_wait(
    ui: &mut eframe::egui::Ui,
    d: &mut crate::time::Delay,
    i: usize,
    tab_name: &str,
) {
    let base_id = format!("{tab_name}wait{i}");

    ui.horizontal(|ui| {
        ui.label("Delay ");
        let saved_unit = d.unit;

        let mem_text_id = base_id.clone() + "memtext";
        let mut text: String =
            match ui.memory_mut(|mem| mem.data.get_temp::<String>(mem_text_id.clone().into())) {
                Some(t) => t,
                None => d.v.to_string(),
            };

        let mut equal = false;
        if let Ok(v) = text.parse::<f64>() {
            if v == d.v {
                equal = true;
            }
        }

        if !equal {
            ui.label("*");
        }

        // ui.text_edit_singleline(&mut text);
        ui.add(
            eframe::egui::widgets::TextEdit::singleline(&mut text)
                .id((base_id.clone() + "textedit").into()),
        );

        eframe::egui::ComboBox::from_id_source(base_id.clone() + "combobox")
            .selected_text(format!("{:?}    ", d.unit))
            .show_ui(ui, |ui| {
                for unit in [
                    crate::time::TimeUnit::Nanoseconds,
                    crate::time::TimeUnit::Microseconds,
                    crate::time::TimeUnit::Milliseconds,
                    crate::time::TimeUnit::Seconds,
                ] {
                    ui.selectable_value(&mut d.unit, unit, format!("{unit:?}"));
                }
            });

        if let Ok(v) = text.parse::<f64>() {
            d.v = v;
            if text.ends_with('.') || text != format!("{v}") {
                ui.memory_mut(|mem| mem.data.insert_temp(mem_text_id.into(), text))
            } else {
                ui.memory_mut(|mem| mem.data.remove::<String>(mem_text_id.into()))
            }
        } else {
            ui.memory_mut(|mem| mem.data.insert_temp(mem_text_id.into(), text))
        }

        if d.unit != saved_unit {
            let new_v = match d.unit {
                crate::time::TimeUnit::Nanoseconds => saved_unit.to_nanos(d.v),
                crate::time::TimeUnit::Microseconds => saved_unit.to_micros(d.v),
                crate::time::TimeUnit::Milliseconds => saved_unit.to_millis(d.v),
                crate::time::TimeUnit::Seconds => saved_unit.to_seconds(d.v),
            };
            d.v = new_v
        }
    });
}

fn draw_action_keypress(
    ui: &mut eframe::egui::Ui,
    curr_key: &mut inputbot::KeybdKey,
    i: usize,
    tab_name: &str,
) {
    use strum::IntoEnumIterator as _;

    let base_id = format!("{tab_name}keypress{i}");
    ui.horizontal(|ui| {
        ui.label("Kb key press ");
        eframe::egui::ComboBox::from_id_source(base_id + "combobox")
            .selected_text(format!("{curr_key:?}{COMBO_BOX_TEXT_SPACING}"))
            .show_ui(ui, |ui| {
                for key in inputbot::KeybdKey::iter() {
                    ui.selectable_value(curr_key, key, format!("{key:?}"));
                }
            });
    });
}

fn draw_action_keyrelease(
    ui: &mut eframe::egui::Ui,
    curr_key: &mut inputbot::KeybdKey,
    i: usize,
    tab_name: &str,
) {
    use strum::IntoEnumIterator as _;

    let base_id = format!("{tab_name}keyrelease{i}");

    ui.horizontal(|ui| {
        ui.label("Kb key release ");
        eframe::egui::ComboBox::from_id_source(base_id + "combobox")
            .selected_text(format!("{curr_key:?}{COMBO_BOX_TEXT_SPACING}"))
            .show_ui(ui, |ui| {
                for key in inputbot::KeybdKey::iter() {
                    ui.selectable_value(curr_key, key, format!("{key:?}"));
                }
            });
    });
}
fn draw_action_mouse_movement(
    ui: &mut eframe::egui::Ui,
    curr_mode: &mut crate::scripting::CursorMovementMode,
    curr_amount: &mut (i32, i32),
    i: usize,
    tab_name: &str,
) {
    let base_id = format!("{tab_name}wait{i}");

    fn draw_amount_text_edit(
        ui: &mut eframe::egui::Ui,
        base_id: String,
        amnt: &mut i32,
        name: &str,
    ) {
        // let base = *amnt;

        let mut txt = format!("{amnt}");

        ui.add(
            eframe::egui::widgets::TextEdit::singleline(&mut txt)
                .id((base_id + &format!("textedit{name}")).into())
                .desired_width(50.),
        );

        if let Ok(modified_amnt) = txt.parse::<i32>() {
            *amnt = modified_amnt
        }
    }

    ui.horizontal(|ui| {
        ui.label("Mouse movement ");

        eframe::egui::ComboBox::from_id_source(base_id.clone() + "combobox")
            .selected_text(format!("{curr_mode:?}"))
            .show_ui(ui, |ui| {
                for mode in [
                    crate::scripting::CursorMovementMode::Absolute,
                    crate::scripting::CursorMovementMode::Relative,
                ] {
                    ui.selectable_value(curr_mode, mode, format!("{mode:?}"));
                }
            });

        draw_amount_text_edit(ui, base_id.clone(), &mut curr_amount.0, "X");

        draw_amount_text_edit(ui, base_id.clone(), &mut curr_amount.1, "Y");
    });
}

fn draw_action_buttonpress(
    ui: &mut eframe::egui::Ui,
    curr_btn: &mut inputbot::MouseButton,
    i: usize,
    tab_name: &str,
) {
    use strum::IntoEnumIterator as _;

    let base_id = format!("{tab_name}buttonpress{i}");

    ui.horizontal(|ui| {
        ui.label("Mouse press ");
        eframe::egui::ComboBox::from_id_source(base_id + "combobox")
            .selected_text(format!("{curr_btn:?}{COMBO_BOX_TEXT_SPACING}"))
            .show_ui(ui, |ui| {
                for btn in inputbot::MouseButton::iter() {
                    ui.selectable_value(curr_btn, btn, format!("{btn:?}"));
                }
            });
    });
}

fn draw_action_buttonrelease(
    ui: &mut eframe::egui::Ui,
    curr_btn: &mut inputbot::MouseButton,
    i: usize,
    tab_name: &str,
) {
    use strum::IntoEnumIterator as _;

    let base_id = format!("{tab_name}buttonrelease{i}");

    ui.horizontal(|ui| {
        ui.label("Mouse release ");
        eframe::egui::ComboBox::from_id_source(base_id + "combobox")
            .selected_text(format!("{curr_btn:?}{COMBO_BOX_TEXT_SPACING}"))
            .show_ui(ui, |ui| {
                for btn in inputbot::MouseButton::iter() {
                    ui.selectable_value(curr_btn, btn, format!("{btn:?}"));
                }
            });
    });
}

fn draw_action_scroll(
    ui: &mut eframe::egui::Ui,
    curr_dir: &mut crate::scripting::ScrollDirection,
    curr_amnt: &mut i32,
    i: usize,
    tab_name: &str,
) {
    let base_id = format!("{tab_name}scroll{i}");

    ui.horizontal(|ui| {
        ui.label("Mouse scroll ");
        eframe::egui::ComboBox::from_id_source(base_id.clone() + "combobox")
            .selected_text(format!("{curr_dir:?}"))
            .show_ui(ui, |ui| {
                for dir in [
                    crate::scripting::ScrollDirection::X,
                    crate::scripting::ScrollDirection::Y,
                ] {
                    ui.selectable_value(curr_dir, dir, format!("{dir:?}"));
                }
            });

        let mut txt = format!("{curr_amnt}");

        ui.add(
            eframe::egui::widgets::TextEdit::singleline(&mut txt)
                .id((base_id + "textedit").into())
                .desired_width(50.),
        );

        if let Ok(modified_amnt) = txt.parse::<i32>() {
            *curr_amnt = modified_amnt
        }
    });
}

fn draw_action_keysequence(
    ui: &mut eframe::egui::Ui,
    curr_seq: &mut String,
    i: usize,
    tab_name: &str,
) {
    let base_id = format!("{tab_name}keysequence{i}");

    ui.horizontal(|ui| {
        ui.label("key sequence ");

        let hint_text = "Write a sentence here";
        let id = base_id + "textedit";

        ui.add(
            eframe::egui::widgets::TextEdit::multiline(curr_seq)
                .id(id.into())
                .hint_text(hint_text)
                .desired_rows(1),
        );
    });
}
