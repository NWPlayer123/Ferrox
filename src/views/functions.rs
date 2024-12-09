use egui_extras::{Column, TableBuilder};

pub struct FunctionsTab;

impl FunctionsTab {
    pub fn update(&mut self, ui: &mut egui::Ui) {
        // This is currently placeholder data. TODO: add actual state? First let's see how performant it is
        // without caching gui elements
        let available_size = ui.available_size();
        let lookup: [(&str, u32); 3] = [
            ("__check_pad3", 0x80003100),
            ("__set_debug_bba", 0x80003140),
            ("__get_debug_bba", 0x8000314C),
        ];

        egui::ScrollArea::both().auto_shrink([false, false]).show(ui, |ui| {
            ui.set_width(available_size.x);
            ui.set_height(available_size.y);
            TableBuilder::new(ui)
                .column(Column::auto().at_least(140.0).resizable(true)) // Function name column with initial width
                .column(Column::remainder().at_least(80.0)) // Address column with minimum width
                .header(20.0, |mut header| {
                    // Add headers
                    header.col(|ui| {
                        ui.heading("Function Name");
                    });
                    header.col(|ui| {
                        ui.heading("Address");
                    });
                })
                .body(|body| {
                    body.rows(20.0, lookup.len(), |mut row| {
                        let index = row.index();
                        row.col(|ui| {
                            ui.label(lookup[index].0);
                        });
                        row.col(|ui| {
                            ui.label(format!("{:08X}", lookup[index].1));
                        });
                    });
                });
        });
    }
}
