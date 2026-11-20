use polars::{
    frame::DataFrame, lazy::frame::IntoLazy as _, prelude::NamedFrom as _, series::Series,
};

use crate::{
    app::{Budgy, BudgyError},
    extra_ui::DataFrameUi as _,
};

impl Budgy {
    pub fn display_budget_tab(&mut self, ui: &mut egui::Ui) -> Result<(), BudgyError> {
        ui.heading("Budget");
        ui.separator();

        if let Some(df) = &self.budget_df {
            ui.display_dataframe(df);
        } else if let Some(lf) = self.budget_lf.clone() {
            self.budget_df = Some(lf.collect()?);
        } else {
            let categories = &self.budget.categories;
            let amounts = &self.budget.amounts;

            let df = DataFrame::new(vec![
                Series::new("Category".into(), categories).into(),
                Series::new("Budget Amount".into(), amounts).into(),
            ])?;
            self.budget_df = Some(df.clone());
            self.budget_lf = Some(df.lazy());
        }

        Ok(())
    }
}
