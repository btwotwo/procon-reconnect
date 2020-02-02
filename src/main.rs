mod settings;
use settings::Settings;
use winapi_bluetooth::device::*;
use winapi_bluetooth::radio::*;

fn main() -> std::io::Result<()> {
    let settings = Settings::init()?;
    let mut device = get_device_info(&settings);
    let radio = BluetoothRadioSearch::new().nth(0).unwrap()?;

    println!("{:?}", device);

    if device.is_connected() {
        eprint!("Procon is already connected. Exiting.");
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

fn get_device_info(settings: &Settings) -> BluetoothDeviceInfo {
    let params = BluetoothDeviceSearchParams::new(None).with_return_all();

    let search = BluetoothDeviceSearch::new(params)
        .map(|x| x.unwrap())
        .find(|x| x.address() == settings.procon_address);

    match search {
        None => {
            eprint!("Procon address is not valid. Try to remove settings.json.");
            std::process::exit(1);
        }

        Some(device) => device,
    }
}
