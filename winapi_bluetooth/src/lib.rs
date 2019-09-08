pub mod winapi_imports;
pub mod radio;
pub mod device;

use std::io;

fn last_error() -> io::Error {
    io::Error::last_os_error()
}
