use serde::{Deserialize, Serialize};
use winapi_bluetooth::device::*;
use winapi_bluetooth::radio::BluetoothRadioHandle;

use std::fs;
use std::io;

const SETTINGS_FILE: &str = "settings.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub procon_address: u64,
}

impl Settings {
    pub fn init(radio: &BluetoothRadioHandle) -> io::Result<Settings> {
        let file = fs::File::open(SETTINGS_FILE);

        match file {
            Ok(file) => {
                let reader = io::BufReader::new(file);
                let settings = serde_json::from_reader(reader)?;

                Ok(settings)
            }

            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => {
                    let settings = first_time_launch(radio);
                    let file = std::fs::File::create(SETTINGS_FILE)?;
                    let buffer = io::BufWriter::new(file);

                    serde_json::to_writer(buffer, &settings)?;

                    Ok(settings)
                }

                _ => Err(e),
            },
        }
    }
}

fn first_time_launch(radio: &BluetoothRadioHandle) -> Settings {
    use promptly::prompt_default;

    let devices =
        BluetoothDeviceSearch::new(BluetoothDeviceSearchParams::new(Some(radio)).with_return_all());

    let devices: Vec<BluetoothDeviceInfo> = devices.filter_map(Result::ok).collect();

    println!("Looks like that application is launched for the first time. Let's do some basic setup and save it.\n");
    println!("Please enter the index of Pro Controller displayed on your screen.");
    println!();

    for (i, device) in devices.iter().enumerate() {
        println!("{}. {}", i, device.name().to_string_lossy());
    }

    loop {
        let index: u32 = prompt_default("Which one is Pro Controller?", 0);

        match devices.get(index as usize) {
            Some(item) => {
                return Settings {
                    procon_address: item.address(),
                }
            }

            None => {
                println!("Invalid input");
            }
        };
    }
}
