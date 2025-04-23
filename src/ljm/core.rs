use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::sync::OnceLock;
use std::{
    ffi::{c_char, c_uint, CString},
    os::raw::c_double,
};
#[cfg(feature = "stream")]
use std::{fmt::Display, sync::RwLock};

#[cfg(feature = "dynlink")]
use libloading::{Library, Symbol};

#[cfg(feature = "staticlink")]
use crate::lib;

use crate::{
    ljm::handle::{ConnectionType, DeviceHandleInfo, DeviceType},
    LJMError,
};

#[cfg(feature = "stream")]
use crate::ljm::stream::LJMStream;
#[cfg(feature = "lua")]
use crate::lua::LJMLua;

static LJM_WRAPPER: OnceLock<LJMLibrary> = OnceLock::new();

pub struct LJMLibrary {
    #[cfg(feature = "dynlink")]
    pub library: Option<Library>,

    #[cfg(feature = "stream")]
    stream: RwLock<HashMap<i32, LJMStream>>,

    // A device can only have one module at a time.
    #[cfg(feature = "lua")]
    module: RwLock<Option<LJMLua>>,
}

impl Debug for LJMLibrary {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "LJMWrapper")
    }
}

impl LJMLibrary {
    pub(crate) fn error_code<T>(value: T, error_code: i32) -> Result<T, LJMError> {
        if error_code != 0 {
            return Err(LJMError::ErrorCode(
                error_code.into(),
                LJMLibrary::error_to_string(error_code)?,
            ));
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

    #[cfg(feature = "dynlink")]
    fn get_library<'a>() -> Result<&'a Library, LJMError> {
        LJM_WRAPPER
            .get()
            .ok_or(LJMError::Uninitialized)?
            .library
            .as_ref()
            .ok_or(LJMError::Uninitialized)
    }

    #[cfg(feature = "dynlink")]
    unsafe fn get_c_function<T>(name: &[u8]) -> Result<Symbol<T>, LJMError> {
        let library = LJMLibrary::get_library()?;

        match library.get::<T>(name) {
            Ok(v) => Ok(v),
            Err(e) => Err(e.into()),
        }
    }

    #[cfg(all(feature = "dynlink", not(feature = "staticlink")))]
    pub fn is_initialised(&self) -> bool {
        self.library.is_some()
    }

    #[cfg(all(feature = "staticlink", not(feature = "dynlink")))]
    pub fn is_initialised(&self) -> bool {
        true
    }

    /// `unsafe`
    /// Initializes a labjack interface with the static library.
    ///
    /// # Safety
    /// This value is unsafe as it calls the underlying C library.
    /// The library is found at default paths, or at an overriden location
    /// specified by the `path` argument.
    #[cfg(all(feature = "dynlink", not(feature = "staticlink")))]
    pub unsafe fn init(path: Option<String>) -> Result<(), LJMError> {
        let library: Library = unsafe {
            let library_path = path.unwrap_or_else(LJMLibrary::get_library_path);

            match Library::new(library_path) {
                Ok(library) => library,
                Err(error) => return Err(LJMError::StartupError(error)),
            }
        };

        LJM_WRAPPER
            .set(LJMLibrary {
                library: Some(library),
                #[cfg(feature = "stream")]
                stream: RwLock::new(None),
                #[cfg(feature = "lua")]
                module: RwLock::new(None),
            })
            .map_err(LJMError::WrapperInvalid)
    }

    #[cfg(all(feature = "staticlink", not(feature = "dynlink")))]
    pub unsafe fn init() -> Result<(), LJMError> {
        LJM_WRAPPER
            .set(LJMLibrary {
                #[cfg(feature = "stream")]
                stream: RwLock::new(HashMap::new()),
                #[cfg(feature = "lua")]
                module: RwLock::new(None),
            })
            .map_err(LJMError::WrapperInvalid)
    }

    #[doc(alias = "LJM_ErrorToString")]
    pub fn error_to_string(error_code: i32) -> Result<String, LJMError> {
        // Allocate using stack. LJM States will not overflow.
        // https://support.labjack.com/docs/errortostring-ljm-user-s-guide#ErrorToString
        let mut buffer: [c_char; 256] = [0; 256];
        #[cfg(feature = "staticlink")]
        unsafe {
            lib::LJM_ErrorToString(error_code, buffer.as_mut_ptr())
        };
        #[cfg(feature = "dynlink")]
        let err_to_str: Symbol<extern "C" fn(i32, *mut c_char)> =
            unsafe { LJMLibrary::get_c_function(b"LJM_ErrorToString")? };
        #[cfg(feature = "dynlink")]
        err_to_str(error_code, buffer.as_mut_ptr());

        let as_vec = buffer.into_iter().map(|v| v as u8).collect::<Vec<u8>>();

        match std::str::from_utf8(as_vec.as_slice()) {
            Ok(v) => Ok(v.to_string()),
            Err(e) => Err(LJMError::LibraryError(format!(
                "Unable to retrieve buffer pointer. {}",
                e
            ))),
        }
    }

    /// Converts a MODBUS name to its address and type
    /// Returns a tuple of (address, type) in (i32, i32) format.
    /// Verifiable with: - [LabJack Modbus Map](https://labjack.com/pages/support/?doc=/datasheets/t-series-datasheet/31-modbus-map-t-series-datasheet/)
    #[doc(alias = "LJM_NameToAddress")]
    pub fn name_to_address<T>(identifier: T) -> Result<(i32, i32), LJMError>
    where
        T: ToString,
    {
        #[cfg(feature = "dynlink")]
        let n_to_addr: Symbol<extern "C" fn(*const c_char, *mut i32, *mut i32) -> i32> =
            unsafe { LJMLibrary::get_c_function(b"LJM_NameToAddress")? };

        let name =
            CString::new(identifier.to_string()).map_err(|_| LJMError::CStringConversionFailed)?;
        let mut address: i32 = 0;
        let mut typ: i32 = 0;

        #[cfg(feature = "dynlink")]
        let error_code = n_to_addr(name.as_ptr(), &mut address, &mut typ);
        #[cfg(feature = "staticlink")]
        let error_code = unsafe { lib::LJM_NameToAddress(name.as_ptr(), &mut address, &mut typ) };

        LJMLibrary::error_code((address, typ), error_code)
    }

    /// Digitally writes to address
    /// Takes a handle to the labjack, the name to be written and the value to be written.
    /// Does not return a value.
    #[doc(alias = "LJM_eWriteName")]
    pub fn write_name<T: Into<Vec<u8>>, V: Into<c_double>>(
        handle: i32,
        name_to_write: T,
        value_to_write: V,
    ) -> Result<(), LJMError> {
        #[cfg(feature = "dynlink")]
        let d_write_to_addr: Symbol<extern "C" fn(i32, *const c_char, c_double) -> i32> =
            unsafe { LJMLibrary::get_c_function(b"LJM_eWriteName")? };

        let ntw = CString::new(name_to_write).map_err(|_| LJMError::CStringConversionFailed)?;
        let vtw = c_double::from(value_to_write.into());

        #[cfg(feature = "dynlink")]
        let error_code = d_write_to_addr(handle, ntw.as_ptr(), vtw);
        #[cfg(feature = "staticlink")]
        let error_code = unsafe { lib::LJM_eWriteName(handle, ntw.as_ptr(), vtw) };

        LJMLibrary::error_code((), error_code)
    }

    #[doc(alias = "LJM_eWriteName")]
    pub fn write_addr<V: Into<c_double>>(
        handle: i32,
        address: i32,
        data_type: i32,
        value_to_write: V,
    ) -> Result<(), LJMError> {
        #[cfg(feature = "dynlink")]
        let d_write_to_addr: Symbol<extern "C" fn(i32, i32, i32, c_double) -> i32> =
            unsafe { LJMLibrary::get_c_function(b"LJM_eWriteAddress")? };

        let vtw = c_double::from(value_to_write.into());

        #[cfg(feature = "dynlink")]
        let error_code = d_write_to_addr(handle, address, data_type, vtw);
        #[cfg(feature = "staticlink")]
        let error_code = unsafe { lib::LJM_eWriteAddress(handle, address, data_type, vtw) };

        LJMLibrary::error_code((), error_code)
    }

    #[doc(alias = "LJM_eWriteNameByteArray")]
    pub fn write_name_byte_array<T: Into<Vec<u8>>, B: Into<Vec<u8>>>(
        handle: i32,
        name_to_write: T,
        size: i32,
        bytes: B,
    ) -> Result<(), LJMError> {
        #[cfg(feature = "dynlink")]
        let d_write_name_byte_array: Symbol<
            extern "C" fn(i32, *const c_char, i32, *const c_char, *mut i32) -> i32,
        > = unsafe { LJMLibrary::get_c_function(b"LJM_eWriteNameByteArray")? };

        let btw = CString::new(bytes).map_err(|_| LJMError::CStringConversionFailed)?; // Bytes-To-Write
        let ntw = CString::new(name_to_write).map_err(|_| LJMError::CStringConversionFailed)?; // Name-To-Write

        let mut error_addr: i32 = 0;
        #[cfg(feature = "dynlink")]
        let error_code =
            d_write_name_byte_array(handle, ntw.as_ptr(), size, btw.as_ptr(), &mut error_addr);
        #[cfg(feature = "staticlink")]
        let error_code = unsafe {
            lib::LJM_eWriteNameByteArray(handle, ntw.as_ptr(), size, btw.as_ptr(), &mut error_addr)
        };

        LJMLibrary::error_code((), error_code)
    }

    #[doc(alias = "LJM_eReadNameByteArray")]
    pub fn read_name_byte_array<T: Into<Vec<u8>>>(
        handle: i32,
        name_to_read: T,
        size: i32,
    ) -> Result<Vec<u8>, LJMError> {
        #[cfg(feature = "dynlink")]
        let d_write_name_byte_array: Symbol<
            extern "C" fn(i32, *const c_char, i32, *mut u8, *mut i32) -> i32,
        > = unsafe { LJMLibrary::get_c_function(b"LJM_eReadNameByteArray")? };

        // Only allocate as much as we need, and do so such that
        // it will be initialized when written to - the C lib
        // does not know the context, so we'll Box it (heap-alloc)
        // with a static size of `size`.
        #[cfg(feature = "staticlink")]
        let mut buffer: Box<[c_char]> = vec![c_char::from(0); size as usize].into_boxed_slice();
        #[cfg(feature = "dynlink")]
        let mut buffer: Box<[u8]> = vec![0u8; size as usize].into_boxed_slice();

        let ntr = CString::new(name_to_read).map_err(|_| LJMError::CStringConversionFailed)?; // Name-To-Write

        let mut error_addr: i32 = 0;
        #[cfg(feature = "dynlink")]
        let error_code = d_write_name_byte_array(
            handle,
            ntr.as_ptr(),
            size,
            buffer.as_mut_ptr(),
            &mut error_addr,
        );
        #[cfg(feature = "staticlink")]
        let error_code = unsafe {
            lib::LJM_eWriteNameByteArray(
                handle,
                ntr.as_ptr(),
                size,
                buffer.as_mut_ptr(),
                &mut error_addr,
            )
        };

        #[cfg(feature = "staticlink")]
        let as_vec = buffer.to_vec().into_iter().map(|v| v as u8).collect();

        #[cfg(feature = "dynlink")]
        let as_vec = buffer.into_vec();

        LJMLibrary::error_code(as_vec, error_code)
    }

    /// Reads from a labjack given the handle and name to read.
    /// Returns an f64 value that is read from the labjack.
    #[doc(alias = "LJM_eReadName")]
    pub fn read_name<T: Into<Vec<u8>>>(handle: i32, name_to_read: T) -> Result<f64, LJMError> {
        #[cfg(feature = "dynlink")]
        let d_read_from_aadr: Symbol<
            extern "C" fn(i32, *const c_char, *mut c_double) -> i32,
        > = unsafe { LJMLibrary::get_c_function(b"LJM_eReadName")? };

        let ntr = CString::new(name_to_read).map_err(|_| LJMError::CStringConversionFailed)?;
        let mut vtr = c_double::from(-1);

        #[cfg(feature = "dynlink")]
        let error_code = d_read_from_aadr(handle, ntr.as_ptr(), &mut vtr);
        #[cfg(feature = "staticlink")]
        let error_code = unsafe { lib::LJM_eReadName(handle, ntr.as_ptr(), &mut vtr) };

        LJMLibrary::error_code(vtr, error_code)
    }

    #[doc(alias = "LJM_eReadName")]
    pub fn read_addr(handle: i32, addr: i32, data_type: i32) -> Result<f64, LJMError> {
        #[cfg(feature = "dynlink")]
        let d_read_from_aadr: Symbol<extern "C" fn(i32, i32, i32, *mut c_double) -> i32> =
            unsafe { LJMLibrary::get_c_function(b"LJM_eReadAddress")? };

        let mut vtr = c_double::from(-1);

        #[cfg(feature = "dynlink")]
        let error_code = d_read_from_aadr(handle, addr, data_type, &mut vtr);
        #[cfg(feature = "staticlink")]
        let error_code = unsafe { lib::LJM_eReadAddress(handle, addr, data_type, &mut vtr) };

        LJMLibrary::error_code(vtr, error_code)
    }

    /// Opens a LabJack and returns the handle id as an i32.
    #[doc(alias = "LJM_OpenS")]
    pub fn open_jack<T: Into<Vec<u8>>>(
        device_type: DeviceType,
        connection_type: ConnectionType,
        identifier: T,
    ) -> Result<i32, LJMError> {
        #[cfg(feature = "dynlink")]
        let open_s: Symbol<
            extern "C" fn(*const c_char, *const c_char, *const c_char, *mut i32) -> i32,
        > = unsafe { LJMLibrary::get_c_function(b"LJM_OpenS")? };

        let device_type =
            CString::new(device_type.to_string()).map_err(|_| LJMError::CStringConversionFailed)?;
        let connection_type = CString::new(connection_type.to_string())
            .map_err(|_| LJMError::CStringConversionFailed)?;
        let identifier = CString::new(identifier).map_err(|_| LJMError::CStringConversionFailed)?;

        let mut handle_id: i32 = 0;

        #[cfg(feature = "dynlink")]
        let error_code = open_s(
            device_type.as_ptr(),
            connection_type.as_ptr(),
            identifier.as_ptr(),
            &mut handle_id,
        );
        #[cfg(feature = "staticlink")]
        let error_code = unsafe {
            lib::LJM_OpenS(
                device_type.as_ptr(),
                connection_type.as_ptr(),
                identifier.as_ptr(),
                &mut handle_id,
            )
        };

        LJMLibrary::error_code(handle_id, error_code)
    }

    /// Closes a LabJack given it's handle id as an i32.
    #[doc(alias = "LJM_Close")]
    pub fn close_jack(handle_id: i32) -> Result<i32, LJMError> {
        #[cfg(feature = "dynlink")]
        let close: Symbol<extern "C" fn(i32) -> i32> =
            unsafe { LJMLibrary::get_c_function(b"LJM_Close")? };

        #[cfg(feature = "dynlink")]
        let error_code = close(handle_id);

        #[cfg(feature = "staticlink")]
        let error_code = unsafe { lib::LJM_Close(handle_id) };

        LJMLibrary::error_code(handle_id, error_code)
    }

    /// Closes all LabJacks connected.
    #[doc(alias = "LJM_CloseAll")]
    pub fn close_all() -> Result<(), LJMError> {
        #[cfg(feature = "dynlink")]
        let close_all: Symbol<extern "C" fn() -> i32> =
            unsafe { LJMLibrary::get_c_function(b"LJM_CloseAll")? };

        #[cfg(feature = "dynlink")]
        let error_code = close_all();

        #[cfg(feature = "staticlink")]
        let error_code = unsafe { lib::LJM_CloseAll() };

        LJMLibrary::error_code((), error_code)
    }

    /// Converts an IPV4 numerical representation, outputting the corresponding
    /// decimal-dot notation for it.
    ///
    /// # Safety
    /// This function is unsafe due to C pointer recovery. Different systems
    /// will handle this behaviour differently, use with caution. Test experimentally,
    /// before ever using in a production environment.
    #[doc(alias = "LJM_NumberToIP")]
    pub unsafe fn number_to_ip(number: i32) -> Result<String, LJMError> {
        #[cfg(feature = "dynlink")]
        let d_number_to_ip: Symbol<
            extern "C" fn(::std::os::raw::c_uint, *mut c_char) -> i32,
        > = unsafe { LJMLibrary::get_c_function(b"LJM_NumberToIP")? };

        let number: c_uint = c_uint::try_from(number).map_err(|error| {
            LJMError::LibraryError(format!(
                "Unable to convert number into C unsigned integer. {}",
                error
            ))
        })?;

        let ip_address =
            CString::new("000.000.000.000").map_err(|_| LJMError::CStringConversionFailed)?;
        let ip_pointer = ip_address.into_raw();

        #[cfg(feature = "dynlink")]
        let error_code = d_number_to_ip(number, ip_pointer);
        #[cfg(feature = "staticlink")]
        let error_code = unsafe { lib::LJM_NumberToIP(number, ip_pointer) };

        let retrieved_pointer = unsafe { CString::from_raw(ip_pointer) };
        let recovered_ip = retrieved_pointer.into_string().map_err(|error| {
            LJMError::LibraryError(format!("Unable to retrieve IP pointer. {}", error))
        })?;

        LJMLibrary::error_code(recovered_ip, error_code)
    }

    /// Informs regarding device connection type
    #[doc(alias = "LJM_GetHandleInfo")]
    pub fn get_handle_info(handle: i32) -> Result<DeviceHandleInfo, LJMError> {
        #[cfg(feature = "dynlink")]
        let get_handle_info: Symbol<
            extern "C" fn(i32, *mut i32, *mut i32, *mut i32, *mut i32, *mut i32, *mut i32) -> i32,
        > = unsafe { LJMLibrary::get_c_function(b"LJM_GetHandleInfo")? };

        let mut device_type: i32 = 0;
        let mut connection_type: i32 = 0;
        let mut serial_number: i32 = 0;
        let mut ip_address: i32 = 0;
        let mut port: i32 = 0;
        let mut max_bytes_per_megabyte: i32 = 0;

        #[cfg(feature = "dynlink")]
        let error_code = get_handle_info(
            handle,
            &mut device_type,
            &mut connection_type,
            &mut serial_number,
            &mut ip_address,
            &mut port,
            &mut max_bytes_per_megabyte,
        );
        #[cfg(feature = "staticlink")]
        let error_code = unsafe {
            lib::LJM_GetHandleInfo(
                handle,
                &mut device_type,
                &mut connection_type,
                &mut serial_number,
                &mut ip_address,
                &mut port,
                &mut max_bytes_per_megabyte,
            )
        };

        LJMLibrary::error_code(
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

    /// This returns a boolean for the stream state (on/off)
    /// for a specific device handle. There may still be streams
    /// activated for other devices, however the function will
    /// only give the stream state for the specified handle.
    #[cfg(feature = "stream")]
    pub fn is_stream_active(handle_id: i32) -> bool {
        let wrapper = LJM_WRAPPER.get();

        match wrapper {
            Some(w) => {
                let stream = w.stream.read();

                match stream {
                    Ok(s) => s.get(&handle_id).is_some(),
                    Err(_) => false,
                }
            }
            None => false,
        }
    }

    /// Starts a LJM Stream, stopped with `stream_stop`.
    /// Returns actual device scan rate (chosen by LabJack).
    ///
    /// As opposed to `stream_start`, accepts any Vec<T> where T: ToString.
    /// Such that, you may provide the string-representation of LJM addresses,
    /// permitting each can be decoded by LJMLibrary::name_to_address into a
    /// logical LJM address.
    ///
    /// `suggested_scan_rate` The scan rate forwarded to LJM which it will attempt to use
    ///
    #[doc(alias = "LJM_eStreamStart")]
    #[cfg(feature = "stream")]
    pub fn stream_start_addr<T>(
        handle: i32,
        scans_per_read: i32,
        suggested_scan_rate: f64,
        streams: Vec<T>,
    ) -> Result<f64, LJMError>
    where
        T: ToString + Display,
    {
        let addresses: Result<Vec<i32>, LJMError> =
            streams.iter().try_fold(Vec::new(), |mut acc, a| {
                let (address, _) = LJMLibrary::name_to_address(a)?;
                acc.push(address);
                Ok(acc)
            });

        LJMLibrary::stream_start(handle, scans_per_read, suggested_scan_rate, addresses?)
    }

    /// Starts a LJM Stream, stopped with `stream_stop`.
    /// Returns actual device scan rate (chosen by LabJack)
    ///
    /// `suggested_scan_rate` The scan rate forwarded to LJM which it will attempt to use
    ///
    #[doc(alias = "LJM_eStreamStart")]
    #[cfg(feature = "stream")]
    pub fn stream_start(
        handle: i32,
        scans_per_read: i32,
        suggested_scan_rate: f64,
        addresses: Vec<i32>,
    ) -> Result<f64, LJMError> {
        #[cfg(feature = "dynlink")]
        let stream_start: Symbol<
            extern "C" fn(i32, i32, i32, *const i32, *mut c_double) -> i32,
        > = unsafe { LJMLibrary::get_c_function(b"LJM_eStreamStart")? };

        let addr_slice: &[i32] = &addresses;
        let mut scan_rate: f64 = suggested_scan_rate;

        #[cfg(feature = "dynlink")]
        let error_code = stream_start(
            handle,
            scans_per_read,
            addresses.len() as i32,
            addr_slice.as_ptr(),
            &mut scan_rate,
        );
        #[cfg(feature = "staticlink")]
        let error_code = unsafe {
            lib::LJM_eStreamStart(
                handle,
                scans_per_read,
                addresses.len() as i32,
                addr_slice.as_ptr(),
                &mut scan_rate,
            )
        };

        // If we don't have an error we will initialize the stream
        if error_code == 0 {
            let wrapper = LJM_WRAPPER.get();

            let mut stream = wrapper
                .ok_or(LJMError::Uninitialized)?
                .stream
                .write()
                .map_err(|_| LJMError::PoisonedLock)?;

            stream.insert(
                handle,
                LJMStream {
                    scan_list: addresses,
                    scans_per_read,
                },
            );
        }

        LJMLibrary::error_code(scan_rate, error_code)
    }

    /// Stops an LJM Stream started with `stream_start`, returns the stream
    /// that was active when the function was called. If none, no stream was
    /// in place.
    #[doc(alias = "LJM_eStreamStop")]
    #[cfg(feature = "stream")]
    pub fn stream_stop(handle: i32) -> Result<Option<LJMStream>, LJMError> {
        #[cfg(feature = "dynlink")]
        let stream_stop: Symbol<extern "C" fn(i32) -> i32> =
            unsafe { LJMLibrary::get_c_function(b"LJM_eStreamStop")? };

        #[cfg(feature = "dynlink")]
        let error_code = stream_stop(handle);
        #[cfg(feature = "staticlink")]
        let error_code = unsafe { lib::LJM_eStreamStop(handle) };

        // Remove stream from active
        let stream = LJM_WRAPPER
            .get()
            .ok_or(LJMError::Uninitialized)?
            .stream
            .write()
            .map_err(|e| LJMError::LibraryError(e.to_string()))?
            .remove(&handle);

        LJMLibrary::error_code(stream, error_code)
    }

    /// Stops an LJM Stream started with `stream_start`
    #[doc(alias = "LJM_eStreamRead")]
    #[cfg(feature = "stream")]
    pub fn stream_read(handle: i32) -> Result<Vec<f64>, LJMError> {
        let lock = LJM_WRAPPER
            .get()
            .ok_or(LJMError::Uninitialized)?
            .stream
            .read()
            .map_err(|_| LJMError::StreamNotStarted)?;

        let stream_value = lock.get(&handle).ok_or(LJMError::StreamNotStarted)?;

        #[cfg(feature = "dynlink")]
        let stream_read: Symbol<extern "C" fn(i32, *mut f64, *mut i32, *mut i32) -> i32> =
            unsafe { LJMLibrary::get_c_function(b"LJM_eStreamRead")? };

        let mut dev_scan_backlog: i32 = 0;
        let mut ljm_scan_backlog: i32 = 0;

        // Length = ScansPerRead * NumberOfAddresses
        let scan_length = stream_value.scans_per_read as usize * stream_value.scan_list.len();

        let mut addr_slice = vec![0.0; scan_length];

        #[cfg(feature = "dynlink")]
        let error_code = stream_read(
            handle,
            addr_slice.as_mut_ptr(),
            &mut dev_scan_backlog,
            &mut ljm_scan_backlog,
        );
        #[cfg(feature = "staticlink")]
        let error_code = unsafe {
            lib::LJM_eStreamRead(
                handle,
                addr_slice.as_mut_ptr(),
                &mut dev_scan_backlog,
                &mut ljm_scan_backlog,
            )
        };

        LJMLibrary::error_code(addr_slice, error_code)
    }

    /// Digitally writes an integer config
    /// Does not return a value
    #[doc(alias = "LJM_WriteLibraryConfigS")]
    pub fn set_config<V: Into<c_double>, S: Into<String>>(
        config_name: S,
        config_value: V,
    ) -> Result<(), LJMError> {
        #[cfg(feature = "dynlink")]
        let d_write_to_addr: Symbol<extern "C" fn(*const c_char, c_double) -> i32> =
            unsafe { LJMLibrary::get_c_function(b"LJM_WriteLibraryConfigS")? };

        let ntw =
            CString::new(config_name.into()).map_err(|_| LJMError::CStringConversionFailed)?;
        let vtw = c_double::from(config_value.into());

        #[cfg(feature = "dynlink")]
        let error_code = d_write_to_addr(ntw.as_ptr(), vtw);
        #[cfg(feature = "staticlink")]
        let error_code = unsafe { lib::LJM_WriteLibraryConfigS(ntw.as_ptr(), vtw) };

        LJMLibrary::error_code((), error_code)
    }

    /// Reads from a labjack given the handle and name to read.
    /// Returns an f64 value that is rxead from the labjack.
    #[doc(alias = "LJM_ReadLibraryConfigS")]
    pub fn get_config(config_name: String) -> Result<f64, LJMError> {
        #[cfg(feature = "dynlink")]
        let d_read_library_config: Symbol<
            extern "C" fn(*const c_char, *mut c_double) -> i32,
        > = unsafe { LJMLibrary::get_c_function(b"LJM_ReadLibraryConfigS")? };

        let ntr = CString::new(config_name).map_err(|_| LJMError::CStringConversionFailed)?;
        let mut vtr = c_double::from(-1);

        #[cfg(feature = "dynlink")]
        let error_code = d_read_library_config(ntr.as_ptr(), &mut vtr);
        #[cfg(feature = "staticlink")]
        let error_code = unsafe { lib::LJM_ReadLibraryConfigS(ntr.as_ptr(), &mut vtr) };

        LJMLibrary::error_code(vtr, error_code)
    }

    #[cfg(all(feature = "lua", feature = "tokio"))]
    pub async fn set_module(handle: i32, module: LJMLua, debug: bool) -> Result<(), LJMError> {
        LJMLibrary::replace_module(handle, module)?;
        LJMLibrary::stop_module(handle).await?;
        LJMLibrary::start_module(handle, debug)
    }

    #[cfg(all(feature = "lua", not(feature = "tokio")))]
    pub fn set_module(handle: i32, module: LJMLua, debug: bool) -> Result<(), LJMError> {
        LJMLibrary::replace_module(handle, module)?;
        LJMLibrary::stop_module(handle)?;
        LJMLibrary::start_module(handle, debug)
    }

    #[cfg(feature = "lua")]
    fn start_module(handle: i32, debug: bool) -> Result<(), LJMError> {
        let wrapper = LJM_WRAPPER.get();
        let module = wrapper
            .ok_or(LJMError::Uninitialized)?
            .module
            .read()
            .map_err(|_| LJMError::PoisonedLock)?
            .clone()
            .ok_or(LJMError::ScriptNotSet)?;

        LJMLibrary::write_name(handle, "LUA_SOURCE_SIZE", module.size() as u32)?;
        LJMLibrary::write_name_byte_array(
            handle,
            "LUA_SOURCE_WRITE",
            module.size() as i32,
            module.script(),
        )?;

        if debug {
            LJMLibrary::write_name(handle, "LUA_DEBUG_ENABLE", 1)?;
            LJMLibrary::write_name(handle, "LUA_DEBUG_ENABLE_DEFAULT", 1)?;
        }

        LJMLibrary::write_name(handle, "LUA_RUN", 1)?;

        Ok(())
    }

    #[cfg(feature = "lua")]
    fn replace_module(handle: i32, module: LJMLua) -> Result<(), LJMError> {
        // If there is a script still running, we shouldn't replace anything.
        if LJMLibrary::module_running(handle)? {
            return Err(LJMError::ScriptStillRunning);
        }

        let wrapper = LJM_WRAPPER.get();
        wrapper
            .ok_or(LJMError::Uninitialized)?
            .module
            .write()
            .map(|mut rwg| {
                rwg.replace(module);
            })
            .map_err(|_| LJMError::PoisonedLock)
    }

    #[cfg(feature = "lua")]
    pub fn module_running(handle: i32) -> Result<bool, LJMError> {
        Ok(LJMLibrary::read_name(handle, "LUA_RUN")? == 1_f64)
    }

    #[cfg(all(feature = "lua", feature = "tokio"))]
    pub async fn stop_module(handle: i32) -> Result<(), LJMError> {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_millis(50));

        while LJMLibrary::module_running(handle)? {
            LJMLibrary::write_name(handle, "LUA_RUN", 0)?;
            interval.tick().await;
        }

        Ok(())
    }

    #[cfg(all(feature = "lua", not(feature = "tokio")))]
    pub fn stop_module(handle: i32) -> Result<(), LJMError> {
        while LJMLibrary::module_running(handle)? {
            LJMLibrary::write_name(handle, "LUA_RUN", 0)?;
            std::thread::sleep(std::time::Duration::from_millis(50))
        }

        Ok(())
    }
}
