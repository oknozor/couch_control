use evdev::uinput::{VirtualDevice, VirtualDeviceBuilder};
use evdev::{AttributeSet, EventType, InputEvent, Key};
use log::info;

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
