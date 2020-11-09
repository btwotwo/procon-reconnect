use winapi::shared::ntdef::NULL;
use super::winapi_imports::*;
use super::last_error;
use winapi::um::bluetoothapis::{HBLUETOOTH_RADIO_FIND, BLUETOOTH_RADIO_INFO};
use std::io;

use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;

pub struct BluetoothRadioHandle(pub HANDLE);
pub struct BluetoothRadioInfo(BLUETOOTH_RADIO_INFO);


impl BluetoothRadioInfo {
    pub fn name(&self) -> OsString {
        OsString::from_wide(&self.0.szName)
    }
}

impl Default for BluetoothRadioInfo {
    fn default() -> Self {
        let mut val: BLUETOOTH_RADIO_INFO = unsafe {std::mem::zeroed()};
        val.dwSize = std::mem::size_of::<BLUETOOTH_RADIO_INFO>() as DWORD;

        BluetoothRadioInfo(val)
    }
}

impl BluetoothRadioHandle {
    pub fn get_radio_info(&self) -> io::Result<BluetoothRadioInfo> {
        use winapi::um::bluetoothapis::BluetoothGetRadioInfo;

        let mut info = BluetoothRadioInfo::default();

        let result = unsafe {
            BluetoothGetRadioInfo(self.0, &mut info.0)
        };
    
        match result {
            ERROR_SUCCESS => Ok(info),
            _ => Err(io::Error::from_raw_os_error(result as i32))
        }
    }
}

impl Drop for BluetoothRadioHandle {
    fn drop(&mut self) {
        use winapi::um::handleapi::CloseHandle;

        let res = unsafe { CloseHandle(self.0) };

        if res == 0 {
            panic!("Failed to release radio! Error: {}", last_error());
        }
    }
}

/// Wrapper over [`BluetoothFindFirstRadio`](https://docs.microsoft.com/en-us/windows/win32/api/bluetoothapis/nf-bluetoothapis-bluetoothfindfirstradio),
/// [`BluetoothFindNextRadio`](https://docs.microsoft.com/en-us/windows/win32/api/bluetoothapis/nf-bluetoothapis-bluetoothfindnextradio)
/// and [`BluetoothFindRadioClose`](https://docs.microsoft.com/en-us/windows/win32/api/bluetoothapis/nf-bluetoothapis-bluetoothfindradioclose)
/// implemented in the form of [`Iterator`](std::iter::Iterator)
/// 
/// # Example
/// 
/// Basic usage:
/// ```
/// let search = BluetoothRadioSearch::new();
/// 
/// for val in search {
///    // do something with handle
/// }
/// ```

pub struct BluetoothRadioSearch {
    search: HBLUETOOTH_RADIO_FIND,
    init: bool,
}

impl Default for BluetoothRadioSearch {
    fn default() -> Self {
        BluetoothRadioSearch {
            init: false,
            search: std::ptr::null_mut()
        }
    }
}

impl BluetoothRadioSearch {
    /// Creates new [`Iterator`](std::iter::Iterator) over the bluetooth radios.
    pub fn new() -> Self {
        BluetoothRadioSearch::default()
    }

    fn find_first(&mut self) -> io::Result<BluetoothRadioHandle> {
        use std::ptr::null_mut;
        use winapi::um::bluetoothapis::{BluetoothFindFirstRadio, BLUETOOTH_FIND_RADIO_PARAMS};

        let mut radio_handle: HANDLE = null_mut();

        let result = unsafe {
            BluetoothFindFirstRadio(
                &BLUETOOTH_FIND_RADIO_PARAMS {
                    dwSize: std::mem::size_of::<BLUETOOTH_FIND_RADIO_PARAMS>() as DWORD,
                },
                &mut radio_handle,
            )
        };

        if result.is_null() {
            Err(last_error())
        } else {
            self.search = result;
            self.init = true;
            Ok(BluetoothRadioHandle(radio_handle))
        }
    }

    /// You can use this function instead of iterator.
    pub fn find_next(&mut self) -> io::Result<BluetoothRadioHandle> {
        use winapi::um::bluetoothapis::BluetoothFindNextRadio;

        let mut radio_handle: HANDLE = std::ptr::null_mut();

        if !self.init {
            self.find_first()
        } else {
            let result = unsafe { BluetoothFindNextRadio(self.search, &mut radio_handle) };

            if result != FALSE {
                Ok(BluetoothRadioHandle(radio_handle))
            } else {
                Err(last_error())
            }
        }
    }
}

impl Iterator for BluetoothRadioSearch {
    type Item = io::Result<BluetoothRadioHandle>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.find_next() {
            Ok(res) => Some(Ok(res)),
            Err(error) => match error.raw_os_error(){
                Some(ERROR_NO_MORE_ITEMS) => None,
                _ => Some(Err(error)),
            },
        }
    }
}

impl Drop for BluetoothRadioSearch {
    fn drop(&mut self) {
        use winapi::um::bluetoothapis::BluetoothFindRadioClose;

        if self.search == NULL {
            return;
        }

        let result = unsafe { BluetoothFindRadioClose(self.search) };

        if result == FALSE {
            panic!("Failed to close search! Error: {}", last_error())
        }
    }
}