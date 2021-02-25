mod settings;
use std::io::Result;

use settings::Settings;
use winapi_bluetooth::device::*;
use winapi_bluetooth::radio::*;

fn main() -> std::io::Result<()> {
    println!("Initalizing...");
    let radio = get_radio()?;

    println!("Found Bluetooth connector");
    let settings = Settings::init()?;

    println!("Initialized settings");
    let mut device = get_device_info(&settings, &radio);

    println!("{:?}", device);

    if device.is_connected() {
        exit("Pro Controller is already connected. Exiting.")
    }

    if device.is_remembered() {
        println!("Device is remembered. Removing device");
        device.remove_device()?;

        println!("Device removed");
    }

    device.authenticate_device(Some(&radio))?;
    println!("Device autheticated");

    let service_count = device.count_installed_services()?;

    if service_count == 0 {
        println!("Enabling HID service...");
        device.enable_hid_service(&radio)?;
    }

    Ok(())
}

fn get_radio() -> Result<BluetoothRadioHandle> {
    let radio = BluetoothRadioSearch::new().next();

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
