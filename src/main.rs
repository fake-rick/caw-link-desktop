slint::include_modules!();

mod caw;
use caw::{
    connector::{Connector, ConnectorError},
    devices::{self},
    protocols::discover::{self, DISCOVER_MAGIC},
};
use lazy_static::lazy_static;
use std::{collections::HashMap, sync::Mutex, thread, time::Duration};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

lazy_static! {
    static ref CONNECTORS: Mutex<HashMap<u32, HashMap<u32, Connector>>> =
        Mutex::new(HashMap::new());
}

fn main() -> std::result::Result<(), slint::PlatformError> {
    let task = thread::spawn(move || loop {
        // 发现新设备
        devices::serial::Serial::search(
            115200,
            DISCOVER_MAGIC.as_slice(),
            |device, code| -> Result<()> {
                discover::Discover::check_device_magic(&code[0..4])?;
                let v = discover::Discover::parse(&code[4..12])?;
                if let Ok(mut type_map) = CONNECTORS.lock() {
                    let (device_id, type_id) = v.get_id();
                    if !type_map.contains_key(&type_id) {
                        type_map.insert(type_id, HashMap::new());
                    }
                    if let Some(id_map) = type_map.get_mut(&type_id) {
                        if !id_map.contains_key(&device_id) {
                            id_map.insert(device_id, Connector::new(Box::new(device)));
                            println!("insert device: type_id:{} device_id:{}", type_id, device_id);
                        }
                    }
                }
                Ok(())
            },
        );

        if let Ok(mut type_map) = CONNECTORS.lock() {
            for (_, id_map) in type_map.iter_mut() {
                id_map.retain(|_, conn| !conn.check_timeout());
            }
        }

        thread::sleep(Duration::from_secs(3));
    });

    let ui = AppWindow::new()?;
    let _ = ui.run();
    drop(task);
    Ok(())
}
