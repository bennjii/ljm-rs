use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub enum DeviceType {
    T4,
    T7,
    T8,
    TSERIES,
    DIGIT,
    ANY,
    EMULATED(i32),
    UNKNOWN(i32),
}

impl From<i32> for DeviceType {
    fn from(value: i32) -> Self {
        match value {
            4 => DeviceType::T4,
            7 => DeviceType::T7,
            8 => DeviceType::T8,
            200 => DeviceType::DIGIT,
            -999..=-1 => DeviceType::EMULATED(value),
            value => DeviceType::UNKNOWN(value),
        }
    }
}

#[derive(Clone, Debug)]
pub enum ConnectionType {
    USB,
    ETHERNET,
    WIFI,
    ANY,
    UNKNOWN(i32),
}

impl From<i32> for ConnectionType {
    fn from(value: i32) -> Self {
        match value {
            1 => ConnectionType::USB,
            3 => ConnectionType::ETHERNET,
            4 => ConnectionType::WIFI,
            value => ConnectionType::UNKNOWN(value),
        }
    }
}

#[derive(Clone, Debug)]
pub struct DeviceHandleInfo {
    pub device_type: DeviceType,
    pub connection_type: ConnectionType,
    pub ip_address: i32,

    pub max_bytes_per_megabyte: i32,
    pub serial_number: i32,
    pub port: i32,
}

impl Display for DeviceHandleInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // DT on CT @ ...B/MB
        // 000.000.000:0000
        // SERIAL_NUMBER

        write!(
            f,
            "{} on {} @ {}B/MB\n{}:{} => {}",
            self.device_type,
            self.connection_type,
            self.max_bytes_per_megabyte,
            self.ip_address,
            self.port,
            self.serial_number
        )
    }
}

impl Display for DeviceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            DeviceType::T4 => "T4".to_string(),
            DeviceType::T7 => "T7".to_string(),
            DeviceType::T8 => "T8".to_string(),
            DeviceType::TSERIES => "TSERIES".to_string(),

            DeviceType::DIGIT => "DIGIT".to_string(),
            DeviceType::ANY => "ANY".to_string(),

            DeviceType::EMULATED(value) => format!("EMULATED::[{value}]"),
            DeviceType::UNKNOWN(value) => format!("ANY::[{value}]"),
        };

        write!(f, "{}", str)
    }
}

impl Display for ConnectionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            ConnectionType::USB => "USB",
            ConnectionType::WIFI => "WIFI",
            ConnectionType::ETHERNET => "ETHERNET",
            ConnectionType::ANY | ConnectionType::UNKNOWN(_) => "ANY",
        }
        .to_string();
        write!(f, "{}", str)
    }
}
