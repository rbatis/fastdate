use std::fmt::{Display, Formatter, Write};
use std::str::FromStr;
use crate::DateTime;
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

    /// from RFC3339Micro = "2006-01-02T15:04:05.999999"
    fn from_str(s: &str) -> Result<Time, Error> {
        //"00:00:00.000000";
        let mut date = Time {
            micro: 0,
            sec: 0,
            min: 0,
            hour: 0,
        };
        let bytes = s.as_bytes();
        if bytes.len() > 8 {
            if let Ok(hour) = std::str::from_utf8(&bytes[0..2])
                .unwrap_or_default()
                .parse::<u8>()
            {
                date.hour = hour;
            }
            if let Ok(min) = std::str::from_utf8(&bytes[3..5])
                .unwrap_or_default()
                .parse::<u8>()
            {
                date.min = min;
            }
            if let Ok(sec) = std::str::from_utf8(&bytes[6..8])
                .unwrap_or_default()
                .parse::<u8>()
            {
                date.sec = sec;
            }
            if bytes.len() > 9 {
                if let Ok(ns) = std::str::from_utf8(&bytes[9..bytes.len()])
                    .unwrap_or_default()
                    .parse::<u32>()
                {
                    date.micro = ns;
                }
            }
        }
        Ok(date)
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