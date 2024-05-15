mod mount;

use std::{
    fs::File,
    io::{BufRead, Cursor},
    path::Path,
};

use flexi_logger::LogSpecification;
use rev_buf_reader::RevBufReader;
use slint::VecModel;

slint::include_modules!();

/// Default locaion of the logs for matauri-bay
const MATAURI_BAY_LOGS: &str = "/data/pattern/.config/matauri-bay/log/logs.txt";

/// Load logs from default matauri-bay log location. In case the logs don't exist (ie we are on a different platform)
/// load an example file for illustration
fn load_logs() -> Vec<slint::SharedString> {
    let log_path = Path::new(MATAURI_BAY_LOGS);
    File::open(log_path)
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
        })
}

fn main() -> eyre::Result<()> {
    // Ignore if the Logger fails to initialize, we can signal in application later on
    if let Err(e) =
        flexi_logger::Logger::with(LogSpecification::debug()).start()
    {
        eprintln!("Failed to initialize logger: {e}");
    }


    mount::mount();

    // let mounted = Mount::new("/dev/sda1", "/mnt/usb-drive");
    // match mounted {
    //     Ok(m) => {
    //         m.into_unmount_drop(UnmountFlags::DETACH);
    //         log::info!("Successfully mounted USB");
    //     }
    //     Err(e) => log::warn!("Failed to mount USB: {e}"),
    // }

    let ui = AppWindow::new()?;
    let results = load_logs();
    let log_files = VecModel::from_slice(&results);
    ui.global::<Directories>().set_log_files(log_files);

    // Refresh log file being displayed
    {
        let ui_handle = ui.as_weak();
        ui.global::<Directories>().on_refresh(move || {
            let ui = ui_handle.unwrap();
            let new_results = load_logs();
            ui.global::<Directories>()
                .set_log_files(VecModel::from_slice(&new_results))
        });
    }

    ui.run()?;
    Ok(())
}
