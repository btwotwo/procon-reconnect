use serde::{Deserialize, Serialize};
use serde_json;

use winapi_bluetooth::device::*;
use winapi_bluetooth::radio::*;

use std::fs;
use std::io;

const SETTINGS_FILE: &'static str = "settings.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub procon_address: u64,
    pub radio_address: Option<u64>,
}

impl Settings {
    pub fn init() -> io::Result<Settings> {
        let file = fs::File::open(SETTINGS_FILE);

        match file {
            Ok(file) => {
                let reader = io::BufReader::new(file);

                let settings = serde_json::from_reader(reader)?;
                Ok(settings)
            }

            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => {
                    let settings = first_time_launch();
                    let file = std::fs::File::create(SETTINGS_FILE).unwrap();
                    let buffer = io::BufWriter::new(file);

                    serde_json::to_writer(buffer, &settings).unwrap();

                    Ok(settings)
                }

                _ => Err(e),
            },
        }
    }
}

fn first_time_launch() -> Settings {
    use promptly::prompt_default;

    let devices =
        BluetoothDeviceSearch::new(BluetoothDeviceSearchParams::new(None).with_return_all());

    let devices: Vec<BluetoothDeviceInfo> = devices.map(|x| x.unwrap()).collect();

    println!("Looks like that application is launched for the first time. Let's do some basic setup and save it.\n");
    println!("Please enter the index of Pro Controller displayed on your screen.");

    for (i, device) in devices.iter().enumerate() {
        println!("{}. {}", i, device.name().to_string_lossy());
    }

    loop {
        let index: u32 = prompt_default("Which one is Pro Controller?", 0);

        match devices.get(index as usize) {
            Some(item) => {
                return Settings {
                    procon_address: item.address(),
                    radio_address: None,
                }
            }

            None => {
                println!("Invalid input");
            }
        };
    }
}
