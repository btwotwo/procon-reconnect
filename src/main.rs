use winapi_bluetooth::device::*;
use winapi_bluetooth::radio::BluetoothRadioSearch;

fn main() {
    let search_params = BluetoothDeviceSearchParams::new(None).with_return_all();

    let search = BluetoothDeviceSearch::new(search_params);

    for val in search {
        let test = val.unwrap();
        println!("{:?}", test);
    }

    // let search = BluetoothRadioSearch::new();

    // for val in search {
    //     let test = val.unwrap();
    //     let info = test.get_radio_info().unwrap();
    //     let name = info.name();

    //     println!("radio name is {}", name.to_string_lossy());
    // }
}
