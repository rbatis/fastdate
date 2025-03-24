#![allow(unused_assignments)]

pub extern crate time1;

pub mod error;
pub mod sys;

mod date;
mod datetime;
mod time;

pub use date::*;
pub use datetime::*;
use std::time::Duration;
pub use time::*;

// get a character from the bytes as as a decimal
macro_rules! get_digit {
    ($bytes:ident, $index:expr, $error:expr) => {
        match $bytes.get($index) {
            Some(c) if c.is_ascii_digit() => c - b'0',
            _ => return Err(Error::E($error.to_string())),
        }
    };
}
pub(crate) use get_digit;
// as above without bounds check, requires length to checked first!
macro_rules! get_digit_unchecked {
    ($bytes:ident, $index:expr, $error:expr) => {
        match $bytes.get_unchecked($index) {
            c if c.is_ascii_digit() => c - b'0',
            _ => return Err(Error::E($error.to_string())),
        }
    };
}
pub(crate) use get_digit_unchecked;

pub trait DurationFrom {
    fn from_minute(minute: u64) -> Self;
    fn from_hour(hour: u64) -> Self;
    fn from_day(day: u64) -> Self;
}

impl DurationFrom for Duration {
    #[inline]
    fn from_minute(minute: u64) -> Self {
        Duration::from_secs(minute * 60)
    }
    #[inline]
    fn from_hour(hour: u64) -> Self {
        Duration::from_minute(hour * 60)
    }
    #[inline]
    fn from_day(day: u64) -> Self {
        Duration::from_hour(day * 24)
    }
}
