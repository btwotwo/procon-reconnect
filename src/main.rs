use winapi_bluetooth::radio::BluetoothRadioSearch;

fn main() {
    let search = BluetoothRadioSearch::new();

    for val in search {
        let test = val.unwrap();
        let info = test.get_radio_info().unwrap();
        let name = info.name();

        let another_name = info.name();

        println!("device name is {}", name.to_string_lossy());
        println!("test {}", another_name.to_string_lossy());
    }
}
