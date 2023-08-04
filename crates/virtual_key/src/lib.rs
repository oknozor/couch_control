use crate::keyboard::FakeKeyboard;
use evdev::{AbsoluteAxisType, InputEventKind};
use std::sync::mpsc::Receiver;
use std::time::SystemTime;

mod keyboard;

pub use evdev::Key;
use log::{info};

const AXIS_THRESHOLD: i32 = 30000;
const PRESSURE_DELAY: u128 = 150;

pub fn emulate_keyboard(
    shutdown_rx: Receiver<()>,
    mappings: Vec<(Key, Key)>,
) -> anyhow::Result<()> {
    let mut virtual_keyboard = FakeKeyboard::new()?;
    let devices = evdev::enumerate().map(|t| t.1).collect::<Vec<_>>();
    let Some(mut dev) = devices.into_iter().find(|dev| dev.name() == Some("Generic X-Box pad")) else {
        {
            panic!("No such device")
        }
    };

    info!("X-Box controller device found");
    let mut last_axis_input_time = SystemTime::now();

    info!("Starting Keyboard emulation");
    loop {
        for ev in dev.fetch_events().unwrap() {
            match ev.kind() {
                  InputEventKind::Key(key) => {
                    let key = mappings
                        .iter()
                        .find(|(xbox_key, _)| key == *xbox_key)
                        .map(|(_, map)| map);

                    if let Some(key) = key {
                        virtual_keyboard.emit_key(*key, ev.value()).unwrap();
                    }
                }
                InputEventKind::AbsAxis(axis) => {
                    let value = ev.value();
                    match axis {
                        AbsoluteAxisType::ABS_X => {
                            let event_timestamp = ev.timestamp();

                            if value.abs() > AXIS_THRESHOLD {
                                let elapsed = event_timestamp
                                    .duration_since(last_axis_input_time)
                                    .unwrap()
                                    .as_millis();
                                if elapsed > PRESSURE_DELAY {
                                    last_axis_input_time = event_timestamp;
                                    if value > 0 {
                                        virtual_keyboard.emit_key(Key::KEY_RIGHT, 1)?;
                                        virtual_keyboard.emit_key(Key::KEY_RIGHT, 0)?;
                                    } else if value < 0 {
                                        virtual_keyboard.emit_key(Key::KEY_LEFT, 1)?;
                                        virtual_keyboard.emit_key(Key::KEY_LEFT, 0)?;
                                    }
                                }
                            }
                        }
                        AbsoluteAxisType::ABS_Y => {
                            let event_timestamp = ev.timestamp();
                            let value = ev.value();

                            if value.abs() > AXIS_THRESHOLD {
                                let elapsed = event_timestamp
                                    .duration_since(last_axis_input_time)
                                    .unwrap()
                                    .as_millis();
                                if elapsed > PRESSURE_DELAY {
                                    last_axis_input_time = event_timestamp;
                                    if value > 0 {
                                        virtual_keyboard.emit_key(Key::KEY_DOWN, 1)?;
                                        virtual_keyboard.emit_key(Key::KEY_DOWN, 0)?;
                                    } else if value < 0 {
                                        virtual_keyboard.emit_key(Key::KEY_UP, 1)?;
                                        virtual_keyboard.emit_key(Key::KEY_UP, 0)?;
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::emulate_keyboard;
    use evdev::Key;
    use std::sync::mpsc::channel;

    #[test]
    fn emulate_keyboard_works() {
        emulate_keyboard(
            channel().1,
            vec![
                (Key::BTN_SOUTH, Key::KEY_ENTER),
                (Key::BTN_EAST, Key::KEY_DELETE),
                (Key::BTN_SELECT, Key::KEY_LEFTSHIFT),
                (Key::BTN_START, Key::KEY_LEFTCTRL),
                (Key::BTN_MODE, Key::KEY_LEFTMETA),
            ],
        )
        .unwrap();
    }
}
