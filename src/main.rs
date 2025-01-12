mod image_processing;
mod ui;

use crate::ui::MyApp;

fn main() -> Result<(), eframe::Error> {
    eframe::run_native(
        "Image Viewer",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}
