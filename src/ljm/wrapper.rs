extern crate libloading;
use std::{
    ffi::{c_char, CString},
    os::raw::c_double,
};

use libloading::{Library, Symbol};

pub struct LJMWrapper {
    pub library: Library,
}

#[derive(Debug)]
pub enum LJMError {
    StartupError(libloading::Error)
}

impl LJMWrapper {
    pub fn get_library_path() -> String {
        let os = std::env::consts::OS;

        if os == "windows" {
            "LabJackM.dll".to_string()
        } else if os == "linux" {
            "/usr/local/lib/libLabJackM.so".to_string()
        } else if os == "macos" {
            "/usr/local/lib/libLabJackM.dylib".to_string()
        } else {
            panic!("Not a supported operating system.");
        }
    }

    /// `unsafe`
    /// Initializes a labjack interface with the static library.
    pub unsafe fn init() -> Result<Self, LJMError> {
        let library: Library = unsafe {
            let library_path = LJMWrapper::get_library_path();

            match Library::new(library_path) {
                Ok(library) => library,
                Err(error) => return Err(LJMError::StartupError(error))
            }
        };

        Ok(LJMWrapper { library })
    }

    /// Converts a MODBUS name to its address and type
    /// Returns a tuple of (address, type) in (i32, i32) format.
    /// Verifiable with: -
    /// https://labjack.com/pages/support/?doc=/datasheets/t-series-datasheet/31-modbus-map-t-series-datasheet/
    pub fn name_to_address(&self, identifier: String) -> (i32, i32) {
        let n_to_addr: Symbol<extern "C" fn(*const c_char, *mut i32, *mut i32) -> i32> =
            unsafe { self.library.get(b"LJM_NameToAddress").unwrap() };

        let name = CString::new(identifier).expect("CString conversion failed");
        let mut address: i32 = 0;
        let mut typ: i32 = 0;

        n_to_addr(name.as_ptr(), &mut address, &mut typ);

        (address, typ)
    }

    /// Digitally writes to address
    /// Takes a handle to the labjack, the name to be written and the value to be written.
    /// Does not return a value.
    pub fn write_name(&self, handle: i32, name_to_write: String, value_to_write: u32) {
        let d_write_to_addr: Symbol<extern "C" fn(i32, *const c_char, c_double)> =
            unsafe { self.library.get(b"LJM_eWriteName").unwrap() };

        let ntw = CString::new(name_to_write).expect("CString conversion failed");
        let vtw = c_double::from(value_to_write);

        d_write_to_addr(handle, ntw.as_ptr(), vtw);
    }

    /// Reads from a labjack given the handle and name to read.
    /// Returns an f64 value that is read from the labjack.
    pub fn read_name(&self, handle: i32, name_to_read: String) -> f64 {
        let d_read_from_aadr: Symbol<extern "C" fn(i32, *const c_char, c_double)> =
            unsafe { self.library.get(b"LJM_eReadName").unwrap() };

        let ntr = CString::new(name_to_read).expect("CString conversion failed");
        let vtr = c_double::from(0);

        d_read_from_aadr(handle, ntr.as_ptr(), vtr);

        vtr.into()
    }

    /// Opens a LabJack and returns the handle id as an i32.
    pub fn open_jack(&self, identifier: String) -> i32 {
        let open_s: Symbol<
            extern "C" fn(*const c_char, *const c_char, *const c_char, *mut i32) -> i32,
        > = unsafe { self.library.get(b"LJM_OpenS").unwrap() };

        let device_type = CString::new("ANY".to_string()).expect("CString conversion failed");
        let connection_type = CString::new("ANY".to_string()).expect("CString conversion failed");
        let ident = CString::new(identifier).expect("CString conversion failed");

        let mut vtr: i32 = 0;

        open_s(
            device_type.as_ptr(),
            connection_type.as_ptr(),
            ident.as_ptr(),
            &mut vtr,
        );

        vtr
    }
}
