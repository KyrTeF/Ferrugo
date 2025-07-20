use eframe::egui;
use crate::editor::Ferrugo;
use std::env;

struct DemoApp {
    editor1: Ferrugo, // Default editor
    editor2: Ferrugo, // Editor with text
    editor3: Ferrugo, // Editor with path
    current_editor: usize,
}

impl Default for DemoApp {
    fn default() -> Self {
        // Get first command line argument as file path
        let file_path = env::args().nth(1);

        Self {
            // Default empty editor
            editor1: Ferrugo::default(),

            // Editor with custom text
            editor2: Ferrugo::with_text(
                "This is a preloaded text!\n\nTry editing me..."
            ),

            // Editor loaded from file path
            editor3: match file_path {
                Some(path) => Ferrugo::with_path(&path),
                None => Ferrugo::with_text(
                    "No file path provided\n\nRun with: cargo run --example demo -- path/to/file.txt"
                ),
            },

            // Start with editor1
            current_editor: 0,
        }
    }
}

impl eframe::App for DemoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Editor selection tabs
            ui.horizontal(|ui| {
                if ui.selectable_label(self.current_editor == 0, "Empty Editor").clicked() {
                    self.current_editor = 0;
                }
                if ui.selectable_label(self.current_editor == 1, "Text Editor").clicked() {
                    self.current_editor = 1;
                }
                if ui.selectable_label(self.current_editor == 2, "File Editor").clicked() {
                    self.current_editor = 2;
                }
            });

            ui.separator();

            // Status information
            ui.horizontal(|ui| {
                let editor = match self.current_editor {
                    0 => &self.editor1,
                    1 => &self.editor2,
                    _ => &self.editor3,
                };

                if let Some(path) = &editor.file_path {
                    ui.label(format!("Editing: {}", path.display()));
                } else {
                    ui.label("Editing: New Document");
                }

                ui.label("|");
                ui.label(format!("Chars: {}", editor.text.chars().count()));
                ui.label("|");
                ui.label(format!("Lines: {}", editor.text.lines().count()));
            });

            ui.separator();

            // Show selected editor
            match self.current_editor {
                0 => self.editor1.show(ui),
                1 => self.editor2.show(ui),
                _ => self.editor3.show(ui),
            }
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 700.0])
            .with_title("Ferrugo Editor Demo - Multiple Initialization Methods"),
        ..Default::default()
    };

    eframe::run_native(
        "Ferrugo Editor Demo",
        options,
        Box::new(|_cc| Ok(Box::new(DemoApp::default()))),
    )
}