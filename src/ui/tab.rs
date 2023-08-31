pub struct Tab {
    name: String,
    key_sequence: crate::scripting::KeySequence,
    runner_handle: crate::scripting::runner::RunnerHandle,
    test_str: String,

    current_action_index: usize,
}

impl Tab {
    pub fn new(name: String) -> Self {
        let seq = crate::scripting::KeySequence::new(vec![
            crate::scripting::Action::Wait(crate::time::Delay::new(1.)),
            crate::scripting::Action::Wait(crate::time::Delay::new(1.)),
            crate::scripting::Action::MouseMovement(
                crate::scripting::CursorMovementMode::Relative,
                (1000, 0),
            ),
        ]);

        let runner = crate::scripting::runner::RunnerHandle::new();

        runner
            .thread_channel
            .send(crate::scripting::runner::RunnerMessage::NewSequence(
                seq.clone(),
            ))
            .unwrap();

        Self {
            test_str: name.clone(),
            current_action_index: 0,
            runner_handle: runner,
            name,
            key_sequence: seq,
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn update_runner(&mut self) {
        loop {
            match self.runner_handle.thread_channel.try_recv() {
                Ok(msg) => match msg {
                    crate::scripting::runner::RunnerMessage::Goodbye => {
                        println!("The runner thread {} exited", self.name)
                    }
                    crate::scripting::runner::RunnerMessage::CrusorUpdate(id) => {
                        self.current_action_index = id;
                    }
                    _ => {
                        println!("Unexpected thread message: {msg:?}")
                    }
                },
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

    pub fn draw(&mut self, ui: &mut eframe::egui::Ui) {
        self.update_runner();
        ui.separator();

        /*
            vertical,
            horizontal
            truc a gauche
            vertical
            spacer with size of width -  truc a gauche size - liste size
            liste
        */

        let scrollbar_rect_id = ui.id().with("right_rect_scrollbar");

        let last_width: Option<f32> = ui.memory_mut(|mem| mem.data.get_temp(scrollbar_rect_id));
        ui.allocate_ui_at_rect(
            {
                let mut r = ui.max_rect();

                r.min.y += 100.;
                if let Some(w) = last_width {
                    r.min.x = r.max.x - w;
                }
                r
            },
            |ui| {
                let res = ui
                    .group(|ui| {
                        eframe::egui::ScrollArea::both()
                            .max_height(500.)
                            // .max_width(150.)
                            //             x      y
                            .auto_shrink([true, true])
                            .show(ui, |ui| {
                                ui.label(self.test_str.clone());
                                for (i, action) in self.key_sequence.actions().iter().enumerate() {
                                    let mut text = match action {
                                        crate::scripting::Action::Wait(d) => {
                                            format!("Delay {}ms", d.as_millis())
                                        }
                                        crate::scripting::Action::KeyPress(key) => {
                                            format!("Key({key:?}) press")
                                        }
                                        crate::scripting::Action::KeyRelease(key) => {
                                            format!("Key({key:?}) release")
                                        }
                                        crate::scripting::Action::MouseMovement(mode, amount) => {
                                            format!("Mouse movement {mode:?} {amount:?}")
                                        }
                                        crate::scripting::Action::ButtonPress(btn) => {
                                            format!("Button({btn:?}) press")
                                        }
                                        crate::scripting::Action::ButtonRelease(btn) => {
                                            format!("Button({btn:?}) release")
                                        }
                                        crate::scripting::Action::Scroll(dir, amount) => {
                                            format!("Mouse scroll {dir:?} {amount}")
                                        }
                                        crate::scripting::Action::Stop => "Stop".to_string(),
                                    };

                                    // custom X spacing :D
                                    text.push_str("   ");

                                    let text = eframe::egui::RichText::new(text).background_color(
                                        if i == self.current_action_index {
                                            eframe::egui::Color32::GREEN
                                        } else {
                                            eframe::egui::Color32::TRANSPARENT
                                        },
                                    );

                                    ui.add(eframe::egui::widgets::Label::new(text).wrap(false));
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
                            width + 10.
                            // width
                        },
                    )
                });
                ui.vertical(|ui| {
                    // Repaint if width changed
                    match last_width {
                        None => ui.ctx().request_repaint(),
                        Some(last_width) if last_width != width => ui.ctx().request_repaint(),
                        Some(_) => {}
                    }
                })
            },
        );

        let amount: usize = 10;
        ui.horizontal(|ui| {
            if ui.button("add").clicked() {
                for _i in 0..amount {
                    self.test_str.push('a');
                }
            }
            if ui.button("remove").clicked() {
                for _i in 0..amount {
                    self.test_str.pop();
                }
            }

            if ui.button("Move1").clicked() {
                self.current_action_index =
                    (self.current_action_index + 1) % (self.key_sequence.actions().len() - 1);
                // println!("{}",)
            }
        });
    }
}
