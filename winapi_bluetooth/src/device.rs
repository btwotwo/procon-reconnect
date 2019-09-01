use super::winapi_imports::*;
use winapi::um::bluetoothapis::BLUETOOTH_DEVICE_INFO;
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;

/// Represents Bluetooth Device Info
///
/// This is a wrapper over [`BLUETOOTH_DEVICE_INFO`](https://docs.microsoft.com/en-us/windows/win32/api/bluetoothapis/ns-bluetoothapis-bluetooth_device_info_struct)
pub struct BluetoothDeviceInfo(BLUETOOTH_DEVICE_INFO);

impl BluetoothDeviceInfo {
    pub fn is_connected(&self) -> bool {
        self.0.fConnected == TRUE
    }

    pub fn is_remembered(&self) -> bool {
        self.0.fRemembered == TRUE
    }

    pub fn is_authenticated(&self) -> bool {
        self.0.fAuthenticated == TRUE
    }

    pub fn name(&self) -> OsString {
        OsString::from_wide(&self.0.szName)
    }
}
