use std::vec;

use eframe::egui;
use egui_extras::{Size, TableBuilder};
use log::info;

use crate::chore::Chore;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct App {
    chores: Vec<Chore>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            chores: vec![
                Chore {
                    name: "Staubsaugen".to_owned(),
                    due: "Today".to_owned(),
                },
                Chore {
                    name: "Bad".to_owned(),
                    due: "Tomorrow".to_owned(),
                },
            ],
        }
    }
}

impl App {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for App {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            egui::warn_if_debug_build(ui);

            TableBuilder::new(ui)
                .striped(true)
                .column(Size::at_least(Size::relative(0.3), 100.0))
                .column(Size::at_least(Size::relative(0.3), 100.0))
                .column(Size::remainder())
                .header(20.0, |mut header| {
                    header.col(|ui| {
                        ui.heading("Chore");
                    });
                    header.col(|ui| {
                        ui.heading("Due");
                    });
                    header.col(|ui| {
                        ui.heading("Actions");
                    });
                })
                .body(|mut body| {
                    for chore in self.chores.iter() {
                        body.row(30.0, |mut row| {
                            row.col(|ui| {
                                ui.label(format!("{}", chore.name));
                            });
                            row.col(|ui| {
                                ui.label(format!("{}", chore.due));
                            });
                            row.col(|ui| {
                                if ui.button("done").clicked() {
                                    info!("Done button clicked");
                                }
                            });
                        });
                    }
                });
        });
    }
}
