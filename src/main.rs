slint::include_modules!();

mod caw;
use caw::devices::{self, serial::Serial};
use lazy_static::lazy_static;
use std::{sync::Mutex, thread, time::Duration};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

lazy_static! {
    static ref SERIAL: Mutex<Option<Serial>> = Mutex::new(None);
}

fn main() -> std::result::Result<(), slint::PlatformError> {
    let task = thread::spawn(move || loop {
        let w_buf = vec![0x00u8, 0x00, 0x00, 0x00];
        match devices::serial::Serial::search_and_create(
            115200,
            w_buf.as_slice(),
            |code| -> Result<()> {
                println!("{:?}", code);
                Ok(())
            },
        ) {
            Err(err) => println!("error: {:?}", err),
            Ok(serial) => {
                let _ = SERIAL.lock().map(|mut s| match *s {
                    Some(_) => (),
                    None => *s = Some(serial),
                });
            }
        }
        thread::sleep(Duration::from_secs(3));
    });

    let ui = AppWindow::new()?;
    let _ = ui.run();
    drop(task);
    Ok(())
}
