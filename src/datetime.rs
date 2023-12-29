use crate::error::Error;
use crate::sys::Timespec;
use crate::{Date, Time};
use once_cell::sync::Lazy;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::cmp;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Deref, Sub};
use std::str::FromStr;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use time1::format_description::well_known::Rfc3339;
use time1::UtcOffset;

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
/// Convert timestamp into/from `SystemTime` to use.
/// Supports compare and sorting.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct DateTime {
    pub inner: time1::OffsetDateTime,
}

impl DateTime {
    ///utc time
    pub fn utc() -> Self {
        Self::from_system_time(SystemTime::now(), 0)
    }
    ///local zone time
    pub fn now() -> Self {
        let offset = GLOBAL_OFFSET.deref().clone();
        Self::from_system_time(SystemTime::now(), 0).set_offset(offset)
    }

    /// set offset
    /// ```rust
    /// let mut  dt = fastdate::DateTime::utc();
    /// dt = dt.set_offset(fastdate::offset_sec());
    /// ```
    pub fn set_offset(mut self, mut offset_sec: i32) -> DateTime {
        if offset_sec >= 86399 {
            offset_sec = 86399;
        }
        if offset_sec <= -86399 {
            offset_sec = -86399;
        }
        self.inner = self
            .inner
            .to_offset(UtcOffset::from_whole_seconds(offset_sec).unwrap());
        self
    }

    /// add Duration
    pub fn add(mut self, d: Duration) -> Self {
        self.inner = self.inner.add(d);
        self
    }

    /// sub Duration
    pub fn sub(mut self, d: Duration) -> Self {
        self.inner = self.inner.sub(d);
        self
    }

