slint::include_modules!();

mod caw;
use caw::{
    connector::Connector,
    devices::{self, serial::Serial},
    event::Event,
    protocols::{
        code::{CmdCode, SystemCode},
        discover::{self, DISCOVER_MAGIC},
        pingpong::pong,
    },
};

use lazy_static::lazy_static;

lazy_static! {
    static ref CONNECTORS: Mutex<HashMap<u32, HashMap<u32, Connector>>> =
        Mutex::new(HashMap::new());
}

use std::{collections::HashMap, sync::Mutex, thread, time::Duration};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// 事件注册
fn event_build() -> Event {
    Event::new().register(CmdCode::System(SystemCode::Pong), pong)
}

/// 硬件发现服务回调函数
fn discover_callback(mut device: Serial, buf: &[u8]) -> Result<()> {
    println!("discover thread id:{:?}", thread::current().id());
    discover::Discover::check_device_magic(&buf[0..4])?;
    let v = discover::Discover::parse(&buf[4..12])?;
    if let Ok(mut type_map) = CONNECTORS.lock() {
        let (device_id, type_id) = v.get_id();
        if !type_map.contains_key(&type_id) {
            type_map.insert(type_id, HashMap::new());
        }
        if let Some(id_map) = type_map.get_mut(&type_id) {
            if !id_map.contains_key(&device_id) {
                device.set_id(device_id, type_id);
                let mut connector = Connector::new(Box::new(device));
                connector.event_loop(event_build());
                id_map.insert(device_id, connector);
                println!("insert device: type_id:{} device_id:{}", type_id, device_id);
            }
        }
    }
    Ok(())
}

fn main() -> std::result::Result<(), slint::PlatformError> {
    println!("main thread id:{:?}", thread::current().id());
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    rt.block_on(async {
        tokio::spawn(async {
            loop {
                devices::serial::Serial::search(
                    115200,
                    DISCOVER_MAGIC.as_slice(),
                    discover_callback,
                );
            }
        });
        tokio::spawn(async {
            let mut interval = tokio::time::interval(Duration::from_secs(3));
            loop {
                if let Ok(mut type_map) = CONNECTORS.lock() {
                    for (_, id_map) in type_map.iter_mut() {
                        id_map.retain(|_, conn| !conn.check_timeout());
                    }
                }
                interval.tick().await;
            }
        });
    });

    let ret = tokio::task::block_in_place(|| {
        println!("ui thread id:{:?}", thread::current().id());
        let ui = AppWindow::new().unwrap();
        ui.run()
    });
    rt.shutdown_background();
    ret
}
