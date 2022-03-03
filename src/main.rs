#![windows_subsystem = "windows"]

mod logging;
mod tymbark;
mod history;

use eframe::{egui::{self, Slider, Button, Label, Sense}, epi, epaint::Vec2};
use history::History;
use tymbark::TymbarkGenerator;

use clap::Parser;

#[derive(Parser)]
struct Args {
    #[clap(short, long, default_value_t = String::from("food.txt"))]
    file: String
}

struct TymbarkGUI {
    nouns: u32,
    generator: TymbarkGenerator,
    tymbark: String,
    history: History,
}

impl Default for TymbarkGUI {
    fn default() -> TymbarkGUI {
        let args = Args::parse();

        let default_tymbark = String::default();

        TymbarkGUI {
            nouns: 2,
            generator: TymbarkGenerator::new(&args.file),
            tymbark: default_tymbark,
            history: History::new(5)
        }
    }
}

impl epi::App for TymbarkGUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {
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
                    clipboard_win::set_clipboard(clipboard_win::formats::Unicode, self.tymbark.as_str()).unwrap();
                }
            });
        });
    }

    fn name(&self) -> &str {
        "Tymbark GUI"
    }
}

fn main() {
    logging::setup_logger();

    let options = eframe::NativeOptions {
        initial_window_size: Some(Vec2::new(400.0, 300.0)),
        ..Default::default()
    };
    eframe::run_native(Box::new(TymbarkGUI::default()), options);
}