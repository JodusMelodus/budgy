use std::io::Cursor;

use polars::{
    error::PolarsError,
    frame::DataFrame,
    io::{
        SerReader as _, SerWriter as _,
        csv::{
            read::{CsvParseOptions, CsvReadOptions},
            write::CsvWriter,
        },
    },
    lazy::{
        dsl::col,
        frame::{IntoLazy as _, LazyFrame},
    },
};
use rfd::FileHandle;
use thiserror::Error;

use crate::app::Budgy;

pub enum BudgyFileType {
    Budget,
    Statement,
    Summary,
}

pub struct ImportFile {
    pub file_type: BudgyFileType,
    pub bytes: Vec<u8>,
}

#[derive(Debug, Error)]
pub enum BudgyFileError {
    #[error("Budget file is empty")]
    Empty,
    #[error("Failed to read CSV data")]
    Polars(#[from] PolarsError),
    #[error("Failed to write file")]
    File(#[from] std::io::Error),
}

#[cfg(target_arch = "wasm32")]
pub fn execute<F: std::future::Future<Output = ()> + 'static>(f: F) {
    wasm_bindgen_futures::spawn_local(f);
}

#[cfg(not(target_arch = "wasm32"))]
pub fn execute<F: std::future::Future<Output = ()> + Send + 'static>(f: F) {
    std::thread::spawn(move || pollster::block_on(f));
}

fn import_csv_file(bytes: Vec<u8>) -> Result<LazyFrame, BudgyFileError> {
    if bytes.is_empty() {
        return Err(BudgyFileError::Empty);
    }

    Ok(CsvReadOptions::default()
        .with_has_header(true)
        .with_parse_options(CsvParseOptions::default().with_try_parse_dates(true))
        .into_reader_with_file_handle(Cursor::new(bytes))
        .finish()?
        .lazy())
}

pub fn import_statement(bytes: Vec<u8>) -> Result<LazyFrame, BudgyFileError> {
    let mut lazy_frame = import_csv_file(bytes)?;

    lazy_frame = lazy_frame.filter(
        col("Category")
            .is_not_null()
            .and(col("Parent Category").is_not_null()),
    );

    Ok(lazy_frame)
}

pub fn import_budget(bytes: Vec<u8>) -> Result<LazyFrame, BudgyFileError> {
    import_csv_file(bytes)
}

pub fn import_summary(bytes: Vec<u8>) -> Result<LazyFrame, BudgyFileError> {
    import_csv_file(bytes)
}

pub async fn export_csv(
    file: &FileHandle,
    data_frame: &mut DataFrame,
) -> Result<(), BudgyFileError> {
    let mut buffer = Vec::new();

    CsvWriter::new(Cursor::new(&mut buffer))
        .include_header(true)
        .with_separator(b',')
        .finish(data_frame)?;

    file.write(&buffer).await?;
    Ok(())
}

impl Budgy {
    pub fn open_csv_file(&self, file_type: BudgyFileType) {
        let tx = self.tx.clone();

        execute(async move {
            let Some(file) = rfd::AsyncFileDialog::new()
                .add_filter("CSV Files", &["csv"])
                .pick_file()
                .await
            else {
                log::error!("Failed to read CSV file");
                return;
            };

            let bytes = file.read().await;
            if let Err(e) = tx.send(super::ImportFile { file_type, bytes }) {
                log::error!("Failed to send loaded file data: {e}");
            }
        });
    }

    pub fn export_csv(&self) {
        let Some(budget_summary_df) = self.budget_summary.clone() else {
            log::error!("No budget summary available");
            return;
        };

        let Some(mut summary) = budget_summary_df
            .lazy()
            .select([col("Category"), col("Balance")])
            .collect()
            .ok()
        else {
            log::error!("Failed to serialize CSV file");
            return;
        };

        execute(async move {
            if let Some(file) = rfd::AsyncFileDialog::new()
                .set_title("Export summary")
                .set_file_name("summary.csv")
                .save_file()
                .await
            {
                export_csv(&file, &mut summary)
                    .await
                    .expect("Failed to save file");
            }
        });
    }
}
