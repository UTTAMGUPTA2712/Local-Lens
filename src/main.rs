mod app;
mod db;
mod ml;
mod scanner;
mod ocr;
mod processing;

use app::ImageTagger;
use eframe::NativeOptions;

fn main() -> eframe::Result<()> {
    let options = NativeOptions::default();
    eframe::run_native(
        "Local Lens",
        options,
        Box::new(|cc| Box::new(ImageTagger::new(cc))),
    )
}
