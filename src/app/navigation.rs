use egui::{Align, Layout, vec2};

use crate::app::{Budgy, tabs::Tabs};

trait NavigationUi {
    fn navigation_option<'a>(
        &mut self,
        icon: impl egui::IntoAtoms<'a>,
        on_hover_text: impl Into<egui::WidgetText>,
        selected: bool,
    ) -> egui::Response;
}

impl NavigationUi for egui::Ui {
    fn navigation_option<'a>(
        &mut self,
        icon: impl egui::IntoAtoms<'a>,
        on_hover_text: impl Into<egui::WidgetText>,
        selected: bool,
    ) -> egui::Response {
        self.add_sized(
            egui::vec2(self.available_width(), 30.0),
            egui::Button::new(icon).selected(selected),
        )
        .on_hover_text(on_hover_text)
    }
}

impl Budgy {
    pub fn navigation(&mut self, ui: &mut egui::Ui) {
        egui::Panel::left("navigation")
            .exact_size(if self.nav_collapsed { 70.0 } else { 180.0 })
            .show_separator_line(false)
            .resizable(false)
            .show(ui, |ui| {
                self.display_navigation_menu(ui);
            });
    }

    pub fn display_navigation_menu(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.spacing_mut().button_padding = egui::vec2(12.0, 3.0);

            ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                if ui
                    .add_sized(
                        vec2(70.0 - ui.spacing().button_padding.x, 30.0),
                        egui::Button::new(if self.nav_collapsed {
                            "▶️"
                        } else {
                            "◀️"
                        }),
                    )
                    .on_hover_text(if self.nav_collapsed {
                        "Expand sidebar"
                    } else {
                        "Collapse sidebar"
                    })
                    .clicked()
                {
                    self.nav_collapsed = !self.nav_collapsed;
                }
            });

            if ui
                .navigation_option(
                    if self.nav_collapsed {
                        "🏠"
                    } else {
                        "Dashboard"
                    },
                    "Dashboard",
                    self.current_tab == Tabs::Dashboard,
                )
                .clicked()
            {
                self.current_tab = Tabs::Dashboard;
            }

            if ui
                .navigation_option(
                    if self.nav_collapsed {
                        "📄"
                    } else {
                        "Statement"
                    },
                    "Statement",
                    self.current_tab == Tabs::Statement,
                )
                .clicked()
            {
                self.current_tab = Tabs::Statement;
            }

            if ui
                .navigation_option(
                    if self.nav_collapsed { "💳" } else { "Budget" },
                    "Budget",
                    self.current_tab == Tabs::Budget,
                )
                .clicked()
            {
                self.current_tab = Tabs::Budget;
            }

            if ui
                .navigation_option(
                    if self.nav_collapsed {
                        "📨"
                    } else {
                        "Summary"
                    },
                    "Summary",
                    self.current_tab == Tabs::BudgetSummary,
                )
                .clicked()
            {
                self.current_tab = Tabs::BudgetSummary;
            }

            if ui
                .navigation_option(
                    if self.nav_collapsed {
                        "⚙️"
                    } else {
                        "Settings"
                    },
                    "Settings",
                    self.current_tab == Tabs::Settings,
                )
                .clicked()
            {
                self.current_tab = Tabs::Settings;
            }
        });
    }
}
