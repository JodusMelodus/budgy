use egui::{KeyboardShortcut, Ui};

pub trait ShortcutUi {
    fn button_with_shortcut(
        &mut self,
        text: impl Into<egui::WidgetText>,
        shortcut: KeyboardShortcut,
    ) -> bool;
}

impl ShortcutUi for Ui {
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
        let columns = df.get_columns();

        egui::ScrollArea::horizontal().show(self, |ui| {
            egui_extras::TableBuilder::new(ui)
                .striped(true)
                .resizable(true)
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .columns(egui_extras::Column::auto(), columns.len())
                .header(20.0, |mut header| {
                    for col in columns {
                        header.col(|ui| {
                            ui.strong(col.name().as_str());
                        });
                    }
                })
                .body(|body| {
                    body.rows(22.0, height, |mut row| {
                        let row_idx = row.index();

                        for column in columns {
                            row.col(|ui| {
                                let value = column
                                    .get(row_idx)
                                    .unwrap_or(polars::datatypes::AnyValue::Null);
                                ui.label(value.to_string());
                            });
                        }
                    });
                });
        });
    }
}
