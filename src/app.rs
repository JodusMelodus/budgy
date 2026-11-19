use egui::{Align, Key, KeyboardShortcut, Layout, Modifiers, Theme, vec2};
use polars::{error::PolarsError, frame::DataFrame, lazy::frame::LazyFrame};
use std::sync::mpsc::{Receiver, Sender, channel};
use thiserror::Error;

use crate::{
    file::{BudgyFileType, ImportFile, import_budget, import_statement, import_summary},
    tabs::Tabs,
};

pub const CTRL_E: KeyboardShortcut = KeyboardShortcut::new(Modifiers::CTRL, Key::E);
#[cfg(not(target_arch = "wasm32"))]
pub const ALT_F4: KeyboardShortcut = KeyboardShortcut::new(Modifiers::ALT, Key::F4);

#[derive(Debug, Error)]
pub enum BudgyError {
    #[error("Polars failed")]
    Polars(#[from] PolarsError),
    #[error("No DataFrame/LazyFrame")]
    MissingFrame(String),
}

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct Budget {
    categories: Vec<String>,
    amounts: Vec<f64>,
}

pub struct Budgy {
    pub(crate) budget_summary: Option<DataFrame>,

    pub(crate) budget_df: Option<DataFrame>,
    pub(crate) budget_lf: Option<LazyFrame>,

    pub(crate) statement_df: Option<DataFrame>,
    pub(crate) statement_lf: Option<LazyFrame>,

    pub(crate) previous_summary_lf: Option<LazyFrame>,

    pub(crate) current_tab: Tabs,

    // File picking
    pub(crate) tx: Sender<ImportFile>,
    rx: Receiver<ImportFile>,

    budget: Budget,
    error: Option<String>,

    pub(crate) nav_collapsed: bool,
    pub(crate) theme: Theme,
}

impl Budgy {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let (tx, rx) = channel();
        let budget = if let Some(storage) = cc.storage {
            eframe::get_value::<Budget>(storage, "budget").unwrap_or_default()
        } else {
            Budget::default()
        };

        Self {
            budget_summary: None,

            budget_df: None,
            budget_lf: None,

            statement_df: None,
            statement_lf: None,

            previous_summary_lf: None,

            current_tab: Tabs::Dashboard,

            tx,
            rx,

            budget,
            error: None,

            nav_collapsed: false,
            theme: cc.egui_ctx.system_theme().unwrap_or(Theme::Dark),
        }
    }
}

impl eframe::App for Budgy {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        if let Some(budget_df) = &self.budget_df {
            let category = budget_df
                .column("Category")
                .expect("Failed to get Category column")
                .str()
                .expect("Failed to convert to string")
                .into_iter()
                .map(|v| v.unwrap_or_default().to_owned())
                .collect::<Vec<String>>();
            let budget_amount = budget_df
                .column("Budget Amount")
                .expect("Failed to get Category column")
                .f64()
                .expect("Failed to convert to float")
                .into_iter()
                .map(|v| v.unwrap_or_default())
                .collect::<Vec<f64>>();

            self.budget.categories = category;
            self.budget.amounts = budget_amount;

            eframe::set_value(storage, "budget", &self.budget);
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.ctx().set_theme(self.theme);

        if ui.ctx().input_mut(|i| i.consume_shortcut(&CTRL_E)) {
            self.export_csv()
                .map_err(|e| self.error = Some(e.to_string()))
                .ok();
        }

        while let Ok(file) = self.rx.try_recv() {
            match file.file_type {
                BudgyFileType::Budget => {
                    self.budget_df = None;
                    self.budget_lf = import_budget(file.bytes)
                        .map_err(|e| self.error = Some(e.to_string()))
                        .ok();
                    self.budget.categories = Vec::new();
                    self.budget.amounts = Vec::new();
                }
                BudgyFileType::Statement => {
                    self.statement_df = None;
                    self.statement_lf = import_statement(file.bytes)
                        .map_err(|e| self.error = Some(e.to_string()))
                        .ok();
                }
                BudgyFileType::Summary => {
                    self.statement_df = None;
                    self.previous_summary_lf = import_summary(file.bytes)
                        .map_err(|e| self.error = Some(e.to_string()))
                        .ok();
                }
            }
        }

        self.menu(ui);

        self.navigation(ui);

        egui::CentralPanel::default().show(ui, |ui| {
            ui.spacing_mut().button_padding = vec2(8.0, 4.0);

            if let Err(e) = match self.current_tab {
                Tabs::Statement => self.display_statement_tab(ui),
                Tabs::BudgetSummary => self.display_budget_summary_tab(ui),
                Tabs::Budget => self.display_budget_tab(ui),
                Tabs::Dashboard => self.display_dashboard(ui),
                Tabs::Settings => self.display_settings(ui),
            } {
                match e {
                    BudgyError::Polars(e) => self.error = Some(format!("Polars Error: {e}")),
                    BudgyError::MissingFrame(e) => self.error = Some(format!("Missing Frame: {e}")),
                }
            }
        });

        if let Some(error) = self.error.clone() {
            egui::Modal::new("Error".into()).show(ui.ctx(), |ui| {
                ui.spacing_mut().button_padding = vec2(8.0, 4.0);

                ui.heading("Error");
                ui.separator();
                ui.label(&error);
                log::error!("Error: {error}");

                ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                    if ui.button("Ok").clicked() {
                        self.error = None;
                    }
                });
            });
        }
    }
}
