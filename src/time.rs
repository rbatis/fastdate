use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::{DateTime, get_digit, get_digit_unchecked};
use crate::error::Error;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
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

impl Time{
    /// Parse a time from bytes with a starting index, no check is performed for extract characters at
    /// the end of the string
    pub(crate) fn parse_bytes_partial(bytes: &[u8], offset: usize) -> Result<(Self, usize), Error> {
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
                _ => return Err(Error::E("InvalidCharTimeSep".to_string())),
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

        let (second, microsecond) = match bytes.get(offset + 5) {
            Some(b':') => {
                let s1 = get_digit!(bytes, offset + 6, "InvalidCharSecond");
                let s2 = get_digit!(bytes, offset + 7, "InvalidCharSecond");
                let second = s1 * 10 + s2;
                if second > 59 {
                    return Err(Error::E("OutOfRangeSecond".to_string()));
                }
                length = 8;

                let mut microsecond = 0;
                let frac_sep = bytes.get(offset + 8).copied();
                if frac_sep == Some(b'.') || frac_sep == Some(b',') {
                    length = 9;
                    let mut i: usize = 0;
                    loop {
                        match bytes.get(offset + length + i) {
                            Some(c) if (b'0'..=b'9').contains(c) => {
                                microsecond *= 10;
                                microsecond += (c - b'0') as u32;
                            }
                            _ => {
                                break;
                            }
                        }
                        i += 1;
                        if i > 6 {
                            return Err(Error::E("SecondFractionTooLong".to_string()));
                        }
                    }
                    if i == 0 {
                        return Err(Error::E("SecondFractionMissing".to_string()));
                    }
                    if i < 6 {
                        microsecond *= 10_u32.pow(6 - i as u32);
                    }
                    length += i;
                }
                (second, microsecond)
            }
            _ => (0, 0),
        };
        let t = Self {
            micro: microsecond,
            sec: second,
            min: minute,
            hour
        };
        Ok((t, length))
    }
}

impl From<DateTime> for Time{
    fn from(arg: DateTime) -> Self {
        Time{
            micro: arg.micro,
            sec: arg.sec,
            min: arg.min,
            hour: arg.hour
        }
    }
}


impl FromStr for Time {
    type Err = Error;

    /// from RFC3339Micro = "15:04:05.999999"
    fn from_str(s: &str) -> Result<Time, Error> {
        //"00:00:00.000000";
        let (t,_)=Time::parse_bytes_partial(s.as_bytes(),0)?;
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

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use crate::Time;

    #[test]
    fn test_date() {
        let d = Time::from_str("11:12:13.123456").unwrap();
        println!("{}", d);
        assert_eq!("11:12:13.123456".to_string(), d.to_string());

        let d = Time::from_str("11:12:13.12345").unwrap();
        println!("{}", d);
        assert_eq!("11:12:13.012345".to_string(), d.to_string());

        let d = Time::from_str("11:12:13.1234").unwrap();
        println!("{}", d);
        assert_eq!("11:12:13.001234".to_string(), d.to_string());
    }


}