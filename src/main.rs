extern crate libloading;

use std::{ffi::CString, os::raw::c_double};

use libloading::{Library, Symbol};

struct LJMWrapper {
    pub library: libloading::Library,
}

impl LJMWrapper {
    fn get_library_path() -> String {
        let os = std::env::consts::OS;

        if os == "windows" {
            "LabJackM.dll".to_string()
        } else if os == "linux" {
            "libLabJackM.so".to_string()
        } else if os == "macos" {
            "/usr/local/lib/libLabJackM.dylib".to_string()
        } else {
            panic!("Not a supported operating system.");
        }
    }

    pub unsafe fn init() -> Self {
        let library: Library = unsafe {
            let library_path = LJMWrapper::get_library_path();
            Library::new(library_path).expect("Failed to load library")
        };

        LJMWrapper { library }
    }

    /// Converts a MODBUS name to its address and type
    /// Returns a tuple of (address, type) in (i32, i32) format.
    /// Verifiable with: -
    /// https://labjack.com/pages/support/?doc=/datasheets/t-series-datasheet/31-modbus-map-t-series-datasheet/
    pub fn name_to_address(&self, identifier: String) -> (i32, i32) {
        let n_to_addr: Symbol<extern "C" fn(*const i8, *mut i32, *mut i32) -> i32> =
            unsafe { self.library.get(b"LJM_NameToAddress").unwrap() };

        let name = CString::new(identifier).expect("CString conversion failed");
        let mut address: i32 = 0;
        let mut typ: i32 = 0;

        n_to_addr(name.as_ptr(), &mut address, &mut typ);

        (address, typ)
    }

    /// Digitally writes to address
    pub fn digital_write_to_address(
        &self,
        identifier: String,
        name_to_write: String,
        value_to_write: u32,
    ) {
        let d_write_to_addr: Symbol<extern "C" fn(*const i8, *const i8, c_double)> =
            unsafe { self.library.get(b"LJM_eWriteAddress").unwrap() };

        let id = CString::new(identifier).expect("CString conversion failed");
        let ntw = CString::new(name_to_write).expect("CString conversion failed");
        let vtw = c_double::from(value_to_write);

        d_write_to_addr(id.as_ptr(), ntw.as_ptr(), vtw);
    }
}

fn main() {
    let ljm_wrapper = unsafe { LJMWrapper::init() };

    let (addr, typ) = ljm_wrapper.name_to_address("AIN0".to_string());
    ljm_wrapper.digital_write_to_address("AIN0".to_string(), "AIN0_RANGE".to_string(), 15_u32);

    println!("Function result: {}:{}", addr, typ);
}
