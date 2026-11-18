use egui::vec2;

use crate::app::Budgy;

impl Budgy {
    pub fn help_menu(ui: &mut egui::Ui) {
        ui.set_width(180.0);
        ui.spacing_mut().button_padding = vec2(8.0, 4.0);

        #[cfg(not(target_arch = "wasm32"))]
        if ui.button("Open WebApp").clicked() {
            Self::open_web_app();
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn open_web_app() {
        if let Err(e) = webbrowser::open("https://jodusmelodus.github.io/budgy/") {
            log::error!("Failed to open web link: {e}");
        }
    }
}
