//! Embeddable text editor component
// Import necessary libraries
use eframe::egui; // GUI library
use std::fs; // File system operations
use std::path::{Path, PathBuf}; // Platform-independent path handling

/// Text editor with file operations
pub struct Ferrugo {
    /// Current text content
    pub text: String, // Stores the current text content (By default none if new/unsaved)
    /// Current file path
    pub file_path: Option<PathBuf>, // Stores the current file path (By default none if new/unsaved)
}

// Implement default values
impl Default for Ferrugo {
    /// Create a new empty editor
    fn default() -> Self { // Sets default values for text and file_path
        Self {
            text: String::new(), // Start with empty text
            file_path: None, // No file path initially
        }
    }
}

// Implement new functions
impl Ferrugo {
    /// Create editor with initial text
    pub fn with_text(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            file_path: None,
        }
    }

    /// Create editor with initial text loaded from a file path
    pub fn with_path(path: impl AsRef<Path>) -> Self {
        let path_buf = path.as_ref().to_path_buf();
        let text = fs::read_to_string(&path_buf)
            .unwrap_or_default();

        Self {
            text, // Store file contents found in path
            file_path: Some(path_buf), // Store path
        }
    }

    /// Open file via dialog
    pub fn open_file(&mut self) {
        // Show open file dialog
        if let Some(path) = rfd::FileDialog::new().pick_file() {
            // Attempt to read file contents
            if let Ok(contents) = fs::read_to_string(&path) {
                self.text = contents; // Store file contents
                self.file_path = Some(path); // Store file path
            }
        }
    }

    /// Save to current opened file
    pub fn save_file(&mut self) {
        // Check if it has a file path
        if let Some(path) = &self.file_path {
            // Write text to file
            let _ = fs::write(path, &self.text);
        } else {
            // If it has no path, use Save as
            self.save_file_as();
        }
    }

    /// Save via "Save as" dialog
    // Save file with new name/path
    pub fn save_file_as(&mut self) {
        // Show save file dialog
        if let Some(path) = rfd::FileDialog::new().save_file() {
            // Write text to new file
            let _ = fs::write(&path, &self.text);
            // Store new file path(Thus user edits newly saved file)
            self.file_path = Some(path);
        }
    }

    /// Render editor UI
    pub fn show(&mut self, ui: &mut egui::Ui) {
        // Action buttons inside horizontal ui at top
        ui.horizontal(|ui| {
            if ui.button("📂 Open").clicked() {
                self.open_file();
            }
            if ui.button("💾 Save").clicked() {
                self.save_file();
            }
            if ui.button("💾 Save As").clicked() {
                self.save_file_as();
            }
            // Status information
            ui.horizontal(|ui| {
                ui.label("|");
                ui.label(format!("Chars: {}", self.text.chars().count())); // Display number of Characters
                ui.label("|");
                ui.label(format!("Lines: {}", self.text.lines().count())); // Display number of Lines
                ui.label("|");

                // Display file name and path
                if let Some(path) = &self.file_path {
                    ui.label(format!("Editing: {}", path.display()));
                } else {
                    ui.label("Editing: New Document");
                }
            });
        });

        // Text edit area with scrollbars
        egui::ScrollArea::both()
            .auto_shrink([false; 2]) // Keep scrollbar always visible
            .show(ui, |ui| {
                // Add multiline text editor
                ui.add(
                    egui::TextEdit::multiline(&mut self.text)
                        .desired_width(f32::INFINITY) // Expand to full width
                );
            });
    }
}