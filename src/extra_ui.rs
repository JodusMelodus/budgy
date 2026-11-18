use egui::{KeyboardShortcut, Ui};

pub trait ExtraUi {
    fn button_with_shortcut(
        &mut self,
        text: impl Into<egui::WidgetText>,
        shortcut: KeyboardShortcut,
    ) -> bool;
}

impl ExtraUi for Ui {
    fn button_with_shortcut(
        &mut self,
        text: impl Into<egui::WidgetText>,
        shortcut: KeyboardShortcut,
    ) -> bool {
        let button = egui::Button::new(text).shortcut_text(self.ctx().format_shortcut(&shortcut));
        self.add(button).clicked()
    }
}

pub trait DataFrameUi {
    fn display_dataframe(&mut self, df: &polars::frame::DataFrame);
}

impl DataFrameUi for egui::Ui {
    fn display_dataframe(&mut self, df: &polars::frame::DataFrame) {
        let height = df.height();
        let column_names = df.get_column_names();

        egui::ScrollArea::horizontal().show(self, |ui| {
            egui_extras::TableBuilder::new(ui)
                .striped(true)
                .resizable(true)
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .columns(egui_extras::Column::auto(), column_names.len())
                .header(20.0, |mut header| {
                    for name in &column_names {
                        header.col(|ui| {
                            ui.strong(name.to_string());
                        });
                    }
                })
                .body(|body| {
                    body.rows(22.0, height, |mut row| {
                        let row_idx = row.index();

                        for column_name in &column_names {
                            row.col(|ui| {
                                if let Ok(column) = df.column(column_name) {
                                    let value = column
                                        .get(row_idx)
                                        .unwrap_or(polars::datatypes::AnyValue::Null);
                                    ui.label(format!("{value}"));
                                }
                            });
                        }
                    });
                });
        });
    }
}
