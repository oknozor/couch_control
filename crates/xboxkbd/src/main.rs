use crate::keyboard::emulate_keyboard;
use evdev::Key;
use log::{info, LevelFilter};
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;
use sysinfo::{System, SystemExt};
use systemd_journal_logger::JournalLog;

mod keyboard;

fn main() -> anyhow::Result<()> {
    let mut emulation_running = false;
    JournalLog::default()
        .with_extra_fields(vec![("VERSION", env!("CARGO_PKG_VERSION"))])
        .with_syslog_identifier("xboxkbd".to_string())
        .install()
        .unwrap();

    log::set_max_level(LevelFilter::Info);

    let mut shutdown_sender = None;
    loop {
        let (shutdown_tx, shutdown_rx) = channel();
        let s = System::new_all();
        let steam_proc = s.processes_by_exact_name("steam").next();

        match steam_proc {
            None if !emulation_running => {
                info!("Steam not running starting xbox keyboard emulation");
                emulation_running = true;
                shutdown_sender = Some(shutdown_tx);

                thread::spawn(move || {
                    emulate_keyboard(
                        shutdown_rx,
                        vec![
                            (Key::BTN_SOUTH, Key::KEY_ENTER),
                            (Key::BTN_EAST, Key::KEY_DELETE),
                            (Key::BTN_SELECT, Key::KEY_LEFTSHIFT),
                            (Key::BTN_START, Key::KEY_LEFTCTRL),
                            (Key::BTN_MODE, Key::KEY_LEFTMETA),
                        ],
                    )
                    .expect("Xbox Keyboard emulation  failed")
                });
            }
            Some(_) if emulation_running => {
                info!("Steam is now running shutting down keyboard emulation");
                shutdown_sender
                    .clone()
                    .expect("Sender should be set")
                    .send(())?;
                info!("Message sent mother fucker");
                emulation_running = false
            }
            _ => {}
        }

        thread::sleep(Duration::from_secs(2));
    }
}
