use super::winapi_imports::*;
use winapi::um::bluetoothapis::{BLUETOOTH_DEVICE_INFO, BLUETOOTH_DEVICE_SEARCH_PARAMS, HBLUETOOTH_DEVICE_FIND};
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::io;

use super::radio::BluetoothRadioHandle;
use super::last_error;
/// Wrapper over [`BLUETOOTH_DEVICE_SEARCH_PARAMS`](https://docs.microsoft.com/en-us/windows/win32/api/bluetoothapis/ns-bluetoothapis-bluetooth_device_search_params)
pub struct BluetoothDeviceSearchParams(BLUETOOTH_DEVICE_SEARCH_PARAMS);

impl BluetoothDeviceSearchParams {
    pub fn new(handle: Option<BluetoothRadioHandle>) -> Self {
        use std::mem;

        let mut inner: BLUETOOTH_DEVICE_SEARCH_PARAMS = unsafe {mem::zeroed()};
        
        inner.dwSize = mem::size_of::<BLUETOOTH_DEVICE_SEARCH_PARAMS>() as u32;
        inner.cTimeoutMultiplier = 10;

        if let Some(handle) = handle {
            inner.hRadio = handle.0;
        }

        BluetoothDeviceSearchParams(inner)
    }

    pub fn with_return_all(mut self) -> Self {
        self.0.fReturnAuthenticated = TRUE;
        self.0.fReturnRemembered = TRUE;
        self.0.fReturnUnknown = TRUE;
        self.0.fReturnConnected = TRUE;

        self
    }

    pub fn with_timeout_multiplier(mut self, mult: u8) -> Self {
        self.0.cTimeoutMultiplier = mult;
        self
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
        self.0.fConnected != FALSE
    }

    pub fn is_remembered(&self) -> bool {
        self.0.fRemembered != FALSE
    }

    pub fn is_authenticated(&self) -> bool {
        self.0.fAuthenticated != FALSE
    }

    pub fn name(&self) -> OsString {
        OsString::from_wide(&self.0.szName)
    }

    pub fn address(&self) -> u64 {
        self.0.Address
    }


    pub fn remove_device(&mut self) -> io::Result<()> {
        match self.is_remembered() {
            false => Err(io::Error::new(io::ErrorKind::InvalidInput, "Device is not remembered")),
            true => match unsafe {bluetoothapis::BluetoothRemoveDevice(&self.address())} {
                ERROR_SUCCESS => {
                    self.0.fConnected = FALSE;
                    self.0.fRemembered = FALSE;
                    self.0.fAuthenticated = FALSE;
                    Ok(())
                },
                _ => Err(last_error())
            }
        }

    }

    pub fn authenticate_device(&mut self, handle: Option<BluetoothRadioHandle>) -> io::Result<()> {
        use widestring::U16String;
        use std::convert::TryInto;
        
        let mut passwd = U16String::from_str("0000").into_vec();

        let handle = match handle {
            Some(handle) => handle.0, 
            None => unsafe {std::mem::zeroed()}
        };

        match self.is_authenticated() {
            true => Err(io::Error::new(io::ErrorKind::InvalidInput, "Device is already connected")),
            false => match unsafe {bluetoothapis::BluetoothAuthenticateDevice(
                std::ptr::null_mut(), 
                handle, 
                &mut self.0,
                passwd.as_mut_ptr(),
                passwd.len() as u32)} {    
                    ERROR_SUCCESS => Ok(()),
                    val => Err(io::Error::from_raw_os_error(val.try_into().unwrap()))
                }
        }
    }
}

impl std::fmt::Debug for BluetoothDeviceInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,
            "Name: {}\nAddress: {}\nConnected: {}\nRemembered: {}\nAuthenticated: {}\n", 
            self.name().to_string_lossy(), 
            self.address(), self.is_connected(), 
            self.is_remembered(), self.is_authenticated()
        )

    }
}

pub struct BluetoothDeviceSearch {
    params: BluetoothDeviceSearchParams,
    search: HBLUETOOTH_DEVICE_FIND,
    init: bool
}

impl BluetoothDeviceSearch {
    pub fn new(params: BluetoothDeviceSearchParams) -> Self {
        BluetoothDeviceSearch {
            params,
            search: std::ptr::null_mut(),
            init: false
        }
    }

    fn find_first(&mut self) -> io::Result<BluetoothDeviceInfo> {
        use winapi::um::bluetoothapis::{BluetoothFindFirstDevice};

        let mut bluetooth_info = get_blank_info();

        let result = unsafe {
            BluetoothFindFirstDevice(&self.params.0, &mut bluetooth_info)
        };

        if result.is_null() {
            Err(last_error())
        } else {
            self.search = result;
            self.init = true;
            Ok(BluetoothDeviceInfo(bluetooth_info))
        }
    }

    pub fn find_next(&mut self) -> io::Result<BluetoothDeviceInfo> {
        use winapi::um::bluetoothapis::{BluetoothFindNextDevice};

        let mut info = get_blank_info();

        if !self.init {
            self.find_first()
        } else {
            let result = unsafe {BluetoothFindNextDevice(self.search, &mut info)};
            if result != FALSE {
                Ok(BluetoothDeviceInfo(info))
            } else {
                Err(last_error())
            }
        }
    }

}

impl Iterator for BluetoothDeviceSearch {
    type Item = io::Result<BluetoothDeviceInfo>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.find_next() {
            Ok(res) => Some(Ok(res)),
            Err(error) => match error.raw_os_error() {
                Some(ERROR_NO_MORE_ITEMS) => None,
                _ => Some(Err(error))
            }
        }
    }
}


impl Drop for BluetoothDeviceSearch {
    fn drop(&mut self) {
        use winapi::um::bluetoothapis::BluetoothFindDeviceClose;

        let result = unsafe { BluetoothFindDeviceClose(self.search) };

        if result == FALSE {
            panic!("Failed to close search! Error: {}", last_error())
        }
    }
}


fn get_blank_info() -> BLUETOOTH_DEVICE_INFO {
    let mut info: BLUETOOTH_DEVICE_INFO = unsafe {std::mem::zeroed()};
    info.dwSize = std::mem::size_of::<BLUETOOTH_DEVICE_INFO>() as u32;

    info
}