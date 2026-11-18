use polars::{chunked_array::ops::SortMultipleOptions, lazy::dsl::col};

use crate::{
    app::{Budgy, BudgyError},
    extra_ui::DataFrameUi as _,
};

impl Budgy {
    pub fn display_budget_summary_tab(&mut self, ui: &mut egui::Ui) -> Result<(), BudgyError> {
        ui.heading("Summary");
        ui.separator();

        let Some(df) = &mut self.budget_summary else {
            let mut summary = self
                .previous_summary_lf
                .clone()
                .ok_or_else(|| BudgyError::MissingFrame("Previous summary".to_owned()))?;

            let statement = self
                .statement_lf
                .clone()
                .ok_or_else(|| BudgyError::MissingFrame("Statement".to_owned()))?;

            let budget = self
                .budget_lf
                .clone()
                .ok_or_else(|| BudgyError::MissingFrame("Budget".to_owned()))?;

            summary = summary.rename(["Balance"], ["Previous Balance"], true);
            let mut result = summary.left_join(statement, col("Category"), col("Category"));
            result = result
                .group_by([col("Category")])
                .agg([
                    col("Money In").sum(),
                    col("Money Out").sum(),
                    col("Fee").sum(),
                    col("Previous Balance").max(),
                ])
                .left_join(budget, col("Category"), col("Category"));
            result = result.with_column(
                (col("Money In") + col("Money Out") + col("Fee") - col("Budget Amount")
                    + col("Previous Balance"))
                .alias("Balance"),
            );

            let res = result
                .sort(["Category"], SortMultipleOptions::default())
                .collect()?;
            self.budget_summary = Some(res);
            return Ok(());
        };

        ui.display_dataframe(df);
        Ok(())
    }
}
