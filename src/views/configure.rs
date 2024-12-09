use egui_extras::{Column, TableBuilder};

use crate::{BinaryFormat, ProcessorType};

#[derive(Debug, Default)]
pub enum ImportState {
    #[default]
    Waiting,
    Configured,
    Cancelled,
}

pub struct ImportWindow {
    supported_formats: Vec<(&'static str, BinaryFormat)>,
    supported_processors: Vec<(&'static str, ProcessorType)>,
}

impl ImportWindow {
    pub fn new(
        supported_formats: Vec<(&'static str, BinaryFormat)>,
        supported_processors: Vec<(&'static str, ProcessorType)>,
    ) -> Self {
        Self { supported_formats, supported_processors }
    }

    // TODO: auto-detect format based on input data, needs to be tokio task so we don't bog down the update fn
    pub fn update(
        &mut self, ui: &mut egui::Ui, format: &mut BinaryFormat, processor: &mut ProcessorType,
    ) -> ImportState {
        ui.heading("Binary Format");
        ui.add_space(5.0);
        ui.push_id("binary_format_selector", |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                TableBuilder::new(ui)
                    .striped(true)
                    .sense(egui::Sense::click())
                    .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                    .column(Column::remainder().clip(true))
                    .body(|body| {
                        body.rows(20.0, self.supported_formats.len(), |mut row| {
                            let row_index = row.index();

                            row.set_selected(*format == self.supported_formats[row_index].1);

                            row.col(|ui| {
                                egui::Frame::none().outer_margin(1.0).show(ui, |ui| {
                                    ui.label(self.supported_formats[row_index].0);
                                });
                            });

                            if row.response().clicked() {
                                *format = self.supported_formats[row_index].1;
                            }
                        });
                    });
            });
        });

        ui.add_space(8.0);

        ui.heading("Processor Type");
        ui.add_space(5.0);
        ui.push_id("processor_type_selector", |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                TableBuilder::new(ui)
                    .striped(true)
                    .sense(egui::Sense::click())
                    .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                    .column(Column::remainder().clip(true))
                    .body(|body| {
                        body.rows(20.0, self.supported_processors.len(), |mut row| {
                            let row_index = row.index();

                            row.set_selected(*processor == self.supported_processors[row_index].1);

                            row.col(|ui| {
                                egui::Frame::none().outer_margin(1.0).show(ui, |ui| {
                                    ui.label(self.supported_processors[row_index].0);
                                });
                            });

                            if row.response().clicked() {
                                *processor = self.supported_processors[row_index].1;
                            }
                        });
                    });
            });
        });

        ui.add_space(8.0);

        let mut import_state = ImportState::Waiting;
        ui.horizontal(|ui| {
            if ui.button("Import").clicked() {
                import_state = ImportState::Configured;
            }

            if ui.button("Cancel").clicked() {
                import_state = ImportState::Cancelled;
            }
        });

        ui.add_space(1.0);

        import_state
    }
}
