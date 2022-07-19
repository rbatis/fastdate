#![feature(mem_copy_fn)]

pub mod sys;
pub mod error;

mod date;
mod time;
mod datetime;
pub use date::*;
pub use time::*;
pub use datetime::*;


// get a character from the bytes as as a decimal
macro_rules! get_digit {
    ($bytes:ident, $index:expr, $error:expr) => {
        match $bytes.get($index) {
            Some(c) if (b'0'..=b'9').contains(&c) => c - b'0',
            _ => return Err(Error::E($error.to_string())),
        }
    };
}
pub(crate) use get_digit;
// as above without bounds check, requires length to checked first!
macro_rules! get_digit_unchecked {
    ($bytes:ident, $index:expr, $error:expr) => {
        match $bytes.get_unchecked($index) {
            c if (b'0'..=b'9').contains(&c) => c - b'0',
            _ => return Err(Error::E($error.to_string())),
        }
    };
}
pub(crate) use get_digit_unchecked;