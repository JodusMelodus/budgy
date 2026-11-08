use crate::app::{Budgy, BudgyError};

impl Budgy {
    #[expect(clippy::unnecessary_wraps)]
    #[expect(clippy::unused_self)]
    pub fn display_dashboard(&self, ui: &mut egui::Ui) -> Result<(), BudgyError> {
        ui.heading("Dashboard");
        ui.separator();
        Ok(())
    }
}
