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
use time1::{Month, UtcOffset};
use time1::format_description::well_known::Rfc3339;

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
        if sec > 0 {
            Self::from_system_time(UNIX_EPOCH + Duration::from_secs(sec as u64), 0)
        } else {
            Self::from_system_time(UNIX_EPOCH - Duration::from_secs((-sec) as u64), 0)
        }
    }
    ///from timestamp micros
    pub fn from_timestamp_micros(micros: i64) -> DateTime {
        if micros > 0 {
            Self::from_system_time(UNIX_EPOCH + Duration::from_micros(micros as u64), 0)
        } else {
            Self::from_system_time(UNIX_EPOCH - Duration::from_micros((-micros) as u64), 0)
        }
    }
    ///from timestamp millis
    pub fn from_timestamp_millis(ms: i64) -> DateTime {
        if ms > 0 {
            Self::from_system_time(UNIX_EPOCH + Duration::from_millis(ms as u64), 0)
        } else {
            Self::from_system_time(UNIX_EPOCH - Duration::from_millis((-ms) as u64), 0)
        }
    }
    ///from timestamp nano
    pub fn from_timestamp_nano(nano: i128) -> DateTime {
        if nano > 0 {
            Self::from_system_time(UNIX_EPOCH + Duration::from_nanos(nano as u64), 0)
        } else {
            Self::from_system_time(UNIX_EPOCH - Duration::from_nanos((-nano) as u64), 0)
        }
    }

    /// format support token = ["YYYY","MM","DD","hh","mm","ss",".000000","+00:00"]
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
        let (h, m, _) = self.offset_hms();
        fmt.replacen("YYYY", &self.year().to_string(), 1)
            .replacen("MM", &self.mon().to_string(), 1)
            .replacen("DD", &self.day().to_string(), 1)
            .replacen("hh", &self.hour().to_string(), 1)
            .replacen("mm", &self.minute().to_string(), 1)
            .replacen("ss", &self.sec().to_string(), 1)
            .replacen(".000000000", &format!(".{:09}", self.nano()), 1)
            .replacen(".000000", &format!(".{:06}", self.micro()), 1)
            .replacen("+00:00", &format!("+{:02}:{:02}", h, m), 1)
            .to_string()
    }

    /// parse an string by format.
    /// format support token = ["YYYY","MM","DD","hh","mm","ss",".000000","+00:00"]
    /// format str must be example:
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
        let mut buf: [u8; 32] = *b"0000-00-00T00:00:00.000000+00:00";
        if let Some(year) = format.find("YYYY") {
            let mut index = 0;
            for x in arg[year..(year + 4)].bytes() {
                buf[index] = x;
                index += 1;
            }
        }
        if let Some(mon) = format.find("MM") {
            let mut index = 5;
            for x in arg[mon..(mon + 2)].bytes() {
                buf[index] = x;
                index += 1;
            }
        }
        if let Some(day) = format.find("DD") {
            let mut index = 8;
            for x in arg[day..(day + 2)].bytes() {
                buf[index] = x;
                index += 1;
            }
        }
        if let Some(hour) = format.find("hh") {
            let mut index = 11;
            for x in arg[hour..(hour + 2)].bytes() {
                buf[index] = x;
                index += 1;
            }
        }
        if let Some(minute) = format.find("mm") {
            let mut index = 14;
            for x in arg[minute..(minute + 2)].bytes() {
                buf[index] = x;
                index += 1;
            }
        }
        if let Some(sec) = format.find("ss") {
            let mut index = 17;
            for x in arg[sec..(sec + 2)].bytes() {
                buf[index] = x;
                index += 1;
            }
        }
        let find_nano = false;
        //todo better way parse '.000000000'
        // if let Some(nano) = format.find(".000000000") {
        //     let mut index = 19;
        //     for x in arg[nano..(nano + 10)].bytes() {
        //         buf[index] = x;
        //         index += 1;
        //     }
        //     len += 10;
        //     find_nano = true;
        // }
        if find_nano == false{
            if let Some(micro) = format.find(".000000") {
                let mut index = 19;
                for x in arg[micro..(micro + 7)].bytes() {
                    buf[index] = x;
                    index += 1;
                }
                len += 7;
            }
        }
        if let Some(zone) = format.find("Z") {
            buf[zone] = 'Z' as u8;
            len +=1;
        }
        if let Some(zone) = format.find("+00:00") {
            let mut index = len;
            for x in arg[zone..(zone + 6)].bytes() {
                buf[index] = x;
                index += 1;
            }
            len += 6;
        }
        let str = std::str::from_utf8(&buf[..len]).unwrap_or_default();
        let inner = DateTime::from_str(str)?;
        Ok(inner)
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
        match self.inner.month() {
            Month::January => 1,
            Month::February => 2,
            Month::March => 3,
            Month::April => 4,
            Month::May => 5,
            Month::June => 6,
            Month::July => 7,
            Month::August => 8,
            Month::September => 9,
            Month::October => 10,
            Month::November => 11,
            Month::December => 12,
        }
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
                if offset > 0 {
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
            "0000-00-00 {:02}:{:02}:{:02}.{:09}Z",
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
            let of = UtcOffset::from_whole_seconds(offset_sec()).unwrap();
            let (h, m, _) = of.as_hms();
            if h >= 0 && m >= 0 {
                v.push_str(&format!("+{:02}:{:02}", h, m));
            } else {
                v.push_str(&format!("-{:02}:{:02}", h, m));
            }
        }
        let inner = time1::OffsetDateTime::parse(&v, &Rfc3339).map_err(|e| {
            let info = format!("{} of '{}'", e, arg);
            Error::from(info)
        })?;
        Ok(Self { inner })
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
