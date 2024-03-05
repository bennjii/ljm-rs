use std::{
    ffi::{c_char, c_uint, CString},
    fmt::Display,
    os::raw::c_double,
};

use libloading::{Library, Symbol};
#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer};

use crate::ljm::handle::{ConnectionType, DeviceHandleInfo, DeviceType};

static LJM_DUMMY: LJMWrapper = LJMWrapper::dummy();

pub struct LJMWrapper {
    pub library: Option<Library>,
}

// We always return a dummy wrapper (uninitialized library)
// When being deserialized, as there is no way to correctly serialize
// The wrapper.
#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for LJMWrapper {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(LJMWrapper::dummy())
    }
}

/// Taken from: [LJM ErrorCodes](https://labjack.com/pages/support?doc=%2Fsoftware-driver%2Fljm-users-guide%2Ferror-codes%2F)
///
/// > Note:
/// > We ignore the 0 value as NoError, as
/// > we replace it with a rust Result type.
#[derive(Debug)]
pub enum LJMErrorCode {
    LJMWarning(i32),
    // 200-399
    LJMModbusError(i32),
    // 1200-1216
    LJMLibraryError(i32), // 1220-1399

    DeviceError(i32),
    // 2000-2999
    UserError(i32), // 3900-3999

    Unknown(i32), // For any values outside these ranges.
}

#[derive(Debug)]
pub enum LJMError {
    StartupError(libloading::Error),
    ErrorCode(LJMErrorCode),
    LibraryError(String),
    LibloadingError(libloading::Error),
    Uninitialized,
}

impl From<libloading::Error> for LJMError {
    fn from(value: libloading::Error) -> Self {
        LJMError::LibloadingError(value)
    }
}

impl LJMWrapper {
    pub(crate) fn encode_error(error_code: i32) -> LJMErrorCode {
        match error_code {
            200..=399 => LJMErrorCode::LJMWarning(error_code - 200),
            1200..=1216 => LJMErrorCode::LJMModbusError(error_code - 1200),
            1220..=1399 => LJMErrorCode::LJMLibraryError(error_code - 1220),
            2000..=2999 => LJMErrorCode::DeviceError(error_code - 2000),
            3900..=3999 => LJMErrorCode::UserError(error_code - 3900),
            _ => LJMErrorCode::Unknown(error_code),
        }
    }

    pub(crate) fn error_code<T>(value: T, error_code: i32) -> Result<T, LJMError> {
        if error_code != 0 {
            return Err(LJMError::ErrorCode(LJMWrapper::encode_error(error_code)));
        }

        Ok(value)
    }

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

