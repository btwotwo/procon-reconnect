use std::mem;
use winapi::shared::minwindef::{DWORD, FALSE, TRUE};
use winapi::shared::ntdef::NULL;
use winapi::um::bluetoothapis::*;
use winapi::um::minwinbase::SYSTEMTIME;

type BluetoothAddress = BLUETOOTH_ADDRESS;

#[derive(Debug)]
pub struct BluetoothDeviceInfo {
    pub address: BluetoothAddress,
    pub class_of_device: u32,
    pub connected: bool,
    pub remembered: bool,
    pub authenticated: bool,
    // pub last_seen: SYSTEMTIME,
    // pub last_used: SYSTEMTIME,
    pub name: String,
}

pub fn find_devices() -> Vec<BluetoothDeviceInfo> {
    let search_params = BLUETOOTH_DEVICE_SEARCH_PARAMS {
        dwSize: (mem::size_of::<BLUETOOTH_DEVICE_SEARCH_PARAMS>() as DWORD),
        fReturnAuthenticated: TRUE,
        fReturnRemembered: TRUE,
        fReturnUnknown: TRUE,
        fReturnConnected: TRUE,
        fIssueInquiry: TRUE,
        hRadio: NULL,
        cTimeoutMultiplier: 4,
    };

    let mut devices = Vec::<BluetoothDeviceInfo>::new();

    let mut device_info = BLUETOOTH_DEVICE_INFO {
        dwSize: mem::size_of::<BLUETOOTH_DEVICE_INFO>() as DWORD,
        ..Default::default()
    };

    unsafe {
        let handler = BluetoothFindFirstDevice(&search_params, &mut device_info);

        if handler != NULL {
            loop {
                devices.push(BluetoothDeviceInfo {
                    address: device_info.Address,
                    authenticated: device_info.fAuthenticated == TRUE,
                    class_of_device: device_info.ulClassofDevice,
                    connected: device_info.fConnected == TRUE,
                    remembered: device_info.fRemembered == TRUE,
                    // last_seen: device_info.stLastSeen,
                    // last_used: device_info.stLastUsed,
                    name: String::from_utf16_lossy(&device_info.szName),
                });

                if BluetoothFindNextDevice(handler, &mut device_info) == FALSE {
                    break;
                }
            }
        }

        BluetoothFindDeviceClose(handler);
    }
    devices
}
