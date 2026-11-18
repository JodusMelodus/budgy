use crate::app::Budgy;

pub mod file;
pub mod help;

impl Budgy {
    pub fn menu(&self, ui: &mut egui::Ui) {
        egui::Panel::top("Menu")
            .show_separator_line(false)
            .show(ui, |ui| {
                egui::MenuBar::new().ui(ui, |ui| {
                    ui.spacing_mut().button_padding = egui::vec2(8.0, 4.0);

                    ui.menu_button("File", |ui| {
                        self.file_menu(ui);
                    });

                    ui.menu_button("Help", |ui| {
                        Self::help_menu(ui);
                    });
                });
            });
    }
}
