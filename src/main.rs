slint::include_modules!();

mod caw;
use caw::{
    devices::{
        self,
        serial::{self, Serial},
    },
    protocols::discover::{self, DEVICE_MAGIC, DISCOVER_MAGIC},
};
use lazy_static::lazy_static;
use std::{sync::Mutex, thread, time::Duration};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

lazy_static! {
    static ref SERIAL: Mutex<Option<Serial>> = Mutex::new(None);
}

fn main() -> std::result::Result<(), slint::PlatformError> {
    let task = thread::spawn(move || loop {
        devices::serial::Serial::search(
            115200,
            DISCOVER_MAGIC.as_slice(),
            |serial, code| -> Result<()> {
                discover::Discover::check_device_magic(&code[0..4])?;
                let v = discover::Discover::parse(&code[4..12])?;
                Ok(())
            },
        );
        thread::sleep(Duration::from_secs(3));
    });

    let ui = AppWindow::new()?;
    let _ = ui.run();
    drop(task);
    Ok(())
}
