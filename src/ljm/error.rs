use std::fmt::Debug;

use crate::LJMLibrary;

/// Taken from: [LJM ErrorCodes](https://labjack.com/pages/support?doc=%2Fsoftware-driver%2Fljm-users-guide%2Ferror-codes%2F)
///
/// > Note:
/// > We ignore the 0 value as NoError, as
/// > we replace it with a rust Result type.
#[derive(PartialOrd, PartialEq)]
pub enum LJMErrorCode {
    LJMWarning(i32),
    // 200-399
    LJMModbusError(i32),
    // 1200-1216
    LJMLibraryError(i32),
    // 1220-1399
    DeviceError(i32),
    // 2000-2999
    UserError(i32),
    // 3900-3999
    Unknown(i32), // For any values outside these ranges.
}

impl From<&LJMErrorCode> for i32 {
    fn from(value: &LJMErrorCode) -> Self {
        match value {
            LJMErrorCode::LJMWarning(error_code) => error_code + 200,
            LJMErrorCode::LJMModbusError(error_code) => error_code + 1200,
            LJMErrorCode::LJMLibraryError(error_code) => error_code + 1220,
            LJMErrorCode::DeviceError(error_code) => error_code + 2000,
            LJMErrorCode::UserError(error_code) => error_code + 3900,
            LJMErrorCode::Unknown(error_code) => *error_code,
        }
    }
}

impl From<i32> for LJMErrorCode {
    fn from(error_code: i32) -> Self {
        match error_code {
            200..=399 => LJMErrorCode::LJMWarning(error_code - 200),
            1200..=1216 => LJMErrorCode::LJMModbusError(error_code - 1200),
            1220..=1399 => LJMErrorCode::LJMLibraryError(error_code - 1220),
            2000..=2999 => LJMErrorCode::DeviceError(error_code - 2000),
            3900..=3999 => LJMErrorCode::UserError(error_code - 3900),
            _ => LJMErrorCode::Unknown(error_code),
        }
    }
}

impl Debug for LJMErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code: i32 = self.into();

        let name = match self {
            LJMErrorCode::LJMWarning(_) => "LabJackWarning",
            LJMErrorCode::LJMModbusError(_) => "ModbusError",
            LJMErrorCode::LJMLibraryError(_) => "LibraryError",
            LJMErrorCode::DeviceError(_) => "DeviceError",
            LJMErrorCode::UserError(_) => "UserError",
            LJMErrorCode::Unknown(_) => "UnknownError",
        };

        write!(f, "{}({})", name, code)
    }
}

pub enum LJMError {
    #[cfg(feature = "dynlink")]
    StartupError(libloading::Error),
    #[cfg(feature = "dynlink")]
    LibloadingError(libloading::Error),

    WrapperInvalid(LJMLibrary),
    ErrorCode(LJMErrorCode, String),
    LibraryError(String),

    Uninitialized,
    StreamNotStarted,
    ScriptNotSet,
    ScriptStillRunning,
    BadState,
    NoCoverage,

    PoisonedLock,
    CStringConversionFailed,
}

#[cfg(feature = "dynlink")]
impl From<libloading::Error> for LJMError {
    fn from(value: libloading::Error) -> Self {
        LJMError::LibloadingError(value)
    }
}

impl Debug for LJMError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                #[cfg(feature = "dynlink")]
                LJMError::StartupError(error) => format!("StartupError::{:?}", error),
                #[cfg(feature = "dynlink")]
                LJMError::LibloadingError(error) => format!("LibraryLoadingError::{:?}", error),

                LJMError::ErrorCode(error, value) => format!("LJMError::{:?} ({})", error, value),
                LJMError::LibraryError(error) => format!("LibraryError::{:?}", error),
                LJMError::WrapperInvalid(_) => "WrapperInvalidError".to_string(),
                LJMError::BadState => "BadStateError".to_string(),
                LJMError::NoCoverage => "NoCoverageError".to_string(),

                LJMError::Uninitialized => "UninitializedError".to_string(),
                LJMError::StreamNotStarted => "StreamNotStartedError".to_string(),
                LJMError::ScriptStillRunning => "ScriptStillRunningError".to_string(),
                LJMError::ScriptNotSet => "ScriptNotSetError".to_string(),
                LJMError::PoisonedLock => "PoisonedLockError".to_string(),
                LJMError::CStringConversionFailed => "CStringConversionFailed".to_string(),
            }
        )
    }
}
