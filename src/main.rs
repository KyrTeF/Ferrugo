// Import necessary libraries
use eframe::egui; // GUI library
use std::fs; // File system operations
use std::path::PathBuf; // Platform-independent path handling

// Define the main application structure
struct Ferrugo {
    text: String,            // Stores the current text content
    file_path: Option<PathBuf>, // Stores the current file path (None if new/unsaved)
}

// Implement default values for the application
impl Default for Ferrugo {
    fn default() -> Self {
        Self {
            text: String::new(),  // Start with empty text
            file_path: None,      // No file path initially
        }
    }
}

// Implement application-specific methods
impl Ferrugo {
    // Open a file using a file dialog
    fn open_file(&mut self) {
        // Show file picker dialog
        if let Some(path) = rfd::FileDialog::new().pick_file() {
            // Attempt to read file contents
            if let Ok(contents) = fs::read_to_string(&path) {
                self.text = contents;     // Store file contents
                self.file_path = Some(path); // Store file path
            }
        }
    }

    // Save the current file
    fn save_file(&mut self) {
        // Check if we have a file path
        if let Some(path) = &self.file_path {
            // Write text to file (ignore errors)
            let _ = fs::write(path, &self.text);
        } else {
            // If no path, use Save As
            self.save_file_as();
        }
    }

    // Save file with new name/path
    fn save_file_as(&mut self) {
        // Show save file dialog
        if let Some(path) = rfd::FileDialog::new().save_file() {
            // Write text to new file
            let _ = fs::write(&path, &self.text);
            // Store new file path
            self.file_path = Some(path);
        }
    }
}

// Implement the eframe application interface
impl eframe::App for Ferrugo {
    // Main update loop called every frame
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Create central panel (main content area)
        egui::CentralPanel::default().show(ctx, |ui| {
            // Create horizontal layout for buttons
            ui.horizontal(|ui| {
                // Open button
                if ui.button("Open").clicked() {
                    self.open_file(); // Handle click
                }
                // Save button
                if ui.button("Save").clicked() {
                    self.save_file(); // Handle click
                }
                // Save As button
                if ui.button("Save As").clicked() {
                    self.save_file_as(); // Handle click
                }
            });

            // Create text area with scrollbar
            egui::ScrollArea::both()
                .auto_shrink([false; 2]) // Keep scrollbar always visible
                .show(ui, |ui| {
                    // Add multiline text editor
                    ui.add(
                        egui::TextEdit::multiline(&mut self.text)
                            .desired_width(f32::INFINITY) // Expand to full width
                    );
                });
        });
    }
}

// Main entry point
fn main() -> eframe::Result<()> {
    // Configure window options
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0]) // Initial window size
            .with_title("Ferrugo"), // Window title
        ..Default::default() // Other options use defaults
    };

    // Start the native GUI
    eframe::run_native(
        "Ferrugo", // App name
        options,               // Window options
        // Create app instance
        Box::new(|_cc| Ok(Box::new(Ferrugo::default()))),
    )
}