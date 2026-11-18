use crate::app::{Budgy, BudgyError, BudgyFileType};

impl Budgy {
    pub fn display_statement_tab(&mut self, ui: &mut egui::Ui) -> Result<(), BudgyError> {
        ui.heading("Statement");
        ui.separator();

        if let Some(df) = &self.statement_df {
            Self::display_dataframe(ui, df);
        } else if let Some(lf) = self.statement_lf.clone() {
            self.statement_df = Some(lf.collect()?);
        } else if ui.button("Import statement").clicked() {
            self.open_csv_file(BudgyFileType::Statement);
        }

        Ok(())
    }
}
