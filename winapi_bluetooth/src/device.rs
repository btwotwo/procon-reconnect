use super::winapi_imports::*;
use winapi::um::bluetoothapis::{BLUETOOTH_DEVICE_INFO, BLUETOOTH_DEVICE_SEARCH_PARAMS};
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;



/// Wrapper over [`BLUETOOTH_DEVICE_SEARCH_PARAMS`](https://docs.microsoft.com/en-us/windows/win32/api/bluetoothapis/ns-bluetoothapis-bluetooth_device_search_params)
pub struct BluetoothDeviceSearchParams(BLUETOOTH_DEVICE_SEARCH_PARAMS);

impl BluetoothDeviceSearchParams {
    pub fn new(handle: Option<HANDLE>) -> Self {
        use std::mem;

        let mut inner: BLUETOOTH_DEVICE_SEARCH_PARAMS = unsafe {mem::zeroed()};
        
        inner.dwSize = mem::size_of::<BLUETOOTH_DEVICE_SEARCH_PARAMS>() as u32;
        inner.cTimeoutMultiplier = 10;

        if let Some(handle) = handle {
            inner.hRadio = handle;
        }

        BluetoothDeviceSearchParams(inner)
    }

    pub fn with_return_authenticated(mut self) -> Self {
        self.0.fReturnAuthenticated = TRUE;
        self
    }

    pub fn with_return_remembered(mut self) -> Self {
        self.0.fReturnRemembered = TRUE;
        self
    }

    pub fn with_return_unknown(mut self) -> Self {
        self.0.fReturnUnknown = TRUE;
        self
    }

    pub fn with_return_connected(mut self) -> Self {
        self.0.fReturnConnected = TRUE;
        self
    }

    pub fn with_issue_inquiry(mut self) -> Self {
        self.0.fIssueInquiry = TRUE;
        self
    }
}

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
