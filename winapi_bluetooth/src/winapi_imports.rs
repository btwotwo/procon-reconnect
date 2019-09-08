pub use winapi::shared::minwindef::{DWORD, FALSE, TRUE};
pub use winapi::um::winnt::HANDLE;
pub use winapi::um::bluetoothapis;

pub use winapi::shared::winerror::{
    ERROR_SUCCESS
};

pub const ERROR_NO_MORE_ITEMS: i32 = winapi::shared::winerror::ERROR_NO_MORE_ITEMS as i32;
