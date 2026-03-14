mod config;
mod core;
mod i18n;
mod mover;
mod scanner;
mod ui;
mod utils;

use eframe::NativeOptions;
use ui::main_window::MainWindow;

fn main() -> eframe::Result {
    let _log_guard = utils::logger::init_logging();

    tracing::info!("DevHome Relocator starting");

    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("DevHome Relocator")
            .with_inner_size([1300.0, 700.0])
            .with_min_inner_size([1300.0, 630.0]),
        ..Default::default()
    };

    eframe::run_native(
        "DevHome Relocator",
        options,
        Box::new(|cc| Ok(Box::new(MainWindow::new(cc)))),
    )
}
