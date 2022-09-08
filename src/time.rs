use crate::error::Error;
use crate::{get_digit, get_digit_unchecked, DateTime};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use std::time::Duration;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Time {
    /// 0...999999
    pub micro: u32,
    /// 0...59
    pub sec: u8,
    /// 0...59
    pub min: u8,
    /// 0...23
    pub hour: u8,
}

impl Time {
    /// Parse a time from bytes with a starting index, no check is performed for extract characters at
    /// the end of the string
    pub(crate) fn parse_bytes_partial(bytes: &[u8], offset: usize) -> Result<(Self, usize), Error> {
        if bytes.len() < offset {
            return Ok((
                Self {
                    micro: 0,
                    sec: 0,
                    min: 0,
                    hour: 0,
                },
                0,
            ));
        }
        if bytes.len() - offset < 5 {
            return Err(Error::E("TooShort".to_string()));
        }
        let hour: u8;
        let minute: u8;
        unsafe {
            let h1 = get_digit_unchecked!(bytes, offset, "InvalidCharHour");
            let h2 = get_digit_unchecked!(bytes, offset + 1, "InvalidCharHour");
            hour = h1 * 10 + h2;

            match bytes.get_unchecked(offset + 2) {
                b':' => (),
                _ => (), //return Err(Error::E("InvalidCharTimeSep".to_string())),
            }
            let m1 = get_digit_unchecked!(bytes, offset + 3, "InvalidCharMinute");
            let m2 = get_digit_unchecked!(bytes, offset + 4, "InvalidCharMinute");
            minute = m1 * 10 + m2;
        }

        if hour > 23 {
            return Err(Error::E("OutOfRangeHour".to_string()));
        }

        if minute > 59 {
            return Err(Error::E("OutOfRangeMinute".to_string()));
        }
        let mut length: usize = 5;
        let (second, microsecond) = {
            let s1 = get_digit!(bytes, offset + 6, "InvalidCharSecond");
            let s2 = get_digit!(bytes, offset + 7, "InvalidCharSecond");
            let second = s1 * 10 + s2;
            if second > 59 {
                return Err(Error::E("OutOfRangeSecond".to_string()));
            }
            length = 8;
            let mut microsecond = 0;
            let frac_sep = bytes.get(offset + 8).copied();
            let mut number_buf = *b"         ";
            if frac_sep == Some(b'.') || frac_sep == Some(b',') {
                length = 9;
                let mut i: usize = 0;
                loop {
                    match bytes.get(offset + length + i) {
                        Some(c) if (b'0'..=b'9').contains(c) => {
                            if i >= 9 {
                                return Err(Error::E("SecondFractionTooLong".to_string()));
                            }
                            number_buf[i] = *c;
                        }
                        _ => {
                            break;
                        }
                    }
                    i += 1;
                    if i > 9 {
                        return Err(Error::E("SecondFractionTooLong".to_string()));
                    }
                }
                if i == 0 {
                    return Err(Error::E("SecondFractionMissing".to_string()));
                }
                length += i;
            }
            let mut i = 0;
            for idx in 0..number_buf.len() {
                let item = number_buf[number_buf.len() - 1 - idx];
                if item != ' ' as u8 {
                    //is number
                    let v = (item - '0' as u8) as u32 * 10_u32.pow(i as u32);
                    microsecond += v;
                    i += 1;
                }
            }
            if microsecond > 999999 {
                microsecond = microsecond / 1000;
            }
            (second, microsecond)
        };
        let t = Self {
            micro: microsecond,
            sec: second,
            min: minute,
            hour,
        };
        Ok((t, length))
    }

    /// 0...999999
    pub fn set_micro(mut self, arg: u32) -> Self {
        self.micro = arg;
        self
    }
    /// 0...59
    pub fn set_sec(mut self, arg: u8) -> Self {
        self.sec = arg;
        self
    }
    /// 0...59
    pub fn set_min(mut self, arg: u8) -> Self {
        self.min = arg;
        self
    }
    /// 0...23
    pub fn set_hour(mut self, arg: u8) -> Self {
        self.hour = arg;
        self
    }
}

impl From<Duration> for Time {
    fn from(d: Duration) -> Self {
        let hour = (d.as_secs() / 3600) as u8;
        let min = (d.as_secs() / 60 - (hour as u64 * 60)) as u8;
        let sec = (d.as_secs() - hour as u64 * 3600u64 - min as u64 * 60u64) as u8;
        let micros = d.as_micros()
            - (hour as u128 * 3600000000) as u128
            - (min as u128 * 60000000) as u128
            - (sec as u128 * 1000000) as u128;
        Self {
            micro: micros as u32,
            sec: sec,
            min: min,
            hour: hour,
        }
    }
}

impl From<Time> for Duration {
    fn from(d: Time) -> Self {
        Duration::from_secs(d.hour as u64 * 60 * 60)
            + Duration::from_secs(d.min as u64 * 60)
            + Duration::from_secs(d.sec as u64)
            + Duration::from_micros(d.micro as u64)
    }
}

impl FromStr for Time {
    type Err = Error;

    /// from RFC3339Micro = "15:04:05.999999"
    fn from_str(s: &str) -> Result<Time, Error> {
        //"00:00:00.000000";
        let (t, _) = Time::parse_bytes_partial(s.as_bytes(), 0)?;
        Ok(t)
    }
}

impl Display for Time {
    /// fmt RFC3339Micro = "2006-01-02T15:04:05.999999"
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let mut buf: [u8; 15] = *b"00:00:00.000000";

        buf[0] = b'0' + (self.hour / 10) as u8;
        buf[1] = b'0' + (self.hour % 10) as u8;
        buf[3] = b'0' + (self.min / 10) as u8;
        buf[4] = b'0' + (self.min % 10) as u8;
        buf[6] = b'0' + (self.sec / 10) as u8;
        buf[7] = b'0' + (self.sec % 10) as u8;

        buf[9] = b'0' + (self.micro / 100000 % 10) as u8;
        buf[10] = b'0' + (self.micro / 10000 % 10) as u8;
        buf[11] = b'0' + (self.micro / 1000 % 10) as u8;
        buf[12] = b'0' + (self.micro / 100 % 10) as u8;
        buf[13] = b'0' + (self.micro / 10 % 10) as u8;
        buf[14] = b'0' + (self.micro % 10) as u8;

        f.write_str(std::str::from_utf8(&buf[..]).unwrap())
    }
}

impl Serialize for Time {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Time {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        Time::from_str(&String::deserialize(deserializer)?)
            .map_err(|e| Error::custom(e.to_string()))
    }
}

impl From<DateTime> for Time {
    fn from(arg: DateTime) -> Self {
        Time {
            micro: arg.micro,
            sec: arg.sec,
            min: arg.min,
            hour: arg.hour,
        }
    }
}