    pub fn wrapper_reference() -> &'static LJMWrapper {
        &LJM_DUMMY
    }

    fn get_library(&self) -> Result<&Library, LJMError> {
        match &self.library {
            Some(val) => Ok(val),
            None => Err(LJMError::Uninitialized),
        }
    }

    unsafe fn get_c_function<T>(&self, name: &[u8]) -> Result<Symbol<T>, LJMError> {
        let library = self.get_library()?;

        match library.get::<T>(name) {
            Ok(v) => Ok(v),
            Err(e) => Err(e.into()),
        }
    }

    const fn dummy() -> Self {
        Self { library: None }
    }

    pub fn is_initialised(&self) -> bool {
        self.library.is_some()
    }

    /// `unsafe`
    /// Initializes a labjack interface with the static library.
    ///
    /// # Safety
    /// This value is unsafe as it calls the underlying C library.
    /// The library is found at default paths, or at an overriden location
    /// specified by the `path` argument.
    pub unsafe fn init(path: Option<String>) -> Result<Self, LJMError> {
        let library: Library = unsafe {
            let library_path = path.unwrap_or_else(LJMWrapper::get_library_path);

            match Library::new(library_path) {
                Ok(library) => library,
                Err(error) => return Err(LJMError::StartupError(error)),
            }
        };

        Ok(LJMWrapper {
            library: Some(library),
        })
    }

    /// Converts a MODBUS name to its address and type
    /// Returns a tuple of (address, type) in (i32, i32) format.
    /// Verifiable with: - [LabJack Modbus Map](https://labjack.com/pages/support/?doc=/datasheets/t-series-datasheet/31-modbus-map-t-series-datasheet/)
    #[doc(alias = "LJM_NameToAddress")]
    pub fn name_to_address<T>(&self, identifier: T) -> Result<(i32, i32), LJMError>
    where
        T: ToString,
    {
        let n_to_addr: Symbol<extern "C" fn(*const c_char, *mut i32, *mut i32) -> i32> =
            unsafe { self.get_c_function(b"LJM_NameToAddress")? };

        let name = CString::new(identifier.to_string()).expect("CString conversion failed");
        let mut address: i32 = 0;
        let mut typ: i32 = 0;

        let error_code = n_to_addr(name.as_ptr(), &mut address, &mut typ);

        LJMWrapper::error_code((address, typ), error_code)
    }

    /// Digitally writes to address
    /// Takes a handle to the labjack, the name to be written and the value to be written.
    /// Does not return a value.
    #[doc(alias = "LJM_eWriteName")]
    pub fn write_name(
        &self,
        handle: i32,
        name_to_write: String,
        value_to_write: u32,
    ) -> Result<(), LJMError> {
        let d_write_to_addr: Symbol<extern "C" fn(i32, *const c_char, c_double) -> i32> =
            unsafe { self.get_c_function(b"LJM_eWriteName")? };

        let ntw = CString::new(name_to_write).expect("CString conversion failed");
        let vtw = c_double::from(value_to_write);

        let error_code = d_write_to_addr(handle, ntw.as_ptr(), vtw);

        LJMWrapper::error_code((), error_code)
    }

    /// Reads from a labjack given the handle and name to read.
    /// Returns an f64 value that is read from the labjack.
    #[doc(alias = "LJM_eReadName")]
    pub fn read_name(&self, handle: i32, name_to_read: String) -> Result<f64, LJMError> {
        let d_read_from_aadr: Symbol<extern "C" fn(i32, *const c_char, *mut c_double) -> i32> =
            unsafe { self.get_c_function(b"LJM_eReadName")? };

        let ntr = CString::new(name_to_read).expect("CString conversion failed");
        let mut vtr = c_double::from(-1);

        let error_code = d_read_from_aadr(handle, ntr.as_ptr(), &mut vtr);

        LJMWrapper::error_code(vtr, error_code)
    }

    /// Opens a LabJack and returns the handle id as an i32.
    #[doc(alias = "LJM_OpenS")]
    pub fn open_jack(
        &self,
        device_type: DeviceType,
        connection_type: ConnectionType,
        identifier: String,
    ) -> Result<i32, LJMError> {
        let open_s: Symbol<
            extern "C" fn(*const c_char, *const c_char, *const c_char, *mut i32) -> i32,
        > = unsafe { self.get_c_function(b"LJM_OpenS")? };

        let device_type = CString::new(device_type.to_string())
            .expect("Device Type :: CString conversion failed");
        let connection_type = CString::new(connection_type.to_string())
            .expect("Connection Type :: CString conversion failed");
        let identifier =
            CString::new(identifier).expect("LabJack Identifier :: CString conversion failed");

        let mut handle_id: i32 = 0;

        let error_code = open_s(
            device_type.as_ptr(),
            connection_type.as_ptr(),
            identifier.as_ptr(),
            &mut handle_id,
        );

        LJMWrapper::error_code(handle_id, error_code)
    }

    /// Closes a LabJack given it's handle id as an i32.
    #[doc(alias = "LJM_Close")]
    pub fn close_jack(&self, handle_id: i32) -> Result<i32, LJMError> {
        let close: Symbol<extern "C" fn(i32) -> i32> =
            unsafe { self.get_c_function(b"LJM_Close")? };

        LJMWrapper::error_code(handle_id, close(handle_id))
    }

    /// Closes all LabJacks connected.
    #[doc(alias = "LJM_CloseAll")]
    pub fn close_all(&self, handle_id: i32) -> Result<i32, LJMError> {
        let close_all: Symbol<extern "C" fn() -> i32> =
            unsafe { self.get_c_function(b"LJM_CloseAll")? };

        LJMWrapper::error_code(handle_id, close_all())
    }

    /// Converts an IPV4 numerical representation, outputting the corresponding
    /// decimal-dot notation for it.
    ///
    /// # Safety
    /// This function is unsafe due to C pointer recovery. Different systems
    /// will handle this behaviour differently, use with caution. Test experimentally,
    /// before ever using in a production environment.
    #[doc(alias = "LJM_NumberToIP")]
    pub unsafe fn number_to_ip(&self, number: i32) -> Result<String, LJMError> {
        let d_number_to_ip: Symbol<extern "C" fn(*const c_uint, *mut c_char) -> i32> =
            unsafe { self.get_c_function(b"LJM_NumberToIP")? };

        let number: c_uint = c_uint::try_from(number).map_err(|error| {
            LJMError::LibraryError(format!(
                "Unable to convert number into C unsigned integer. {}",
                error
            ))
        })?;

        let ip_address = CString::new("000.000.000.000").expect("CString conversion failed");
        let ip_pointer = ip_address.into_raw();

        let error_code = d_number_to_ip(&number, ip_pointer);

        let retrieved_pointer = unsafe { CString::from_raw(ip_pointer) };
        let recovered_ip = retrieved_pointer.into_string().map_err(|error| {
            LJMError::LibraryError(format!("Unable to retrieve IP pointer. {}", error))
        })?;

        LJMWrapper::error_code(recovered_ip, error_code)
    }

    /// Informs regarding device connection type
    #[doc(alias = "LJM_GetHandleInfo")]
    pub fn get_handle_info(&self, handle: i32) -> Result<DeviceHandleInfo, LJMError> {
        let get_handle_info: Symbol<
            extern "C" fn(i32, *mut i32, *mut i32, *mut i32, *mut i32, *mut i32, *mut i32) -> i32,
        > = unsafe { self.get_c_function(b"LJM_GetHandleInfo")? };

        let mut device_type: i32 = 0;
        let mut connection_type: i32 = 0;
        let mut serial_number: i32 = 0;
        let mut ip_address: i32 = 0;
        let mut port: i32 = 0;
        let mut max_bytes_per_megabyte: i32 = 0;

        let error_code = get_handle_info(
            handle,
            &mut device_type,
            &mut connection_type,
            &mut serial_number,
            &mut ip_address,
            &mut port,
            &mut max_bytes_per_megabyte,
        );

        LJMWrapper::error_code(
            DeviceHandleInfo {
                device_type: DeviceType::from(device_type),
                connection_type: ConnectionType::from(connection_type),
                ip_address,
                serial_number,
                port,
                max_bytes_per_megabyte,
            },
            error_code,
        )
    }

    /// Returns actual device scan rate (chosen by LabJack)
    #[doc(alias = "LJM_eStreamStart")]
    pub fn stream_start<T>(
        &self,
        handle: i32,
        scans_per_read: i32,
        suggested_scan_rate: f64,
        streams: Vec<T>,
    ) -> Result<f64, LJMError>
    where
        T: ToString + Display,
    {
        let stream_start: Symbol<extern "C" fn(i32, i32, i32, *const i32, *mut c_double) -> i32> =
            unsafe { self.get_c_function(b"LJM_eStreamStart")? };

        let addresses_result: Result<Vec<i32>, LJMError> =
            streams.iter().try_fold(Vec::new(), |mut acc, a| {
                let address = self.name_to_address(a)?.0;
                acc.push(address);
                Ok(acc)
            });

        let addresses = addresses_result?;

        let addr_slice: &[i32] = &addresses;
        let mut scan_rate: f64 = suggested_scan_rate;

        let error_code = stream_start(
            handle,
            scans_per_read,
            addresses.len() as i32,
            addr_slice.as_ptr(),
            &mut scan_rate,
        );

        LJMWrapper::error_code(scan_rate, error_code)
    }
}
