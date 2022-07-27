use std::cmp;
use std::fmt::{self, Display, Formatter};
use std::ops::{Add, Deref, Sub};
use std::str::FromStr;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use once_cell::sync::Lazy;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use crate::{Date, Time};
use crate::error::Error as Error;
use crate::sys::Timespec;

/// Obtain the offset of Utc time and Local time in seconds, using Lazy only once to improve performance
pub static GLOBAL_OFFSET: Lazy<i32> = Lazy::new(|| Timespec::now().local().tm_utcoff);

/// offset with utc 0.zone
pub fn offset_sec() -> i32 {
    GLOBAL_OFFSET.deref().clone()
}

/// Log timestamp type.
///
/// Parse using `FromStr` impl.
/// Format using the `Display` trait.
/// Convert timestamp into/from `SytemTime` to use.
/// Supports comparsion and sorting.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct DateTime {
    /// 0...999999
    pub micro: u32,
    /// 0...59
    pub sec: u8,
    /// 0...59
    pub min: u8,
    /// 0...23
    pub hour: u8,
    /// 1...31
    pub day: u8,
    /// 1...12
    pub mon: u8,
    /// 1970...9999
    pub year: u16,
}

impl DateTime {
    ///utc time
    pub fn utc() -> Self {
        Self::from(SystemTime::now())
    }
    ///local zone time
    pub fn now() -> Self {
        let offset = GLOBAL_OFFSET.deref().clone();
        if offset > 0 {
            Self::from(SystemTime::now() + Duration::from_secs(offset as u64))
        } else {
            Self::from(SystemTime::now() - Duration::from_secs(offset as u64))
        }
    }

    /// set offset
    /// ```rust
    /// let mut  dt = fastdate::DateTime::utc();
    /// dt = dt.set_offset(fastdate::offset_sec());
    /// ```
    pub fn set_offset(self, offset_sec: i32) -> DateTime {
        let time: SystemTime = self.into();
        if offset_sec > 0 {
            Self::from(time + Duration::from_secs(offset_sec as u64))
        } else {
            Self::from(time - Duration::from_secs(offset_sec as u64))
        }
    }

    pub fn add(self, d: Duration) -> Self {
        let systime = SystemTime::from(self) + d;
        Self::from(systime)
    }

    pub fn sub(self, d: Duration) -> Self {
        let systime = SystemTime::from(self) - d;
        Self::from(systime)
    }

    /// is self before on other?
    pub fn before(&self, other: &DateTime) -> bool {
        self < other
    }

    /// is self after on other?
    pub fn after(&self, other: &DateTime) -> bool {
        self > other
    }

    /// unix_timestamp sec
    pub fn unix_timestamp(self) -> i64 {
        let s = SystemTime::from(self)
            .duration_since(UNIX_EPOCH)
            .expect("all times should be after the epoch");
        return s.as_secs() as i64;
    }

    ///unix_timestamp millis
    pub fn unix_timestamp_millis(self) -> i64 {
        let s = SystemTime::from(self)
            .duration_since(UNIX_EPOCH)
            .expect("all times should be after the epoch");
        return s.as_millis() as i64;
    }

    ///unix_timestamp nano
    pub fn unix_timestamp_nano(self) -> u128 {
        let s = SystemTime::from(self)
            .duration_since(UNIX_EPOCH)
            .expect("all times should be after the epoch");
        return s.as_nanos();
    }

    ///from timestamp sec
    pub fn from_timestamp(sec: i64) -> DateTime {
        let v = UNIX_EPOCH + Duration::from_secs(sec as u64);
        Self::from(v)
    }
    ///from timestamp sec
    pub fn from_timestamp_millis(ms: i64) -> DateTime {
        let v = UNIX_EPOCH + Duration::from_millis(ms as u64);
        Self::from(v)
    }
    ///from timestamp sec
    pub fn from_timestamp_nano(nano: u128) -> DateTime {
        let v = UNIX_EPOCH + Duration::from_nanos(nano as u64);
        Self::from(v)
    }
}

impl Add<Duration> for DateTime {
    type Output = DateTime;

    fn add(self, rhs: Duration) -> Self::Output {
        self.add(rhs)
    }
}

impl Sub<Duration> for DateTime {
    type Output = DateTime;

