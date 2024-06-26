pub type Result<T> = core::result::Result<T, SHTError>;
#[cfg_attr(feature = "thiserror", derive(thiserror::Error))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "postcard", derive(postcard::experimental::max_size::MaxSize))]
#[derive(Debug, Copy, Clone, defmt::Format, Ord, PartialOrd, Eq, PartialEq)]
pub enum SHTError {
    #[cfg_attr(feature = "thiserror", error("Read I2C Error"))]
    ReadI2CError,
    #[cfg_attr(feature = "thiserror", error("Write Read I2C Error"))]
    WriteReadI2CError,
    #[cfg_attr(feature = "thiserror", error("Write I2C Error"))]
    WriteI2CError,
    #[cfg_attr(feature = "thiserror", error("Humidity bytes [{bytes_start:#x}, {bytes_end:#x}] expected {expected_checksum:#x} but got the checksum {calculated_checksum:#x}"))]
    InvalidHumidityChecksumError {
        bytes_start: u8,
        bytes_end: u8,
        expected_checksum: u8,
        calculated_checksum: u8,
    },
    #[cfg_attr(feature = "thiserror", error("Temperature bytes [{bytes_start:#x}, {bytes_end:#x}] expected {expected_checksum:#x} but got the checksum {calculated_checksum:#x}"))]
    InvalidTemperatureChecksumError {
        bytes_start: u8,
        bytes_end: u8,
        expected_checksum: u8,
        calculated_checksum: u8,
    },
    #[cfg_attr(feature = "thiserror", error("Status bytes [{bytes_start:#x}, {bytes_end:#x}] expected {expected_checksum:#x} but got the checksum {calculated_checksum:#x}"))]
    InvalidStatusChecksumError {
        bytes_start: u8,
        bytes_end: u8,
        expected_checksum: u8,
        calculated_checksum: u8,
    },
    #[cfg_attr(feature = "thiserror", error("Single shot reading timeout"))]
    ReadingTimeoutError,
    #[cfg_attr(feature = "thiserror", error("This error should not happen"))]
    PlaceholderError,
}
