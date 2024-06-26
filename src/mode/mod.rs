use crate::error::Result;
use crate::Reading;

mod periodic;
pub use periodic::{Periodic, MPS};
mod single_shot;
pub use single_shot::SingleShot;
mod simple_single_shot;
pub use simple_single_shot::SimpleSingleShot;

pub trait Sht31Reader {
    /// Read the sensor readings
    fn read(&mut self) -> impl core::future::Future<Output = Result<Reading>>;
}

pub trait Sht31Measure {
    /// Initialize reading
    fn measure(&mut self) -> impl core::future::Future<Output = Result<()>>;
}
