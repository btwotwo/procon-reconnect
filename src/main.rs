use winapi_bluetooth::radio::BluetoothRadioSearch;

fn main() {
    let search = BluetoothRadioSearch::new();

    for val in search {
        let test = val.unwrap();
        let name = test.get_radio_info().unwrap().name().to_string_lossy();

        println!("device name is {}", name);
    }
}
