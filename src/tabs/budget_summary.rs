use polars::{chunked_array::ops::SortMultipleOptions, datatypes::DataType, lazy::dsl::col};

use crate::{
    app::{Budgy, BudgyError},
    extra_ui::DataFrameUi as _,
};

impl Budgy {
    pub fn display_budget_summary_tab(&mut self, ui: &mut egui::Ui) -> Result<(), BudgyError> {
        ui.heading("Summary");
        ui.separator();

        let Some(df) = &self.budget_summary else {
            if ui.button("Generate").clicked() {
                let mut statement = self
                    .statement_lf
                    .clone()
                    .ok_or_else(|| BudgyError::MissingFrame("Statement".to_owned()))?;
                statement = statement.with_columns([
                    col("Money In").cast(DataType::Float64).fill_null(0),
                    col("Money Out").cast(DataType::Float64).fill_null(0),
                    col("Fee").cast(DataType::Float64).fill_null(0),
                ]);

                let budget = self
                    .budget_lf
                    .clone()
                    .ok_or_else(|| BudgyError::MissingFrame("Budget".to_owned()))?;

                statement = statement.group_by([col("Category")]).agg([
                    col("Money In").sum(),
                    col("Money Out").sum(),
                    col("Fee").sum(),
                ]);

                let mut result = statement.left_join(budget, col("Category"), col("Category"));
                result =
                    result.with_column(col("Budget Amount").cast(DataType::Float64).fill_null(0));
                result = result.sort(["Category"], SortMultipleOptions::default());

                result = result.with_column(
                    (col("Money In") - col("Money Out") + col("Fee") - col("Budget Amount"))
                        .alias("Balance"),
                );

                self.budget_summary = Some(result.collect()?);
            }
            return Ok(());
        };

        ui.display_dataframe(df);
        Ok(())
    }
}
