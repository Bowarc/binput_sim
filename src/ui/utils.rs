pub fn centerer(ui: &mut eframe::egui::Ui, add_contents: impl FnOnce(&mut eframe::egui::Ui)) {
    ui.horizontal(|ui| {
        let id = ui.id().with("_centerer");
        let last_width: Option<f32> = ui.memory_mut(|mem| mem.data.get_temp(id));
        if let Some(last_width) = last_width {
            ui.add_space((ui.available_width() - last_width) / 2.0);
        }
        let res = ui.scope(|ui| add_contents(ui)).response;
        let width = res.rect.width();
        ui.memory_mut(|mem| mem.data.insert_temp(id, width));

        // Repaint if width changed
        match last_width {
            None => ui.ctx().request_repaint(),
            Some(last_width) if last_width != width => ui.ctx().request_repaint(),
            Some(_) => {}
        }
    });
}

pub fn right(ui: &mut eframe::egui::Ui, add_contents: impl FnOnce(&mut eframe::egui::Ui)) {
    ui.horizontal(|ui| {
        let id = ui.id().with("_right");

        let last_width: Option<f32> = ui.memory_mut(|mem| mem.data.get_temp(id));
        dbg!(last_width);
        if let Some(last_width) = last_width {
            ui.add_space(ui.available_width() - last_width);
        }

        let res = ui.scope(|ui| add_contents(ui)).response;
        let width = res.rect.width();

        ui.memory_mut(|mem| mem.data.insert_temp(id, width));

        // Repaint if width changed
        match last_width {
            None => ui.ctx().request_repaint(),
            Some(last_width) if last_width != width => ui.ctx().request_repaint(),
            Some(_) => {}
        }
    });
}
