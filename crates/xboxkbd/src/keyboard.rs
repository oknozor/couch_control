use evdev::uinput::{VirtualDevice, VirtualDeviceBuilder};
use evdev::{AbsoluteAxisType, InputEventKind};
use evdev::{AttributeSet, EventType, InputEvent, Key};
use log::info;
use std::cmp::Ordering;
use std::sync::mpsc::Receiver;
use std::time::SystemTime;

pub struct FakeKeyboard {
    inner: VirtualDevice,
}

impl FakeKeyboard {
    pub fn new() -> anyhow::Result<Self> {
        let mut keys = AttributeSet::<Key>::new();
        // Meta
        keys.insert(Key::KEY_ENTER);
        keys.insert(Key::KEY_LEFTMETA);
        keys.insert(Key::KEY_LEFTSHIFT);
        keys.insert(Key::KEY_LEFTCTRL);
        keys.insert(Key::KEY_DELETE);

        // Arrow
        keys.insert(Key::KEY_UP);
        keys.insert(Key::KEY_LEFT);
        keys.insert(Key::KEY_RIGHT);
        keys.insert(Key::KEY_DOWN);

        Ok(Self {
            inner: VirtualDeviceBuilder::new()?
                .name("Fake Keyboard")
                .with_keys(&keys)?
                .build()?,
        })
    }

    pub fn emit_key(&mut self, key: Key, value: i32) -> anyhow::Result<()> {
        let event_format = if value == 0 {
            "Released"
        } else if value == 1 {
            "Pressed"
        } else {
            "Unknown"
        };
        info!("Event mapped: {event_format}: {key:?}");

        self.inner
            .emit(&[InputEvent::new(EventType::KEY, key.0, value)])?;
        Ok(())
    }
}

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
    while shutdown_rx.try_recv().is_err() {
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
                                    match value.cmp(&0) {
                                        Ordering::Less => {
                                            virtual_keyboard.emit_key(Key::KEY_LEFT, 1)?;
                                            virtual_keyboard.emit_key(Key::KEY_LEFT, 0)?;
                                        }
                                        Ordering::Greater => {
                                            virtual_keyboard.emit_key(Key::KEY_RIGHT, 1)?;
                                            virtual_keyboard.emit_key(Key::KEY_RIGHT, 0)?;
                                        }
                                        Ordering::Equal => {}
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
                                    match value.cmp(&0) {
                                        Ordering::Less => {
                                            virtual_keyboard.emit_key(Key::KEY_UP, 1)?;
                                            virtual_keyboard.emit_key(Key::KEY_UP, 0)?;
                                        }
                                        Ordering::Greater => {
                                            virtual_keyboard.emit_key(Key::KEY_DOWN, 1)?;
                                            virtual_keyboard.emit_key(Key::KEY_DOWN, 0)?;
                                        }
                                        Ordering::Equal => {}
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
    info!("Terminating Keyboard emulation");

    Ok(())
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