    fn sub(self, rhs: Duration) -> Self::Output {
        self.sub(rhs)
    }
}

impl Add<&Duration> for DateTime {
    type Output = DateTime;

    fn add(self, rhs: &Duration) -> Self::Output {
        self.add(rhs.clone())
    }
}

impl Sub<&Duration> for DateTime {
    type Output = DateTime;

    fn sub(self, rhs: &Duration) -> Self::Output {
        self.sub(rhs.clone())
    }
}

impl From<SystemTime> for DateTime {
    fn from(v: SystemTime) -> DateTime {
        let dur = v
            .duration_since(UNIX_EPOCH)
            .expect("all times should be after the epoch");
        let secs_since_epoch = dur.as_secs();

        if secs_since_epoch >= 253402300800 {
            // year 9999
            panic!("date must be before year 9999");
        }

        /* 2000-03-01 (mod 400 year, immediately after feb29 */
        const LEAPOCH: i64 = 11017;
        const DAYS_PER_400Y: i64 = 365 * 400 + 97;
        const DAYS_PER_100Y: i64 = 365 * 100 + 24;
        const DAYS_PER_4Y: i64 = 365 * 4 + 1;

        let days = (secs_since_epoch / 86400) as i64 - LEAPOCH;
        let secs_of_day = secs_since_epoch % 86400;

        let mut qc_cycles = days / DAYS_PER_400Y;
        let mut remdays = days % DAYS_PER_400Y;

        if remdays < 0 {
            remdays += DAYS_PER_400Y;
            qc_cycles -= 1;
        }

        let mut c_cycles = remdays / DAYS_PER_100Y;
        if c_cycles == 4 {
            c_cycles -= 1;
        }
        remdays -= c_cycles * DAYS_PER_100Y;

        let mut q_cycles = remdays / DAYS_PER_4Y;
        if q_cycles == 25 {
            q_cycles -= 1;
        }
        remdays -= q_cycles * DAYS_PER_4Y;

        let mut remyears = remdays / 365;
        if remyears == 4 {
            remyears -= 1;
        }
        remdays -= remyears * 365;

        let mut year = 2000 + remyears + 4 * q_cycles + 100 * c_cycles + 400 * qc_cycles;

        let months = [31, 30, 31, 30, 31, 31, 30, 31, 30, 31, 31, 29];
        let mut mon = 0;
        for mon_len in months.iter() {
            mon += 1;
            if remdays < *mon_len {
                break;
            }
            remdays -= *mon_len;
        }
        let mday = remdays + 1;
        let mon = if mon + 2 > 12 {
            year += 1;
            mon - 10
        } else {
            mon + 2
        };

        DateTime {
            micro: (dur - Duration::from_secs(dur.as_secs())).as_micros() as u32,
            sec: (secs_of_day % 60) as u8,
            min: ((secs_of_day % 3600) / 60) as u8,
            hour: (secs_of_day / 3600) as u8,
            day: mday as u8,
            mon: mon as u8,
            year: year as u16,
        }
    }
}

impl From<DateTime> for SystemTime {
    fn from(v: DateTime) -> SystemTime {
        let leap_years =
            ((v.year - 1) - 1968) / 4 - ((v.year - 1) - 1900) / 100 + ((v.year - 1) - 1600) / 400;
        let mut ydays = match v.mon {
            1 => 0,
            2 => 31,
            3 => 59,
            4 => 90,
            5 => 120,
            6 => 151,
            7 => 181,
            8 => 212,
            9 => 243,
            10 => 273,
            11 => 304,
            12 => 334,
            _ => unreachable!(),
        } + v.day as u64
            - 1;
        if is_leap_year(v.year) && v.mon > 2 {
            ydays += 1;
        }
        let days = (v.year as u64 - 1970) * 365 + leap_years as u64 + ydays;
        let sec = Duration::from_secs(
            v.sec as u64 + v.min as u64 * 60 + v.hour as u64 * 3600 + days * 86400,
        );
        if v.micro > 0 {
            UNIX_EPOCH
                + sec + Duration::from_micros(v.micro as u64)
        } else {
            UNIX_EPOCH
                + sec - Duration::from_micros(v.micro as u64)
        }
    }
}

impl FromStr for DateTime {
    type Err = Error;

