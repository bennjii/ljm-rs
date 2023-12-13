use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub enum DeviceType {
    T4,
    T7,
    T8,
    DIGIT,
    UNKNOWN(i32)
}

impl From<i32> for DeviceType {
    fn from(value: i32) -> Self {
        match value {
            4 => DeviceType::T4,
            7 => DeviceType::T7,
            8 => DeviceType::T8,
            200 => DeviceType::DIGIT,
            value => DeviceType::UNKNOWN(value)
        }
    }
}

#[derive(Clone, Debug)]
pub enum ConnectionType {
    USB,
    ETHERNET,
    WIFI,
    UNKNOWN(i32)
}

impl From<i32> for ConnectionType {
    fn from(value: i32) -> Self {
        match value {
            1 => ConnectionType::USB,
            3 => ConnectionType::ETHERNET,
            4 => ConnectionType::WIFI,
            value => ConnectionType::UNKNOWN(value)
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

        write!(f,
            "{}", format!(
                "{:?} on {:?} @ {}B/MB\n{}:{}\n{}",
                self.device_type,
                self.connection_type,
                self.max_bytes_per_megabyte,
                self.ip_address,
                self.port,
                self.serial_number
            )
        )
    }
}