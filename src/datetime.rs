use crate::error::Error;
use crate::sys::Timespec;
use crate::{Date, DurationFrom, Time};
use once_cell::sync::Lazy;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::cmp;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Deref, Sub};
use std::str::FromStr;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

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
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct DateTime {
    /// 0...999999999
    pub nano: u32,
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
            Self::from(SystemTime::now() - Duration::from_secs(offset.abs() as u64))
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
            Self::from(time - Duration::from_secs(offset_sec.abs() as u64))
        }
    }

    /// add Duration
    pub fn add(self, d: Duration) -> Self {
        Self::from(SystemTime::from(self) + d)
    }

    /// sub Duration
    pub fn sub(self, d: Duration) -> Self {
        Self::from(SystemTime::from(self) - d)
    }

    ///add/sub sec
    pub fn add_sub_sec(self, sec: i64) -> Self {
        if sec > 0 {
            self.add(Duration::from_secs(sec as u64))
        } else {
            self.sub(Duration::from_secs((-sec) as u64))
        }
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
    pub fn unix_timestamp(&self) -> i64 {
        let s = SystemTime::from(self.clone())
            .duration_since(UNIX_EPOCH)
            .expect("all times should be after the epoch");
        return s.as_secs() as i64;
    }

    ///unix_timestamp micros
    pub fn unix_timestamp_micros(&self) -> i64 {
        let s = SystemTime::from(self.clone())
            .duration_since(UNIX_EPOCH)
            .expect("all times should be after the epoch");
        return s.as_micros() as i64;
    }

    ///unix_timestamp millis
    pub fn unix_timestamp_millis(&self) -> i64 {
        let s = SystemTime::from(self.clone())
            .duration_since(UNIX_EPOCH)
            .expect("all times should be after the epoch");
        return s.as_millis() as i64;
    }

    ///unix_timestamp nano
    pub fn unix_timestamp_nano(&self) -> u128 {
        let s = SystemTime::from(self.clone())
            .duration_since(UNIX_EPOCH)
            .expect("all times should be after the epoch");
        return s.as_nanos();
    }

    ///from timestamp sec
    pub fn from_timestamp(sec: i64) -> DateTime {
        if sec > 0 {
            Self::from(UNIX_EPOCH + Duration::from_secs(sec as u64))
        } else {
            Self::from(UNIX_EPOCH - Duration::from_secs(-sec as u64))
        }
    }
    ///from timestamp micros
    pub fn from_timestamp_micros(micros: i64) -> DateTime {
        let v = UNIX_EPOCH + Duration::from_micros(micros as u64);
        Self::from(v)
    }
    ///from timestamp millis
    pub fn from_timestamp_millis(ms: i64) -> DateTime {
        let v = UNIX_EPOCH + Duration::from_millis(ms as u64);
        Self::from(v)
    }
    ///from timestamp nano
    pub fn from_timestamp_nano(nano: u128) -> DateTime {
        let v = UNIX_EPOCH + Duration::from_nanos(nano as u64);
        Self::from(v)
    }

    /// parse an string by format.
    /// format str must be:
    /// ```rust
    ///  fastdate::DateTime::parse("YYYY-MM-DD hh:mm:ss.000000","2022-12-13 11:12:14.123456").unwrap();
    /// ```
    /// or any position
    /// ```rust
    ///  fastdate::DateTime::parse("hh:mm:ss.000000,YYYY-MM-DD","11:12:14.123456,2022-12-13").unwrap();
    /// ```
    pub fn parse(format: &str, arg: &str) -> Result<DateTime, Error> {
        let bytes = arg.as_bytes();
        let mut buf: [u8; 26] = *b"0000-00-00 00:00:00.000000";
        let format_bytes = format.as_bytes();
        let mut idx_year = 0;
        let mut idx_mon = 5;
        let mut idx_day = 8;
        let mut idx_hour = 11;
        let mut idx_min = 14;
        let mut idx_sec = 17;
        let mut idx_micro = 20;
        let mut v = 0;
        for x in format_bytes {
            if x == &('Y' as u8) {
                if v >= bytes.len() {
                    return Err(Error::from("wrong YYYY format!"));
                }
                buf[idx_year] = bytes[v];
                idx_year += 1;
            }
            if x == &('M' as u8) {
                if v >= bytes.len() {
                    return Err(Error::from("wrong MM format!"));
                }
                buf[idx_mon] = bytes[v];
                idx_mon += 1;
            }
            if x == &('D' as u8) {
                if v >= bytes.len() {
                    return Err(Error::from("wrong DD format!"));
                }
                buf[idx_day] = bytes[v];
                idx_day += 1;
            }
            if x == &('h' as u8) {
                if v >= bytes.len() {
                    return Err(Error::from("wrong hh format!"));
                }
                buf[idx_hour] = bytes[v];
                idx_hour += 1;
            }
            if x == &('m' as u8) {
                if v >= bytes.len() {
                    return Err(Error::from("wrong mm format!"));
                }
                buf[idx_min] = bytes[v];
                idx_min += 1;
            }
            if x == &('s' as u8) {
                if v >= bytes.len() {
                    return Err(Error::from("wrong ss format!"));
                }
                buf[idx_sec] = bytes[v];
                idx_sec += 1;
            }
            if x == &('0' as u8) {
                if v >= bytes.len() {
                    return Err(Error::from("wrong .000000 format!"));
                }
                buf[idx_micro] = bytes[v];
                idx_micro += 1;
            }
            v += 1;
        }
        DateTime::from_str(std::str::from_utf8(&buf[..]).unwrap_or_default())
    }

    /// get week_day
    pub fn week_day(&self) -> u8 {
        let secs_since_epoch = self.unix_timestamp();
        /* 2000-03-01 (mod 400 year, immediately after feb29 */
        const LEAPOCH: i64 = 11017;
        let days = (secs_since_epoch / 86400) as i64 - LEAPOCH;
        let mut wday = (3 + days) % 7;
        if wday <= 0 {
            wday += 7
        };
        wday as u8
    }
    /// 0...999999999
    pub fn set_nano(mut self, arg: u32) -> Self {
        self.nano = arg;
        self
    }
    /// get nano 0...999999999
    pub fn get_nano(&self) -> u32 {
        self.nano
    }
    /// 0...999999
    pub fn set_micro(mut self, arg: u32) -> Self {
        self.nano = arg * 1000;
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

    /// get micro secs
    pub fn get_micro(&self) -> u32 {
        self.nano / 1000
    }

    /// get sec
    pub fn get_sec(&self) -> u8 {
        self.sec
    }

    /// get sec
    pub fn get_min(&self) -> u8 {
        self.min
    }

    /// get hour
    pub fn get_hour(&self) -> u8 {
        self.hour
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

impl Sub<DateTime> for DateTime {
    type Output = Duration;

    fn sub(self, rhs: DateTime) -> Self::Output {
        let nano = self.unix_timestamp_nano() - rhs.unix_timestamp_nano();
        Duration::from_nanos(nano as u64)
    }
}


//每个四年的总天数
const FOUR_YEAR: u64 = 366 + 365 + 365 + 365;

fn find_mon_day(is_leap_year: bool, days: u64) -> (u64, u64) {
    let mons = {
        if is_leap_year {
            LeapYear
        } else {
            DefaultYear
        }
    };
    let mon11 = mons[0] + mons[1] + mons[2] + mons[3] + mons[4] + mons[5] + mons[6] + mons[7] + mons[8] + mons[9] + mons[10];
    let mon10 = mons[0] + mons[1] + mons[2] + mons[3] + mons[4] + mons[5] + mons[6] + mons[7] + mons[8] + mons[9];
    let mon9 = mons[0] + mons[1] + mons[2] + mons[3] + mons[4] + mons[5] + mons[6] + mons[7] + mons[8];
    let mon8 = mons[0] + mons[1] + mons[2] + mons[3] + mons[4] + mons[5] + mons[6] + mons[7];
    let mon7 = mons[0] + mons[1] + mons[2] + mons[3] + mons[4] + mons[5] + mons[6];
    let mon6 = mons[0] + mons[1] + mons[2] + mons[3] + mons[4] + mons[5];
    let mon5 = mons[0] + mons[1] + mons[2] + mons[3] + mons[4];
    let mon4 = mons[0] + mons[1] + mons[2] + mons[3];
    let mon3 = mons[0] + mons[1] + mons[2];
    let mon2 = mons[0] + mons[1];
    let mon1 = mons[0];
    if days > mon11 {
        let mon = 12 as u64;
        let day = days - mon11;
        return (mon, day);
    } else if days > mon10 {
        let mon = 11 as u64;
        let day = days - mon10;
        return (mon, day);
    } else if days > mon9 {
        let mon = 10 as u64;
        let day = days - mon9;
        return (mon, day);
    } else if days > mon8 {
        let mon = 9 as u64;
        let day = days - mon8;
        return (mon, day);
    } else if days > mon7 {
        let mon = 8 as u64;
        let day = days - mon7;
        return (mon, day);
    } else if days > mon6 {
        let mon = 7 as u64;
        let day = days - mon6;
        return (mon, day);
    } else if days > mon5 {
        let mon = 6 as u64;
        let day = days - mon5;
        return (mon, day);
    } else if days > mon4 {
        let mon = 5 as u64;
        let day = days - mon4;
        return (mon, day);
    } else if days > mon3 {
        let mon = 4 as u64;
        let day = days - mon3;
        return (mon, day);
    } else if days > mon2 {
        let mon = 3 as u64;
        let day = days - mon2;
        return (mon, day);
    } else if days > mon1 {
        let mon = 2 as u64;
        let day = days - mon1;
        return (mon, day);
    } else {
        let mon = 1 as u64;
        let day = days;
        return (mon, day);
    }
}

// is_leap_year 1968=true
// is_leap_year 1969=false
// is_leap_year 1970=false
// is_leap_year 1971=false
// is_leap_year 1972=true
// is_leap_year 1973=false
// is_leap_year 1974=false
// is_leap_year 1975=false
// is_leap_year 1976=true
impl From<SystemTime> for DateTime {
    fn from(v: SystemTime) -> DateTime {
        let year_1968 = UNIX_EPOCH - Duration::from_day(365) - Duration::from_day(366);
        if v >= year_1968 {
            let dur = v.duration_since(year_1968).unwrap_or_default();
            let sec = dur.as_secs();
            let days = sec / (24 * 3600);
            let num_four = days / (FOUR_YEAR as u64);
            let mut leap_index = 0;//[leap year,year,year,year]
            let mut year = 1968 + num_four * 4;
            let remain_days = days - num_four * (FOUR_YEAR as u64);
            if days % (FOUR_YEAR as u64) != 0 {
                if remain_days > (366 + 365 + 365) {
                    year += 3;
                    leap_index = 3;
                } else if remain_days > (366 + 365) {
                    year += 2;
                    leap_index = 2;
                } else if remain_days > (366) {
                    year += 1;
                    leap_index = 1;
                } else {
                    leap_index = 0;
                }
            }
            println!("year={}", year);
            match leap_index {
                0 => {
                    let day = remain_days;
                    let (mon, day) = find_mon_day(true, day);
                    println!("{},{}", mon, day);
                }
                1 => {
                    let day = remain_days - 366;
                    let (mon, day) = find_mon_day(false, day);
                    println!("{},{}", mon, day);
                }
                2 => {
                    let day = remain_days - 366 - 365;
                    let (mon, day) = find_mon_day(false, day);
                    println!("{},{}", mon, day);
                }
                3 => {
                    let day = remain_days - 366 - 365 - 365;
                    let (mon, day) = find_mon_day(false, day);
                    println!("{},{}", mon, day);
                }
                _ => {}
            }
        } else {
            let dur = year_1968.duration_since(v).unwrap_or_default();
        }
        DateTime {
            nano: 0,
            sec: 0,
            min: 0,
            hour: 0,
            day: 0,
            mon: 0,
            year: 0,
        }
    }
}

impl From<DateTime> for SystemTime {
    fn from(v: DateTime) -> SystemTime {
        let leap_years = ((v.year as i16 - 1) - 1968) / 4 - ((v.year as i16 - 1) - 1900) / 100 + ((v.year as i16 - 1) - 1600) / 400;
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
        let days = (v.year as i64 - 1970) * 365 + leap_years as i64 + ydays as i64;

        let mut t;
        if v.nano > 0 {
            t = UNIX_EPOCH + Duration::from_nanos(v.nano as u64)
        } else {
            t = UNIX_EPOCH - Duration::from_nanos(v.nano as u64)
        }
        if days >= 0 {
            t = t + Duration::from_secs(
                v.sec as u64 + v.min as u64 * 60 + v.hour as u64 * 3600 + days as u64 * 86400,
            );
        } else {
            t = t - Duration::from_secs(
                v.sec as u64 + v.min as u64 * 60 + v.hour as u64 * 3600 + (-days) as u64 * 86400,
            );
        }
        t
    }
}

impl From<Date> for DateTime {
    fn from(arg: Date) -> Self {
        Self {
            nano: 0,
            sec: 0,
            min: 0,
            hour: 0,
            day: arg.day,
            mon: arg.mon,
            year: arg.year,
        }
    }
}

impl From<Time> for DateTime {
    fn from(arg: Time) -> Self {
        Self {
            nano: arg.nano,
            sec: arg.sec,
            min: arg.min,
            hour: arg.hour,
            day: 0,
            mon: 0,
            year: 0,
        }
    }
}

impl FromStr for DateTime {
    type Err = Error;

    /// from RFC3339Nano = "0000-00-00 00:00:00.000000000"
    fn from_str(s: &str) -> Result<DateTime, Error> {
        let bytes = s.as_bytes();
        let mut date = DateTime {
            nano: 0,
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
            date.nano = t.nano;
            let start = 11 + offset;
            //+09:00
            let mut offset_sec = 0;
            if s.len() > start {
                let remin_str = &s[start..];
                let remin_bytes = remin_str.as_bytes();
                if remin_str.len() == 3 {
                    if remin_bytes[0] == b'+' || remin_bytes[0] == b'Z' {
                        offset_sec += ((remin_bytes[1] - b'0') as i32 * 10
                            + (remin_bytes[2] - b'0') as i32)
                            * 3600;
                    } else if remin_bytes[0] == b'-' {
                        offset_sec -= ((remin_bytes[1] - b'0') as i32 * 10
                            + (remin_bytes[2] - b'0') as i32)
                            * 3600;
                    }
                } else if remin_str.len() == 6 {
                    if remin_bytes[0] == b'+' || remin_bytes[0] == b'Z' {
                        //hour
                        offset_sec += ((remin_bytes[1] - b'0') as i32 * 10
                            + (remin_bytes[2] - b'0') as i32)
                            * 3600;
                        //min
                        offset_sec += ((remin_bytes[4] - b'0') as i32 * 10
                            + (remin_bytes[5] - b'0') as i32)
                            * 60;
                    } else if remin_bytes[0] == b'-' {
                        //hour
                        offset_sec -= ((remin_bytes[1] - b'0') as i32 * 10
                            + (remin_bytes[2] - b'0') as i32)
                            * 3600;
                        //min
                        offset_sec -= ((remin_bytes[4] - b'0') as i32 * 10
                            + (remin_bytes[5] - b'0') as i32)
                            * 60;
                    }
                }
            }
            if offset_sec > 0 {
                date = date.add(Duration::from_secs(offset_sec as u64));
            } else if offset_sec < 0 {
                date = date.sub(Duration::from_secs(offset_sec.abs() as u64));
            }
            if bytes[bytes.len() - 1] == 'Z' as u8 {
                date = date.set_offset(crate::offset_sec()); //append offset
            }
        }
        Ok(date)
    }
}

impl Display for DateTime {
    /// fmt RFC3339Nano = "2006-01-02T15:04:05.999999999"
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let mut buf: [u8; 29] = *b"0000-00-00 00:00:00.000000000";
        buf[0] = b'0' + (self.year / 1000) as u8;
        buf[1] = b'0' + (self.year / 100 % 10) as u8;
        buf[2] = b'0' + (self.year / 10 % 10) as u8;
        buf[3] = b'0' + (self.year % 10) as u8;
        buf[5] = b'0' + (self.mon / 10) as u8;
        buf[6] = b'0' + (self.mon % 10) as u8;
        buf[8] = b'0' + (self.day / 10) as u8;
        buf[9] = b'0' + (self.day % 10) as u8;
        let time = Time::from(self.clone());
        let len = time.display_time(11, &mut buf);
        f.write_str(std::str::from_utf8(&buf[..len]).unwrap())
    }
}

impl Ord for DateTime {
    fn cmp(&self, other: &DateTime) -> cmp::Ordering {
        SystemTime::from(self.clone()).cmp(&SystemTime::from(other.clone()))
    }
}

impl PartialOrd for DateTime {
    fn partial_cmp(&self, other: &DateTime) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}


impl Serialize for DateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for DateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        use serde::de::Error;
        let s = String::deserialize(deserializer)?;
        DateTime::from_str(&s).map_err(|e| D::Error::custom(e))
    }
}


