use std::{
    fs::File,
    io::{BufRead, Cursor},
    path::Path,
};

use flexi_logger::LogSpecification;
use rev_buf_reader::RevBufReader;
use slint::VecModel;

slint::include_modules!();

const MATAURI_BAY_LOGS: &str = "/data/pattern/.config/matauri-bay/log/logs.txt";

fn main() -> eyre::Result<()> {
    // Ignore if the Logger fails to initialize, we can signal in application later on
    let _ = flexi_logger::Logger::with(LogSpecification::debug()).start();

    let ui = AppWindow::new()?;
    let log_path = Path::new(MATAURI_BAY_LOGS);
    let results = File::open(log_path)
        .map(RevBufReader::new)
        .map(|buf| {
            buf.lines()
                .map_while(Result::ok)
                .take(100)
                .map(|s| slint::format!("{}", s))
                .collect::<Vec<_>>()
        })
        .unwrap_or_else(|e| {
            log::error!("Failed to open file at {MATAURI_BAY_LOGS}:\n{e}");
            let lines = RevBufReader::new(Cursor::new(include_bytes!(
                "../extra/mini-logs.txt"
            )));
            lines
                .lines()
                .take(150)
                .map(|s| slint::format!("{}", s.unwrap()))
                .collect::<Vec<_>>()
        });
    let log_files = VecModel::from_slice(&results);
    ui.global::<Directories>().set_log_files(log_files);

    ui.on_request_increase_value({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.set_counter(ui.get_counter() + 1);
        }
    });

    ui.run()?;
    Ok(())
}
