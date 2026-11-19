use crate::{
    app::{Budgy, BudgyError},
    extra_ui::DataFrameUi as _,
    file::BudgyFileType,
};

impl Budgy {
    pub fn display_budget_tab(&mut self, ui: &mut egui::Ui) -> Result<(), BudgyError> {
        ui.heading("Budget");
        ui.separator();

        if let Some(df) = &self.budget_df {
            ui.display_dataframe(df);
        } else if let Some(lf) = self.budget_lf.clone() {
            self.budget_df = Some(lf.collect()?);
        } else if ui.button("Import Budget").clicked() {
            self.open_csv_file(BudgyFileType::Budget);
        }

        Ok(())
    }
}
