use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub enum DeviceType {
    T4,
    T7,
    T8,
    TSERIES,
    DIGIT,
    ANY,
    UNKNOWN(i32),
}

impl From<i32> for DeviceType {
    fn from(value: i32) -> Self {
        match value {
            4 => DeviceType::T4,
            7 => DeviceType::T7,
            8 => DeviceType::T8,
            200 => DeviceType::DIGIT,
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
    pub serial_number: i32,
    pub ip_address: i32,
    pub port: i32,
    pub max_bytes_per_megabyte: i32,
}

impl Display for DeviceHandleInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // DT on CT @ ...B/MB
        // 000.000.000:0000
        // SERIAL_NUMBER

        write!(
            f,
            "{:?} on {:?} @ {}B/MB\n{}:{}\n{}",
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
            DeviceType::T4 => "T4",
            DeviceType::T7 => "T7",
            DeviceType::T8 => "T8",
            DeviceType::TSERIES => "TSERIES",
            DeviceType::DIGIT => "DIGIT",
            DeviceType::ANY | DeviceType::UNKNOWN(_) => "ANY",
        }
        .to_string();
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
