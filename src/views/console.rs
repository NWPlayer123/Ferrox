pub struct ConsoleTab;

impl ConsoleTab {
    pub fn update(&mut self, ui: &mut egui::Ui) {
        // This is currently placeholder data. TODO: implement Log and a sender/receiver so we can just use
        // normal warn! macros and it'll get piped to here?
        ui.label("Test of the Ferrox output console!");
        ui.label("This needs to actually be hooked up somehow to a global state.");
    }
}