    /// from RFC3339Nano = "0000-00-00 00:00:00.000000"
    fn from_str(s: &str) -> Result<DateTime, Error> {
        let bytes = s.as_bytes();
        let mut date = DateTime {
            micro: 0,
            sec: 0,
            min: 0,
            hour: 0,
            day: 0,
            mon: 0,
            year: 0,
        };
        if bytes.len() >= 10 {
            let d = Date::parse_bytes_partial(&bytes)?;
            date.year = d.year;
            date.mon = d.mon;
            date.day = d.day;

            let (t, offset) = Time::parse_bytes_partial(&bytes, 11)?;
            date.hour = t.hour;
            date.min = t.min;
            date.sec = t.sec;
            date.micro = t.micro;
            let start = 11 + offset;
            //+09:00
            let mut offset_sec = 0;
            if s.len() > start {
                let remin_str = &s[start..];
                let remin_bytes = remin_str.as_bytes();
                if remin_str.len() == 3 {
                    if remin_bytes[0] == b'+' {
                        offset_sec += ((remin_bytes[1] - b'0') as i32 * 10 + (remin_bytes[2] - b'0') as i32) * 3600;
                    } else if remin_bytes[0] == b'-' {
                        offset_sec -= ((remin_bytes[1] - b'0') as i32 * 10 + (remin_bytes[2] - b'0') as i32) * 3600;
                    }
                } else if remin_str.len() == 6 {
                    if remin_bytes[0] == b'+' {
                        //hour
                        offset_sec += ((remin_bytes[1] - b'0') as i32 * 10 + (remin_bytes[2] - b'0') as i32) * 3600;
                        //min
                        offset_sec += ((remin_bytes[4] - b'0') as i32 * 10 + (remin_bytes[5] - b'0') as i32) * 60;
                    } else if remin_bytes[0] == b'-' {
                        //hour
                        offset_sec -= ((remin_bytes[1] - b'0') as i32 * 10 + (remin_bytes[2] - b'0') as i32) * 3600;
                        //min
                        offset_sec -= ((remin_bytes[4] - b'0') as i32 * 10 + (remin_bytes[5] - b'0') as i32) * 60;
                    }
                }
            }
            if offset_sec > 0 {
                date = date.add(Duration::from_secs(offset_sec as u64));
            } else if offset_sec < 0 {
                date = date.sub(Duration::from_secs(offset_sec.abs() as u64));
            }
            if bytes[bytes.len() - 1] == 'Z' as u8 || bytes[bytes.len() - 1] == 'z' as u8 {
                date.set_offset(crate::offset_sec());//append offset
            }
        }
        Ok(date)
    }
}

impl Display for DateTime {
    /// fmt RFC3339Micro = "2006-01-02T15:04:05.999999"
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut buf: [u8; 26] = *b"0000-00-00 00:00:00.000000";

        buf[0] = b'0' + (self.year / 1000) as u8;
        buf[1] = b'0' + (self.year / 100 % 10) as u8;
        buf[2] = b'0' + (self.year / 10 % 10) as u8;
        buf[3] = b'0' + (self.year % 10) as u8;

        buf[5] = b'0' + (self.mon / 10) as u8;
        buf[6] = b'0' + (self.mon % 10) as u8;

        buf[8] = b'0' + (self.day / 10) as u8;
        buf[9] = b'0' + (self.day % 10) as u8;

        buf[11] = b'0' + (self.hour / 10) as u8;
        buf[12] = b'0' + (self.hour % 10) as u8;
        buf[14] = b'0' + (self.min / 10) as u8;
        buf[15] = b'0' + (self.min % 10) as u8;
        buf[17] = b'0' + (self.sec / 10) as u8;
        buf[18] = b'0' + (self.sec % 10) as u8;

        buf[20] = b'0' + (self.micro / 100000 % 10) as u8;
        buf[21] = b'0' + (self.micro / 10000 % 10) as u8;
        buf[22] = b'0' + (self.micro / 1000 % 10) as u8;
        buf[23] = b'0' + (self.micro / 100 % 10) as u8;
        buf[24] = b'0' + (self.micro / 10 % 10) as u8;
        buf[25] = b'0' + (self.micro % 10) as u8;

        f.write_str(std::str::from_utf8(&buf[..]).unwrap())
    }
}

