use winapi::um::bluetoothapis::{BLUETOOTH_DEVICE_SEARCH_PARAMS, BLUETOOTH_DEVICE_INFO, BluetoothFindFirstDevice, BluetoothFindNextDevice, BluetoothFindDeviceClose};
use std::mem;
use winapi::shared::minwindef::{DWORD, TRUE, FALSE};
use winapi::shared::ntdef::NULL;

fn main() {
    unsafe {
        let search_params = BLUETOOTH_DEVICE_SEARCH_PARAMS {
            dwSize: (mem::size_of::<BLUETOOTH_DEVICE_SEARCH_PARAMS>() as DWORD),
            fReturnAuthenticated: TRUE,
            fReturnRemembered: TRUE,
            fReturnUnknown: FALSE,
            fReturnConnected: FALSE,
            fIssueInquiry: TRUE,
            hRadio: NULL,
            cTimeoutMultiplier: 4
        };

        let mut devices = Vec::new();

        let mut device_info = mem::zeroed::<BLUETOOTH_DEVICE_INFO>();
        device_info.dwSize = mem::size_of::<BLUETOOTH_DEVICE_INFO>() as DWORD;

        let handler = BluetoothFindFirstDevice(&search_params, &mut device_info);

        if handler != NULL {
            loop {
                devices.push(device_info);

                if BluetoothFindNextDevice(handler, &mut device_info) == FALSE {
                    break;
                }
            }
        }

        BluetoothFindDeviceClose(handler);

        for val in devices {
            let name = String::from_utf16_lossy(&val.szName);
            println!("Found device! Name is: {}", name);
            println!("And the address is: {}", val.Address);
        }
    }
}