//平年
static DefaultYear: [u64; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
//闰年
static LeapYear: [u64; 12] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

pub fn is_leap_year(y: u16) -> bool {
    y % 4 == 0 && (y % 100 != 0 || y % 400 == 0)
}

pub fn sum_datetime(s: SystemTime) -> DateTime {
    let d = s.duration_since(UNIX_EPOCH).unwrap();
    let days = d.as_secs() / (24 * 3600);
    let mut total_days = days;
    if d.as_secs() % (24 * 3600) > 0 {
        total_days += 1;
    }
    let total_years = total_days / 365;
    let mut current_year = 1970;
    let mut current_year_is_leap_year = false;
    let mut mon = 0;
    for _ in 0..total_years {
        current_year += 1;
        current_year_is_leap_year = is_leap_year(current_year as u16);
        if current_year_is_leap_year {
            total_days -= 366;
        } else {
            total_days -= 365;
        }
        if total_days < 365 {
            break;
        }
    }
    if current_year_is_leap_year {
        for m in LeapYear {
            mon += 1;
            if total_days > m {
                total_days -= m;
            } else {
                break;
            }
        }
    } else {
        for m in DefaultYear {
            mon += 1;
            if total_days > m {
                total_days -= m;
            } else {
                break;
            }
        }
    }
    let left = d - Duration::from_secs(3600 * days * 24);
    let hour = left.as_secs() / (60 * 60);
    let min = (left - Duration::from_secs(hour * (60 * 60))).as_secs() / 60;
    let sec = (left - Duration::from_secs(hour * (60 * 60)) - Duration::from_secs(min * 60)).as_secs();
    let nano = (left - Duration::from_secs(hour * (60 * 60)) - Duration::from_secs(min * 60) - Duration::from_secs(sec)).as_nanos();
    DateTime {
        nano: nano as u32,
        sec: sec as u8,
        min: min as u8,
        hour: hour as u8,
        day: total_days as u8,
        mon: mon,
        year: current_year as u16,
    }
}