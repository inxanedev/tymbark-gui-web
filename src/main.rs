#![windows_subsystem = "windows"]

mod logging;
mod tymbark;

use eframe::{egui::{self, Slider, Button, Label, Sense}, epi, epaint::Vec2};
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
    tymbark: String
}

impl Default for TymbarkGUI {
    fn default() -> TymbarkGUI {
        let args = Args::parse();

        TymbarkGUI {
            nouns: 2,
            generator: TymbarkGenerator::new(&args.file),
            tymbark: String::from("Tymbark jabłko mięta")
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
            ui.vertical_centered_justified(|ui| {
                let mut button_size = ui.available_size();
                button_size.y = 35.0;
                if ui.add_sized(button_size, Button::new("Generate")).clicked() {
                    self.tymbark = self.generator.generate(self.nouns);
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