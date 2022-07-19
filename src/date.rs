use std::cmp;
use std::fmt::{self, Display, Formatter, Pointer};
use std::str::FromStr;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use crate::DateTime;

use crate::error::Error as Error;

/// Log timestamp type.
///
/// Parse using `FromStr` impl.
/// Format using the `Display` trait.
/// Convert timestamp into/from `SytemTime` to use.
/// Supports comparsion and sorting.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Date {
    /// 1...31
    pub day: u8,
    /// 1...12
    pub mon: u8,
    /// 1970...9999
    pub year: u16,
}

impl From<DateTime> for Date{
    fn from(arg: DateTime) -> Self {
        Date{
            day: arg.day,
            mon: arg.mon,
            year: arg.year
        }
    }
}

impl FromStr for Date {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //"0000-00-00 00:00:00.000000";
        let mut date = Date {
            day: 0,
            mon: 0,
            year: 0,
        };
        let bytes = s.as_bytes();
        if bytes.len() >= 10 {
            if let Ok(year) = std::str::from_utf8(&bytes[0..4])
                .unwrap_or_default()
                .parse::<u16>()
            {
                date.year = year;
            }
            if let Ok(mon) = std::str::from_utf8(&bytes[5..7])
                .unwrap_or_default()
                .parse::<u8>()
            {
                date.mon = mon;
            }
            if let Ok(day) = std::str::from_utf8(&bytes[8..10])
                .unwrap_or_default()
                .parse::<u8>()
            {
                date.day = day;
            }
        }
        Ok(date)
    }
}

impl Display for Date{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut buf: [u8; 10] = *b"0000-00-00";

        buf[0] = b'0' + (self.year / 1000) as u8;
        buf[1] = b'0' + (self.year / 100 % 10) as u8;
        buf[2] = b'0' + (self.year / 10 % 10) as u8;
        buf[3] = b'0' + (self.year % 10) as u8;

        buf[5] = b'0' + (self.mon / 10) as u8;
        buf[6] = b'0' + (self.mon % 10) as u8;

        buf[8] = b'0' + (self.day / 10) as u8;
        buf[9] = b'0' + (self.day % 10) as u8;
        f.write_str(std::str::from_utf8(&buf[..]).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use crate::Date;

    #[test]
    fn test_date() {
        let d = Date::from_str("1234-12-13 11:12:13.123456").unwrap();
        println!("{}", d);
        assert_eq!("1234-12-13".to_string(), d.to_string());
    }
}
