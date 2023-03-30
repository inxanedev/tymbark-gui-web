#![windows_subsystem = "windows"]

mod tymbark;
mod history;

use eframe::{egui::{self, Slider, Button}};
use egui::{Label, Sense};
use history::History;
use tymbark::TymbarkGenerator;
use web_sys::{window};

struct TymbarkGUI {
    nouns: u32,
    generator: TymbarkGenerator,
    tymbark: String,
    history: History,
}

static FOOD_LIST: &str = include_str!("../food.txt");

impl Default for TymbarkGUI {
    fn default() -> TymbarkGUI {
        let default_tymbark = String::default();

        TymbarkGUI {
            nouns: 2,
            generator: TymbarkGenerator::new(FOOD_LIST),
            tymbark: default_tymbark,
            history: History::new(5)
        }
    }
}

impl eframe::App for TymbarkGUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.heading("Tymbark Generator");
            });
            ui.separator();
            ui.add_space(5.0);
            ui.vertical_centered_justified(|ui| {
                ui.heading("Options");
            });
            ui.add_space(5.0);
            ui.horizontal(|ui| {
                ui.label("Noun amount:");
                ui.add(Slider::new(&mut self.nouns, 1..=25));
            });
            ui.add_space(5.0);
            ui.separator();
            ui.add_space(5.0);
            ui.horizontal(|ui| {
                let row_size = ui.available_width() - 3.0 * 5.0;
                let back_size = row_size * 0.1;
                let generate_size = row_size * 0.8;
                let next_size = row_size * 0.1;
                
                let height = 35.0;

                if ui.add_sized([back_size, height], Button::new("<")).clicked() {
                    self.history.back();
                    match self.history.get() {
                        Some(s) => self.tymbark = s.clone(),
                        None => self.history.reset_index()
                    }
                }
                if ui.add_sized([generate_size, height], Button::new("Generate")).clicked() {
                    self.tymbark = self.generator.generate(self.nouns);
                    self.history.add(self.tymbark.clone());
                    self.history.reset_index();
                }
                if ui.add_sized([next_size, height], Button::new(">")).clicked() {
                    self.history.next();
                    match self.history.get() {
                        Some(s) => self.tymbark = s.clone(),
                        None => self.history.reset_index()
                    }
                }
            });

            ui.add_space(10.0);

            ui.vertical_centered_justified(|ui| {
                let display = ui.add_sized(ui.available_size(), Label::new(self.tymbark.as_str()).sense(Sense::click()));
                if display.on_hover_text("Click me to copy!").clicked() {
                    let _ = window().unwrap().navigator().clipboard().unwrap().write_text(self.tymbark.as_str());
                }
            });
        });
    }
}

fn main() {
    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::start_web(
            "canvas", // hardcode it
            web_options,
            Box::new(|_cc| Box::new(TymbarkGUI::default())),
        )
        .await
        .expect("failed to start eframe");
    });
}