mod settings;
use std::io::Result;

use settings::Settings;
use winapi_bluetooth::device::*;
use winapi_bluetooth::radio::*;

fn main() -> std::io::Result<()> {
    let radio = get_radio()?;
    let settings = Settings::init()?;
    let mut device = get_device_info(&settings, &radio);

    println!("{:?}", device);

    if device.is_connected() {
        exit("Pro Controller is already connected. Exiting.")
    }

    if device.is_remembered() {
        device.remove_device()?;
    }

    device.authenticate_device(Some(&radio))?;

    let service_count = device.count_installed_services()?;

    if service_count == 0 {
        device.enable_hid_service(&radio)?;
    }

    Ok(())
}

fn get_radio() -> Result<BluetoothRadioHandle> {
    let radio = BluetoothRadioSearch::new().nth(0);

    match radio {
        None => {
            exit("Bluetooth connector not found!");
        }
        Some(radio) => radio,
    }
}

fn get_device_info(settings: &Settings, radio: &BluetoothRadioHandle) -> BluetoothDeviceInfo {
    let params = BluetoothDeviceSearchParams::new(Some(radio)).with_return_all();

    let search = BluetoothDeviceSearch::new(params)
        .filter_map(Result::ok)
        .find(|x| x.address() == settings.procon_address);

    match search {
        None => {
            exit("Procon address is not valid. Try to remove settings.json.");
        }

        Some(device) => device,
    }
}

fn exit(error: &str) -> ! {
    eprintln!("Error: {}", error);
    std::io::stdin().read_line(&mut String::new()).unwrap();
    std::process::exit(1)
}
