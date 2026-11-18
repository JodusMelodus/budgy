use crate::{
    app::{Budgy, CTRL_E},
    extra_ui::ShortcutUi as _,
    file::BudgyFileType,
};

impl Budgy {
    pub fn file_menu(&self, ui: &mut egui::Ui) {
        ui.set_width(180.0);
        ui.spacing_mut().button_padding = egui::vec2(8.0, 4.0);

        if ui.button("Load previous summary").clicked() {
            self.open_csv_file(BudgyFileType::Summary);
        }

        if ui.button_with_shortcut("Export Summary", CTRL_E) {
            self.export_csv();
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            use crate::app::ALT_F4;

            ui.separator();

            if ui.button_with_shortcut("Exit", ALT_F4) {
                ui.send_viewport_cmd(egui::ViewportCommand::Close);
            }
        }
    }
}