    ///add/sub sec
    pub fn add_sub_sec(self, sec: i64) -> Self {
        if sec >= 0 {
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
        self.inner.unix_timestamp()
    }

    ///unix_timestamp micros
    pub fn unix_timestamp_micros(&self) -> i64 {
        (self.inner.unix_timestamp_nanos() / 1000) as i64
    }

    ///unix_timestamp millis
    pub fn unix_timestamp_millis(&self) -> i64 {
        (self.inner.unix_timestamp_nanos() / 1000000) as i64
    }

    ///unix_timestamp nano
    pub fn unix_timestamp_nano(&self) -> i128 {
        self.inner.unix_timestamp_nanos()
    }

    ///from timestamp sec
    pub fn from_timestamp(sec: i64) -> DateTime {
        if sec >= 0 {
            Self::from_system_time(UNIX_EPOCH + Duration::from_secs(sec as u64), 0)
        } else {
            Self::from_system_time(UNIX_EPOCH - Duration::from_secs((-sec) as u64), 0)
        }
    }
    ///from timestamp micros
    pub fn from_timestamp_micros(micros: i64) -> DateTime {
        if micros >= 0 {
            Self::from_system_time(UNIX_EPOCH + Duration::from_micros(micros as u64), 0)
        } else {
            Self::from_system_time(UNIX_EPOCH - Duration::from_micros((-micros) as u64), 0)
        }
    }
    ///from timestamp millis
    pub fn from_timestamp_millis(ms: i64) -> DateTime {
        if ms >= 0 {
            Self::from_system_time(UNIX_EPOCH + Duration::from_millis(ms as u64), 0)
        } else {
            Self::from_system_time(UNIX_EPOCH - Duration::from_millis((-ms) as u64), 0)
        }
    }
    ///from timestamp nano
    pub fn from_timestamp_nano(nano: i128) -> DateTime {
        if nano >= 0 {
            Self::from_system_time(UNIX_EPOCH + Duration::from_nanos(nano as u64), 0)
        } else {
            Self::from_system_time(UNIX_EPOCH - Duration::from_nanos((-nano) as u64), 0)
        }
    }

    /// format support token = ["YYYY","MM","DD","hh","mm","ss",".000000",".000000000","+00:00"]
    /// ```
    /// let dt = fastdate::DateTime::from((
    ///         fastdate::Date {
    ///             day: 1,
    ///             mon: 1,
    ///             year: 2000,
    ///         },
    ///         fastdate::Time {
    ///             nano: 123456000,
    ///             sec: 11,
    ///             minute: 1,
    ///             hour: 1,
    ///         })).set_offset(8 * 60 * 60);
    ///   println!("{}",dt.format("YYYY/MM/DD/hh/mm/ss/.000000/+00:00"));
    ///   println!("{}",dt.format("YYYY-MM-DD/hh/mm/ss"));
    ///
    /// ```
    pub fn format(&self, fmt: &str) -> String {
        use std::fmt::Write;
        let (mut h, mut m, _) = self.offset_hms();
        let offset = self.offset();
        let add_sub = if offset >= 0 { '+' } else { '-' };
        let mut result = String::with_capacity(fmt.len());
        let chars = fmt.as_bytes();
        let mut index = 0;
        let mut iter = chars.iter();
        while let Some(c) = iter.next() {
            result.push(*c as char);
            if result.ends_with(".000000000") {
                for _ in 0..".000000000".len() {
                    result.pop();
                }
                write!(result, ".{:09}", self.nano()).unwrap()
            } else if result.ends_with(".000000") {
                if (index + 3) < fmt.len()
                    && chars[index + 1] == '0' as u8
                    && chars[index + 2] == '0' as u8
                    && chars[index + 3] == '0' as u8
                {
                    index += 1;
                    continue;
                }
                for _ in 0..".000000".len() {
                    result.pop();
                }
                write!(result, ".{:06}", self.nano() / 1000).unwrap();
            } else if result.ends_with("+00:00") {
                for _ in 0.."+00:00".len() {
                    result.pop();
                }
                h = h.abs();
                m = m.abs();
                write!(result, "{}{:02}:{:02}", add_sub, h, m).unwrap();
            } else if result.ends_with("YYYY") {
                for _ in 0.."YYYY".len() {
                    result.pop();
                }
                write!(result, "{:04}", self.year()).unwrap()
            } else if result.ends_with("MM") {
                for _ in 0.."MM".len() {
                    result.pop();
                }
                result.write_fmt(format_args!("{:02}", self.mon())).unwrap()
            } else if result.ends_with("DD") {
                for _ in 0.."DD".len() {
                    result.pop();
                }
                write!(result, "{:02}", self.day()).unwrap()
            } else if result.ends_with("hh") {
                for _ in 0.."hh".len() {
                    result.pop();
                }
                write!(result, "{:02}", self.hour()).unwrap()
            } else if result.ends_with("mm") {
                for _ in 0.."mm".len() {
                    result.pop();
                }
                write!(result, "{:02}", self.minute()).unwrap();
            } else if result.ends_with("ss") {
                for _ in 0.."ss".len() {
                    result.pop();
                }
                write!(result, "{:02}", self.sec()).unwrap();
            }
            index += 1;
        }
        result
    }

    /// parse an string by format.
    /// format support token = ["YYYY","MM","DD","hh","mm","ss",".000000","+00:00","Z"]
    /// format str must be example:
    /// parse nano
    /// ```rust
    ///  fastdate::DateTime::parse("YYYY-MM-DD hh:mm:ss.000000000Z", "2022-12-13 11:12:14.123456789Z").unwrap();
    ///  fastdate::DateTime::parse("YYYY-MM-DD hh:mm:ss.000000000+00:00", "2022-12-13 11:12:14.123456789+06:00").unwrap();
    /// ```
    /// or time zone(UTC+Hour)
    /// ```rust
    ///  fastdate::DateTime::parse("YYYY-MM-DD hh:mm:ss.000000+00:00", "2022-12-13 11:12:14.123456+06:00").unwrap();
    ///  fastdate::DateTime::parse("YYYY-MM-DD hh:mm:ss.000000+00:00", "2022-12-13 11:12:14.123456-03:00").unwrap();
    /// ```
    /// or time zone(UTC)
    /// ```rust
    ///  fastdate::DateTime::parse("YYYY-MM-DD hh:mm:ss.000000Z", "2022-12-13 11:12:14.123456Z").unwrap();
    /// ```
    /// parse local time
    /// ```rust
    ///  fastdate::DateTime::parse("YYYY-MM-DD hh:mm:ss.000000","2022-12-13 11:12:14.123456").unwrap();
    /// ```
    /// or any position
    /// ```rust
    ///  fastdate::DateTime::parse("YYYY-MM-DD,hh:mm:ss.000000","2022-12-13,11:12:14.123456").unwrap();
    /// ```
    /// or time zone(UTC)
    /// ```rust
    ///  fastdate::DateTime::parse("YYYY-MM-DD hh:mm:ss.000000Z", "2022-12-13 11:12:14.123456Z").unwrap();
    /// ```
    /// or time zone(UTC+Hour)
    /// ```rust
    ///  fastdate::DateTime::parse("YYYY-MM-DD hh:mm:ss.000000+00:00", "2022-12-13 11:12:14.123456+08:00").unwrap();
    /// ```
    /// ```
    pub fn parse(format: &str, arg: &str) -> Result<DateTime, Error> {
        let mut len = 19;
        //this is RFC3339 datetime buffer
        let bytes = arg.as_bytes();
        let mut buf: [u8; 35] = *b"0000-00-00T00:00:00.000000000+00:00";
        if let Some(year) = format.find("YYYY") {
            for index in 0..4 {
                buf[index] = *bytes
                    .get(year + index)
                    .ok_or_else(|| Error::from("warn 'YYYY'"))?;
            }
        }
        if let Some(mon) = format.find("MM") {
            for index in 0..2 {
                buf[5 + index] = *bytes
                    .get(mon + index)
                    .ok_or_else(|| Error::from("warn 'MM'"))?;
            }
        }
        if let Some(day) = format.find("DD") {
            for index in 0..2 {
                buf[8 + index] = *bytes
                    .get(day + index)
                    .ok_or_else(|| Error::from("warn 'DD'"))?;
            }
        }
        if let Some(hour) = format.find("hh") {
            for index in 0..2 {
                buf[11 + index] = *bytes
                    .get(hour + index)
                    .ok_or_else(|| Error::from("warn 'hh'"))?;
            }
        }
        if let Some(minute) = format.find("mm") {
            for index in 0..2 {
                buf[14 + index] = *bytes
                    .get(minute + index)
                    .ok_or_else(|| Error::from("warn 'mm'"))?;
            }
        }
        if let Some(sec) = format.find("ss") {
            for index in 0..2 {
                buf[17 + index] = *bytes
                    .get(sec + index)
                    .ok_or_else(|| Error::from("warn 'ss'"))?;
            }
        }
        let mut find_nano = false;
        //parse '.000000000'
        if let Some(nano) = format.find(".000000000") {
            for index in 0..10 {
                buf[19 + index] = *bytes
                    .get(nano + index)
                    .ok_or_else(|| Error::from("warn '.000000000'"))?;
            }
            len += 10;
            find_nano = true;
        }
        if find_nano == false {
            if let Some(micro) = format.find(".000000") {
                for index in 0..7 {
                    buf[19 + index] = *bytes
                        .get(micro + index)
                        .ok_or_else(|| Error::from("warn '.000000'"))?;
                }
                len += 7;
            }
        }
        let mut have_offset = false;
        if let Some(_) = format.find("Z") {
            buf[len] = 'Z' as u8;
            len += 1;
            have_offset = true;
        }
        if let Some(zone) = format.find("+00:00") {
            for index in 0..6 {
                let x = bytes
                    .get(zone + index)
                    .ok_or_else(|| Error::from("warn '+00:00'"))?;
                buf[len + index] = *x;
            }
            len += 6;
            have_offset = true;
        }
        if have_offset == false {
            let offset_sec = offset_sec();
            let of = UtcOffset::from_whole_seconds(offset_sec).unwrap();
            let (h, m, _) = of.as_hms();
            if offset_sec >= 0 {
                buf[len] = b'+';
                len += 1;
            } else {
                buf[len] = b'-';
                len += 1;
            }
            buf[len] = b'0' + (h.abs() / 10) as u8;
            len += 1;
            buf[len] = b'0' + (h.abs() % 10) as u8;
            len += 1;
            buf[len] = b':';
            len += 1;
            buf[len] = b'0' + (m.abs() / 10) as u8;
            len += 1;
            buf[len] = b'0' + (m.abs() % 10) as u8;
            len += 1;
        }
        let str = std::str::from_utf8(&buf[..len]).unwrap_or_default();
        let inner = time1::OffsetDateTime::parse(str, &Rfc3339)
            .map_err(|e| Error::from(format!("{} of '{}'", e, arg)))?;
        Ok(Self { inner })
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

    pub fn nano(&self) -> u32 {
        self.inner.nanosecond()
    }

    pub fn ms(&self) -> u16 {
        self.inner.millisecond()
    }

    /// get micro secs
    pub fn micro(&self) -> u32 {
        self.inner.microsecond()
    }

    /// get sec
    pub fn sec(&self) -> u8 {
        self.inner.second()
    }

    /// minute
    pub fn minute(&self) -> u8 {
        self.inner.minute()
    }

    /// get hour
    pub fn hour(&self) -> u8 {
        self.inner.hour()
    }

    /// get day
    pub fn day(&self) -> u8 {
        self.inner.day()
    }

    /// get mon
    pub fn mon(&self) -> u8 {
        self.inner.month() as u8
    }

    /// get year
    pub fn year(&self) -> i32 {
        self.inner.year()
    }

    ///offset sec
    pub fn offset(&self) -> i32 {
        self.inner.offset().whole_seconds()
    }

    ///offset_hms: hour,minute,sec
    pub fn offset_hms(&self) -> (i8, i8, i8) {
        self.inner.offset().as_hms()
    }

    pub fn from_system_time(s: SystemTime, offset: i32) -> Self {
        Self {
            inner: time1::OffsetDateTime::from(s),
        }
        .set_offset(offset)
    }

    /// stand "0000-00-00 00:00:00.000000000"
    pub fn display_stand(&self) -> String {
        let mut v = self.display(false);
        v.replace_range(10..11, " ");
        v
    }

    /// RFC3339 "0000-00-00T00:00:00.000000000Z"
    /// RFC3339 "0000-00-00T00:00:00.000000000+00:00:00"
    pub fn display(&self, zone: bool) -> String {
        let mut buf: [u8; 38] = *b"0000-00-00T00:00:00.000000000+00:00:00";
        let len = self.do_display(&mut buf, zone);
        std::str::from_utf8(&buf[..len]).unwrap().to_string()
    }

    /// let mut buf: [u8; 38] = *b"0000-00-00T00:00:00.000000000+00:00:00";
    /// than print this:
    /// RFC3339 "0000-00-00T00:00:00.000000000Z"
    /// RFC3339 "0000-00-00T00:00:00.000000000+00:00:00"
    pub fn do_display(&self, buf: &mut [u8; 38], add_zone: bool) -> usize {
        let year = self.year();
        let mon = self.mon();
        let day = self.day();
        buf[0] = b'0' + (year / 1000) as u8;
        buf[1] = b'0' + (year / 100 % 10) as u8;
        buf[2] = b'0' + (year / 10 % 10) as u8;
        buf[3] = b'0' + (year % 10) as u8;
        buf[5] = b'0' + (mon / 10);
        buf[6] = b'0' + (mon % 10);
        buf[8] = b'0' + (day / 10);
        buf[9] = b'0' + (day % 10);
        let time = Time::from(self.clone());
        let mut len = time.display_time(11, buf);
        if add_zone {
            let offset = self.offset();
            if offset == 0 {
                buf[len] = b'Z';
                len += 1;
            } else {
                let (h, m, s) = self.offset_hms();
                if offset >= 0 {
                    buf[len] = b'+';
                    len += 1;
                    buf[len] = b'0' + (h as u8 / 10);
                    len += 1;
                    buf[len] = b'0' + (h as u8 % 10);
                    len += 1;
                    buf[len] = b':' + (m as u8 / 10);
                    len += 1;
                    buf[len] = b'0' + (m as u8 / 10);
                    len += 1;
                    buf[len] = b'0' + (m as u8 % 10);
                    len += 1;
                    if s != 0 {
                        buf[len] = b':' + (s as u8 / 10);
                        len += 1;
                        buf[len] = b'0' + (s as u8 / 10);
                        len += 1;
                        buf[len] = b'0' + (s as u8 % 10);
                        len += 1;
                    }
                } else {
                    buf[len] = b'-';
                    len += 1;
                    buf[len] = b'0' + (-h as u8 / 10);
                    len += 1;
                    buf[len] = b'0' + (-h as u8 % 10);
                    len += 1;
                    buf[len] = b':' + (-m as u8 / 10);
                    len += 1;
                    buf[len] = b'0' + (-m as u8 / 10);
                    len += 1;
                    buf[len] = b'0' + (-m as u8 % 10);
                    len += 1;
                    if s != 0 {
                        buf[len] = b':' + (-s as u8 / 10);
                        len += 1;
                        buf[len] = b'0' + (-s as u8 / 10);
                        len += 1;
                        buf[len] = b'0' + (-s as u8 % 10);
                        len += 1;
                    }
                }
            }
        }
        len
    }

    pub fn set_nano(mut self, nano: u32) -> Self {
        let v = self.nano();
        if nano != v {
            self = self.sub(Duration::from_nanos(v as u64));
            self = self.add(Duration::from_micros(nano as u64));
        }
        self
    }

    pub fn from_str_default(arg: &str, default_offset: i32) -> Result<DateTime, Error> {
        let mut v = arg.to_string();
        if v.len() == 10 {
            v.push_str("T00:00:00.00");
        }
        if v.len() > 10 && &v[10..11] != "T" {
            v.replace_range(10..11, "T");
        }
        let mut have_offset = None;
        if v.ends_with("Z") {
            v.pop();
            v.push_str("+00:00");
            have_offset = Some(v.len() - 6);
        } else {
            if v.len() >= 6 {
                let index = v.len() - 6;
                let b = &v[index..(index + 1)];
                if b == "+" || b == "-" {
                    have_offset = Some(index);
                }
            }
        }
        if let Some(mut offset) = have_offset {
            if offset >= 1 {
                offset = offset - 1;
                if v.len() > offset {
                    if &v[offset..(offset + 1)] == " " {
                        v.remove(offset);
                    }
                }
            }
        }
        if have_offset.is_none() {
            let of = UtcOffset::from_whole_seconds(default_offset).unwrap();
            let (h, m, _) = of.as_hms();
            if h >= 0 && m >= 0 {
                v.push_str(&format!("+{:02}:{:02}", h.abs(), m.abs()));
            } else {
                v.push_str(&format!("-{:02}:{:02}", h.abs(), m.abs()));
            }
        }
        let inner = time1::OffsetDateTime::parse(&v, &Rfc3339)
            .map_err(|e| Error::from(format!("{} of '{}'", e, arg)))?;
        Ok(Self { inner })
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

impl From<SystemTime> for DateTime {
    fn from(v: SystemTime) -> DateTime {
        DateTime::from_system_time(v, 0)
    }
}

impl From<DateTime> for SystemTime {
    fn from(v: DateTime) -> SystemTime {
        let nano = v.unix_timestamp_nano();
        if nano >= 0 {
            UNIX_EPOCH + Duration::from_nanos(nano as u64)
        } else {
            UNIX_EPOCH - Duration::from_nanos(nano as u64)
        }
    }
}

impl From<Date> for DateTime {
    fn from(arg: Date) -> Self {
        Self::from_str(&format!(
            "{:04}-{:02}-{:02} 00:00:00.000000000Z",
            arg.year, arg.mon, arg.day
        ))
        .unwrap()
    }
}

/// from((Date{},offset_sec()))
impl From<(Date, i32)> for DateTime {
    fn from(arg: (Date, i32)) -> Self {
        Self::from(arg.0)
            .set_offset(arg.1)
            .add_sub_sec(-arg.1 as i64)
    }
}

impl From<Time> for DateTime {
    fn from(arg: Time) -> Self {
        Self::from_str(&format!(
            "0000-01-01 {:02}:{:02}:{:02}.{:09}Z",
            arg.hour, arg.minute, arg.sec, arg.nano
        ))
        .unwrap()
    }
}

impl From<(Date, Time)> for DateTime {
    fn from(arg: (Date, Time)) -> Self {
        Self::from_str(&format!(
            "{:04}-{:02}-{:02} {:02}:{:02}:{:02}.{:09}Z",
            arg.0.year, arg.0.mon, arg.0.day, arg.1.hour, arg.1.minute, arg.1.sec, arg.1.nano
        ))
        .unwrap()
    }
}

///from(Date{},Time{},offset_sec())
impl From<(Date, Time, i32)> for DateTime {
    fn from(arg: (Date, Time, i32)) -> Self {
        let mut datetime = Self::from_str(&format!(
            "{:04}-{:02}-{:02} {:02}:{:02}:{:02}.{:09}Z",
            arg.0.year, arg.0.mon, arg.0.day, arg.1.hour, arg.1.minute, arg.1.sec, arg.1.nano
        ))
        .unwrap();
        datetime = datetime.set_offset(arg.2).add_sub_sec(-arg.2 as i64);
        datetime
    }
}

impl FromStr for DateTime {
    type Err = Error;

    /// parse_from_str
    ///
    /// "2019-10-12T07:20:50.52Z"          (UTC+0)
    /// "2019-10-12T07:20:50.52+00:00"     (UTC+0)
    /// "2019-10-12T14:20:50.52+07:00"     (UTC+7)
    /// "2019-10-12T03:20:50.52-04:00"     (UTC-4)
    fn from_str(arg: &str) -> Result<DateTime, Error> {
        return Self::from_str_default(arg, offset_sec());
    }
}

impl Display for DateTime {
    /// fmt RFC3339Nano = "2006-01-02T15:04:05.999999999"
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let mut buf: [u8; 38] = *b"0000-00-00T00:00:00.000000000+00:00:00";
        let len = self.do_display(&mut buf, true);
        f.write_str(std::str::from_utf8(&buf[..len]).unwrap())
    }
}

impl Ord for DateTime {
    fn cmp(&self, other: &DateTime) -> cmp::Ordering {
        self.unix_timestamp_nano().cmp(&other.unix_timestamp_nano())
    }
}

impl PartialOrd for DateTime {
    fn partial_cmp(&self, other: &DateTime) -> Option<cmp::Ordering> {
        Some(self.unix_timestamp_nano().cmp(&other.unix_timestamp_nano()))
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

#[cfg(not(tarpaulin_include))]
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
