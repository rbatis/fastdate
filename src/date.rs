use crate::{get_digit_unchecked, DateTime};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

use crate::error::Error;

/// Log timestamp type.
///
/// Parse using `FromStr` impl.
/// Format using the `Display` trait.
/// Convert timestamp into/from `SytemTime` to use.
/// Supports comparsion and sorting.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Date {
    /// 1...31
    pub day: u8,
    /// 1...12
    pub mon: u8,
    /// 1970...9999
    pub year: u16,
}

impl Date {
    /// Parse a date from bytes, no check is performed for extract characters at the end of the string
    pub(crate) fn parse_bytes_partial(bytes: &[u8]) -> Result<Self, Error> {
        if bytes.len() < 10 {
            return Err(Error::E("TooShort".to_string()));
        }
        let year: u16;
        let month: u8;
        let day: u8;
        unsafe {
            let y1 = get_digit_unchecked!(bytes, 0, "InvalidCharYear") as u16;
            let y2 = get_digit_unchecked!(bytes, 1, "InvalidCharYear") as u16;
            let y3 = get_digit_unchecked!(bytes, 2, "InvalidCharYear") as u16;
            let y4 = get_digit_unchecked!(bytes, 3, "InvalidCharYear") as u16;
            year = y1 * 1000 + y2 * 100 + y3 * 10 + y4;

            match bytes.get_unchecked(4) {
                b'-' => (),
                _ => (), //return Err(Error::E("InvalidCharDateSep".to_string())),
            }

            let m1 = get_digit_unchecked!(bytes, 5, "InvalidCharMonth");
            let m2 = get_digit_unchecked!(bytes, 6, "InvalidCharMonth");
            month = m1 * 10 + m2;

            match bytes.get_unchecked(7) {
                b'-' => (),
                _ => (), //return Err(Error::E("InvalidCharDateSep".to_string())),
            }

            let d1 = get_digit_unchecked!(bytes, 8, "InvalidCharDay");
            let d2 = get_digit_unchecked!(bytes, 9, "InvalidCharDay");
            day = d1 * 10 + d2;
        }

        // calculate the maximum number of days in the month, accounting for leap years in the
        // gregorian calendar
        let max_days = match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {
                    29
                } else {
                    28
                }
            }
            _ => return Err(Error::E("OutOfRangeMonth".to_string())),
        };

        if day < 1 || day > max_days {
            return Err(Error::E("OutOfRangeDay".to_string()));
        }

        Ok(Self {
            day,
            mon: month,
            year,
        })
    }

    /// 1...31
    pub fn set_day(mut self, arg: u8) -> Self {
        self.day = arg;
        self
    }
    /// 1...12
    pub fn set_mon(mut self, arg: u8) -> Self {
        self.mon = arg;
        self
    }
    /// 1970...9999
    pub fn set_year(mut self, arg: u16) -> Self {
        self.year = arg;
        self
    }

    /// get day
    pub fn get_day(&self) -> u8 {
        self.day
    }

    /// get mon
    pub fn get_mon(&self) -> u8 {
        self.mon
    }

    /// get year
    pub fn get_year(&self) -> u16 {
        self.year
    }

    /// display date and return len
    pub fn display_date(&self, start: usize, buf: &mut [u8]) -> usize {
        buf[start + 0] = b'0' + (self.year / 1000) as u8;
        buf[start + 1] = b'0' + (self.year / 100 % 10) as u8;
        buf[start + 2] = b'0' + (self.year / 10 % 10) as u8;
        buf[start + 3] = b'0' + (self.year % 10) as u8;

        buf[start + 5] = b'0' + (self.mon / 10) as u8;
        buf[start + 6] = b'0' + (self.mon % 10) as u8;

        buf[start + 8] = b'0' + (self.day / 10) as u8;
        buf[start + 9] = b'0' + (self.day % 10) as u8;

        start + 10
    }
}

impl FromStr for Date {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //"0000-00-00";
        let d = Date::parse_bytes_partial(s.as_bytes())?;
        Ok(d)
    }
}

impl Display for Date {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut buf: [u8; 10] = *b"0000-00-00";
        self.display_date(0, &mut buf);
        f.write_str(std::str::from_utf8(&buf[..]).unwrap())
    }
}

impl Serialize for Date {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Date {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        use serde::de::Error;
        Date::from_str(&String::deserialize(deserializer)?)
            .map_err(|e| D::Error::custom(e.to_string()))
    }
}

impl From<DateTime> for Date {
    fn from(arg: DateTime) -> Self {
        Date {
            day: arg.day,
            mon: arg.mon,
            year: arg.year,
        }
    }
}
