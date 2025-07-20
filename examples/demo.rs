use eframe::egui;
use ferrugo::Ferrugo;

struct DemoApp {
    editor1: Ferrugo, // Default editor
    editor2: Ferrugo, // Editor with text
    editor3: Ferrugo, // Editor with path
    current_editor: usize,
}

impl Default for DemoApp {
    fn default() -> Self {
        // Set file path in your code
        let file_path = "/tmp/example.txt"; // File will be created if not exists. Tip! use crate named dirs in your project for better path management in file_path.

        Self {
            // Default empty editor
            editor1: Ferrugo::default(),

            // Editor with custom text
            editor2: Ferrugo::with_text(
                "This is a preloaded text!\n\nTry editing me..."
            ),

            // Editor loaded from file path
            editor3: Ferrugo::with_path(file_path),

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
            .with_title("Ferrugo Text Editor Demo - Multiple Initialization Methods"),
        ..Default::default()
    };

    eframe::run_native(
        "Ferrugo Text Editor Demo",
        options,
        Box::new(|_cc| Ok(Box::new(DemoApp::default()))),
    )
}