use polars::{
    frame::DataFrame, lazy::frame::IntoLazy as _, prelude::NamedFrom as _, series::Series,
};

use crate::app::{Budgy, BudgyError};

impl Budgy {
    pub fn display_budget_tab(&mut self, ui: &mut egui::Ui) -> Result<(), BudgyError> {
        ui.heading("Budget");
        ui.separator();

        if let Some(df) = &self.budget_df {
            Self::display_dataframe(ui, df);
        } else if let Some(lf) = self.budget_lf.clone() {
            self.budget_df = Some(lf.collect()?);
        } else {
            let category = Series::new("Category".into(), &self.budget.categories);
            let amount = Series::new("Budget Amount".into(), &self.budget.amounts);
            let df = DataFrame::new(vec![category.into(), amount.into()])?;
            self.budget_lf = Some(df.lazy());
        }

        Ok(())
    }
}