impl Ord for DateTime {
    fn cmp(&self, other: &DateTime) -> cmp::Ordering {
        SystemTime::from(*self).cmp(&SystemTime::from(*other))
    }
}

impl PartialOrd for DateTime {
    fn partial_cmp(&self, other: &DateTime) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn is_leap_year(y: u16) -> bool {
    y % 4 == 0 && (y % 100 != 0 || y % 400 == 0)
}


impl Serialize for DateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for DateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        use serde::de::Error;
        let s = String::deserialize(deserializer)?;
        DateTime::from_str(&s).map_err(|e| D::Error::custom(e))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use std::time::Duration;
    use crate::{Date, DateTime, Time};

    #[test]
    fn test_other_space() {
        let d = DateTime::from_str("1234_12_13_11_12_13.123456").unwrap();
        println!("{}", d);
        assert_eq!("1234-12-13 11:12:13.123456".to_string(), d.to_string());
    }

    #[test]
    fn test_date() {
        let d = DateTime::from_str("1234-12-13 11:12:13.123456").unwrap();
        println!("{}", d);
        assert_eq!("1234-12-13 11:12:13.123456".to_string(), d.to_string());
    }

    #[test]
    fn test_date_utc() {
        let d = DateTime::now();
        println!("{}", d);
    }

    #[test]
    fn test_date_utc_add() {
        let d = DateTime::now();
        let added = d + Duration::from_secs(1);
        println!("{},{}", d, added);
        assert_eq!(d.add(Duration::from_secs(1)), added);
    }

    #[test]
    fn test_offset() {
        let utc = DateTime::from_str("2022-12-12 12:12:12.000000").unwrap();
        assert_eq!(format!("{}", utc.set_offset(1)), "2022-12-12 12:12:13.000000");
    }

    #[test]
    fn test_timestamp() {
        let mut now = DateTime::utc();
        now.micro = 0;
        let timestamp = now.unix_timestamp();
        let new_time = DateTime::from_timestamp(timestamp);
        assert_eq!(now, new_time);
    }

    #[test]
    fn test_timestamp_millis() {
        let mut now = DateTime::utc();
        now.micro = 0;
        let timestamp = now.unix_timestamp_millis();
        let new_time = DateTime::from_timestamp_millis(timestamp);
        assert_eq!(now, new_time);
    }

    #[test]
    fn test_timestamp_nano() {
        let now = DateTime::utc();
        let timestamp = now.unix_timestamp_nano();
        let new_time = DateTime::from_timestamp_nano(timestamp);
        assert_eq!(now, new_time);
    }

    #[test]
    fn test_unix_timestamp() {
        let d = DateTime::now().unix_timestamp();
        println!("unix:{}", d);
        let d = DateTime::utc().unix_timestamp();
        println!("unix:{}", d);

        let d = DateTime::now().unix_timestamp_millis();
        println!("unix ms:{}", d);
        let d = DateTime::utc().unix_timestamp_millis();
        println!("unix ms:{}", d);

        let d = DateTime::now().unix_timestamp_nano();
        println!("unix nano:{}", d);
        let d = DateTime::utc().unix_timestamp_nano();
        println!("unix nano:{}", d);
    }

    #[test]
    fn test_offset_zone() {
        let utc = DateTime::from_str("2022-12-12 00:00:00-08:00").unwrap();
        println!("{}", utc);
    }

    #[test]
    fn test_into() {
        let utc = DateTime::from_str("2022-12-12 00:00:00+08:00").unwrap();
        let date: Date = utc.into();
        let time: Time = utc.into();
        println!("{},{}", date, time);
        assert_eq!("2022-12-12", date.to_string());
        assert_eq!("08:00:00.000000", time.to_string());
    }

    #[test]
    fn test_befor_after() {
        let date1 = DateTime::from_str("2022-12-12 00:00:00").unwrap();
        let date2 = DateTime::from_str("2022-12-12 01:00:00").unwrap();
        assert_eq!(date2.after(&date1), true);
        assert_eq!(date1.before(&date2), true);
    }

    #[test]
    fn test_parse_z() {
        let date = DateTime::from_str("2022-12-12 00:00:00.000000Z").unwrap();
        let date_offset = date.clone().set_offset(crate::offset_sec());
        assert_eq!("2022-12-12 08:00:00.000000", date_offset.to_string());
    }
}
