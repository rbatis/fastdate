use fastdate::{Date, DateTime, DurationFrom, Time};
use std::cmp::Ordering;
use std::str::FromStr;
use std::time::{Duration, SystemTime};

#[test]
fn test_other_space() {
    let d = DateTime::parse("YYYY-MM-DDThh_mm_ss.000000Z", "1234_12_13_11_12_13.123456").unwrap();
    println!("{}", d);
    assert_eq!("1234-12-13T11:12:13.123456Z".to_string(), d.to_string());
}

#[test]
fn test_date() {
    let d = DateTime::from_str("1234-12-13 11:12:13.123456Z").unwrap();
    println!("{}", d);
    assert_eq!(d.to_string(), "1234-12-13T11:12:13.123456Z".to_string());
}

#[test]
fn test_date2() {
    let d = DateTime::from_str("1234-12-13 11:12:13.123456789Z").unwrap();
    println!("{}", d);
    assert_eq!(d.to_string(), "1234-12-13T11:12:13.123456789Z".to_string());
}

#[test]
fn test_date_local() {
    let d = DateTime::now();
    println!("{}", d);
    println!("{}", d.unix_timestamp());
}

#[test]
fn test_date_utc() {
    let d = DateTime::utc();
    println!("{}", d);
}

#[test]
fn test_date_utc_add() {
    let d = DateTime::now();
    let added = d.clone() + Duration::from_secs(1);
    println!("{},{}", d, added);
    assert_eq!(d.add_duration(Duration::from_secs(1)), added);
}

#[test]
fn test_offset() {
    let utc = DateTime::from_str("2022-12-12T12:12:12.000000+08:00").unwrap();
    println!("{}", utc);
    println!("{}", utc.offset());
    assert_eq!(utc.offset(), 28800);
}

#[test]
fn test_timestamp() {
    let now = DateTime::utc();
    let timestamp = now.unix_timestamp();
    let new_time = DateTime::from_timestamp(timestamp);
    assert_eq!(timestamp, new_time.unix_timestamp());
}

#[test]
fn test_timestamp_sub() {
    let now = DateTime::from_str("1800-01-01 00:00:00Z").unwrap();
    let timestamp = now.unix_timestamp();
    let new_time = DateTime::from_timestamp(timestamp);
    assert_eq!(timestamp, new_time.unix_timestamp());
}

// #[test]
// fn test_timestamp_befor_epoch() {
//     let before = -5259600000;//1969-11-01 03:00:00
//     let date = DateTime::from_timestamp_millis(before);
//
//     println!("get {:?}", date);
//     println!("want {:?}", DateTime {
//         nano: 0,
//         sec: 0,
//         min: 0,
//         hour: 3,
//         day: 1,
//         mon: 11,
//         year: 1969,
//         offset: 0,
//     });
//     assert_eq!(date.to_string(), "1969-11-01 03:00:00");
// }

#[test]
fn test_from_timestamp_nano() {
    let now = DateTime::utc();
    let timestamp = now.unix_timestamp_nano();
    let new_time = DateTime::from_timestamp_nano(timestamp);
    assert_eq!(now, new_time);
}

#[test]
fn test_from_timestamp_nano2() {
    let now = DateTime::from_str("1600-11-15 15:37:33.595407Z").unwrap();
    let timestamp = now.unix_timestamp_nano();
    let new_time = DateTime::from_timestamp_nano(timestamp);
    assert_eq!(now, new_time);
}

#[test]
fn test_from_timestamp_micros() {
    let now = DateTime::from_str("2023-11-15 15:37:33.595407Z").unwrap();
    let timestamp = now.unix_timestamp_micros();
    let new_time = DateTime::from_timestamp_micros(timestamp);
    assert_eq!(now, new_time);
}

#[test]
fn test_from_timestamp_micros2() {
    let now = DateTime::from_str("1600-11-15 15:37:33.595407Z").unwrap();
    let timestamp = now.unix_timestamp_micros();
    let new_time = DateTime::from_timestamp_micros(timestamp);
    assert_eq!(now, new_time);
}

#[test]
fn test_timestamp_millis() {
    let now = DateTime::utc();
    let timestamp = now.unix_timestamp_millis();
    let new_time = DateTime::from_timestamp_millis(timestamp);
    assert_eq!(timestamp, new_time.unix_timestamp_millis());
}

#[test]
fn test_timestamp_nano() {
    let now = DateTime::utc();
    let timestamp = now.unix_timestamp_nano();
    let new_time = DateTime::from_timestamp_nano(timestamp);
    assert_eq!(now, new_time);
}

#[test]
fn test_date_time() {
    let new_time = DateTime::from((
        Date {
            day: 12,
            mon: 12,
            year: 2023,
        },
        Time {
            nano: 12,
            sec: 12,
            minute: 12,
            hour: 12,
        },
    ));
    assert_eq!(new_time.to_string(), "2023-12-12T12:12:12.000000012Z");
}

#[test]
fn test_set_offset() {
    let new_time = DateTime::from((
        Date {
            day: 12,
            mon: 12,
            year: 2023,
        },
        Time {
            nano: 12,
            sec: 12,
            minute: 12,
            hour: 12,
        },
    ));
    let timestamp_nano = new_time.unix_timestamp_nano();
    assert_eq!(new_time.to_string(), "2023-12-12T12:12:12.000000012Z");
    let new_time = new_time.set_offset(8 * 3600);
    assert_eq!(new_time.to_string(), "2023-12-12T20:12:12.000000012+08:00");
    assert_eq!(new_time.unix_timestamp_nano(), timestamp_nano);
}

#[test]
fn test_set_offset2() {
    let new_time = DateTime::from((
        Date {
            day: 12,
            mon: 12,
            year: 2023,
        },
        Time {
            nano: 12,
            sec: 12,
            minute: 12,
            hour: 12,
        },
        8 * 3600,
    ));
    println!("{}", new_time.display_stand());
    assert_eq!(
        new_time.display(true),
        "2023-12-12T12:12:12.000000012+08:00"
    );
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
fn test_offset_zone2() {
    let mut epoch = fastdate::DateTime::from(Date {
        day: 1,
        mon: 1,
        year: 2000,
    });
    println!("{}", epoch);
    epoch = epoch.set_offset(8 * 3600).add_sub_sec(-8 * 3600);
    println!("{}", epoch);
}

#[test]
fn test_into() {
    let utc = DateTime::from_str("2022-12-12 00:00:00+08:00").unwrap();
    let date: Date = utc.clone().into();
    let time: Time = utc.clone().into();
    assert_eq!("2022-12-12", date.to_string());
    assert_eq!("00:00:00", time.to_string());
    assert_eq!(utc.offset(), 28800);
}

#[test]
fn test_befor_after() {
    let date1 = DateTime::from_str("2022-12-12 00:00:00").unwrap();
    let date2 = DateTime::from_str("2022-12-12 01:00:00").unwrap();
    assert!(date2.after(&date1));
    assert!(date1.before(&date2));
}

#[test]
fn test_from_str_date() {
    let date1 = DateTime::from_str("2022-12-12").unwrap();
    assert!(date1.to_string().starts_with("2022-12-12T00:00:00"));
}

#[test]
fn test_from_str_zone() {
    let date1 = DateTime::from_str("2022-12-12 00:00:00+08:00").unwrap();
    println!("{}", date1);
    assert_eq!(date1.offset(), 8 * 3600);
}

#[test]
fn test_from_str_zone_add() {
    let date1 = DateTime::from_str("2022-12-12T00:00:00 +08:00").unwrap();
    assert_eq!(date1.offset(), 28800);
    println!("{}", date1);
}

#[test]
fn test_from_str_zone_sub() {
    let date1 = DateTime::from_str("2022-12-12T00:00:00-08:00").unwrap();
    assert_eq!(date1.offset(), -28800);
    println!("{}", date1);
}

#[test]
fn test_from_str_default_no_zone() {
    let date1 = DateTime::from_str_default("2022-12-12T00:00:00", 28800).unwrap();
    println!("{}", date1);
    assert_eq!(date1.offset(), 28800);
}

#[test]
fn test_from_str_default_no_zone2() {
    let date1 = DateTime::from_str_default("2022-12-12T00:00:00", -(3600 + 60)).unwrap();
    println!("{}", date1);
    assert_eq!(date1.offset(), -3660);
}

#[test]
fn test_from_str_default_no_zone_sec() {
    let date1 = DateTime::from_str_default("2022-12-12T00:00:00", 28800 + 1).unwrap();
    println!("{}", date1);
    assert_eq!(date1.offset(), 28800);
}

#[test]
fn test_from_str_default_no_zone2_sec() {
    let date1 = DateTime::from_str_default("2022-12-12T00:00:00", -(3600 + 60 + 1)).unwrap();
    println!("{}", date1);
    assert_eq!(date1.offset(), -3660);
}

#[test]
fn test_from_str_fail() {
    let date1 = DateTime::from_str("2022");
    assert!(date1.is_err());
    let date1 = DateTime::from_str("");
    assert!(date1.is_err());
}

#[test]
fn test_parse_z() {
    let date = DateTime::from_str("2022-12-12 00:00:00.000000Z").unwrap();
    assert_eq!(date.to_string(), "2022-12-12T00:00:00Z");
}

#[test]
fn test_parse_z_2() {
    let date = DateTime::from_str("2022-12-12 00:00:00.123456789Z").unwrap();
    assert_eq!(date.to_string(), "2022-12-12T00:00:00.123456789Z");
}

#[test]
fn test_parse_z_add() {
    let date = DateTime::from_str("2022-12-12 00:00:00.000000+09:00").unwrap();
    let date_offset = date.clone();
    assert_eq!(date_offset.to_string(), "2022-12-12T00:00:00+09:00");
}

#[test]
fn test_parse_z_sub() {
    let date = DateTime::from_str("2022-12-12 00:00:00.000000-09:00").unwrap();
    let date_offset = date.clone();
    assert_eq!(date_offset.to_string(), "2022-12-12T00:00:00-09:00");
}

#[test]
fn test_parse_s() {
    let date = DateTime::parse("YYYY/MM/DD/hh:mm:ss", "2022/12/12/00:00:00").unwrap();
    assert_eq!(&date.to_string()[..19], "2022-12-12T00:00:00");
}

#[test]
fn test_parse_no_00() {
    let date = DateTime::from_str("2024-07-26 09:03:48+00").unwrap();
    assert_eq!(date.to_string(), "2024-07-26T09:03:48Z");
}

#[test]
fn test_set_offset_sub() {
    let mut date = DateTime::from_str("2022-12-12 09:00:00Z").unwrap();
    date = date.set_offset(-9 * 3600);
    assert_eq!(date.to_string(), "2022-12-12T00:00:00-09:00");
}

#[test]
fn test_time_sub_time() {
    let date = DateTime::from_str("2022-12-12 00:00:00").unwrap();
    let date2 = DateTime::from_str("2022-12-11 00:00:00").unwrap();
    let sub = date - date2;
    assert_eq!(86400, sub.as_secs());
}

#[test]
fn test_parse_format() {
    let date =
        DateTime::parse("YYYY-MM-DD hh:mm:ss.000000Z", "2022-12-13 11:12:14.123456Z").unwrap();
    println!("{}", date);
    assert_eq!(date.to_string(), "2022-12-13T11:12:14.123456Z");
}

#[test]
fn test_parse_format2() {
    let date =
        DateTime::parse("hh:mm:ss.000000Z,YYYY-MM-DD", "11:12:14.123456Z,2022-12-13").unwrap();
    println!("{}", date);
    assert_eq!(date.to_string(), "2022-12-13T11:12:14.123456Z");
}

#[test]
fn test_parse_format3() {
    let date = DateTime::parse("YYYYMMDD", "20231102").unwrap();
    println!("{}", date);
    assert_eq!(&date.to_string()[0..19], "2023-11-02T00:00:00");
}

#[test]
fn test_parse_format4() {
    let date =
        DateTime::parse("YYYY-MM-DD hh:mm:ss.000000Z", "2022-12-13 11:12:14.123456Z").unwrap();
    println!("{}", date);
    assert_eq!(date.to_string(), "2022-12-13T11:12:14.123456Z");
}

#[test]
fn test_parse_format5() {
    let date = DateTime::parse(
        "YYYY-MM-DD hh:mm:ss.000000+00:00",
        "2022-12-13 11:12:14.123456+06:00",
    )
    .unwrap();
    println!("{}", date);
    assert_eq!(date.to_string(), "2022-12-13T11:12:14.123456+06:00");
}

#[test]
fn test_parse_format6() {
    let date =
        DateTime::parse("hh:mm:ss.000000Z,YYYY-MM-DD", "11:12:14.123456Z,2022-12-13").unwrap();
    println!("{}", date);
    assert_eq!(date.to_string(), "2022-12-13T11:12:14.123456Z");
}

#[test]
fn test_parse_format7() {
    let date = DateTime::parse(
        "hh:mm:ss.000000+00:00,YYYY-MM-DD",
        "11:12:14.123456-08:00,2022-12-13",
    )
    .unwrap();
    println!("{}", date);
    assert_eq!(date.to_string(), "2022-12-13T11:12:14.123456-08:00");
}

#[test]
fn test_parse_format8() {
    let date = DateTime::parse(
        "YYYY-MM-DD hh:mm:ss.000000+00:00",
        "2022-12-13 11:12:14.123456+06:00",
    )
    .unwrap();
    println!("{}", date);
    assert_eq!(date.to_string(), "2022-12-13T11:12:14.123456+06:00");
}

#[test]
fn test_parse_format9() {
    let date = DateTime::parse(
        "YYYY-MM-DD hh:mm:ss.000000+00:00",
        "2022-12-13 11:12:14.123456-06:00",
    )
    .unwrap();
    println!("{}", date);
    assert_eq!(date.to_string(), "2022-12-13T11:12:14.123456-06:00");
}

#[test]
fn test_parse_format10() {
    let date = DateTime::parse(
        "YYYY-MM-DD hh:mm:ss.000000000Z",
        "2022-12-13 11:12:14.123456789Z",
    )
    .unwrap();
    println!("{}", date);
    assert_eq!(date.to_string(), "2022-12-13T11:12:14.123456789Z");
}

#[test]
fn test_parse_format11() {
    let date = DateTime::parse(
        "YYYY-MM-DD hh:mm:ss.000000000+00:00",
        "2022-12-13 11:12:14.123456789+06:00",
    )
    .unwrap();
    println!("{}", date);
    assert_eq!(date.to_string(), "2022-12-13T11:12:14.123456789+06:00");
}

#[test]
fn test_parse_format_year_fail() {
    let date = DateTime::parse("YYYY-MM-DD hh:mm:ss.000000000+00:00", "202");
    println!("{:?}", date);
    assert!(date.is_err());
}

#[test]
fn test_parse_format_mon_fail() {
    let date = DateTime::parse("YYYY-MM-DD hh:mm:ss.000000000+00:00", "2022-1");
    assert!(date.is_err());
}

#[test]
fn test_parse_format_day_fail() {
    let date = DateTime::parse("YYYY-MM-DD hh:mm:ss.000000000+00:00", "2022-12-3");
    assert!(date.is_err());
}

#[test]
fn test_parse_format_hour_fail() {
    let date = DateTime::parse("YYYY-MM-DD hh:mm:ss.000000000+00:00", "2022-12-13 1");
    assert!(date.is_err());
}

#[test]
fn test_parse_format_minute_fail() {
    let date = DateTime::parse("YYYY-MM-DD hh:mm:ss.000000000+00:00", "2022-12-13 12:1");
    assert!(date.is_err());
}

#[test]
fn test_parse_format_sec_fail() {
    let date = DateTime::parse("YYYY-MM-DD hh:mm:ss.000000000+00:00", "2022-12-13 12:12:1");
    assert!(date.is_err());
}

#[test]
fn test_parse_format_nano_fail() {
    let date = DateTime::parse(
        "YYYY-MM-DD hh:mm:ss.000000000+00:00",
        "2022-12-13 12:12:12.1",
    );
    assert!(date.is_err());
}

#[test]
fn test_parse_format_no_zone() {
    let date = DateTime::parse("YYYY-MM-DD hh:mm:ss.000000", "2022-12-13 12:12:12.000001").unwrap();
    println!("{}", date);
    assert_eq!(&date.to_string()[0..26], "2022-12-13T12:12:12.000001");
}

#[test]
fn test_parse_format_no_zone_toolong() {
    let date = DateTime::parse(
        "YYYY-MM-DD hh:mm:ss  .000000000",
        "2022-12-13 12:12:12  .000000001",
    );
    println!("{:?}", date);
    assert!(date.is_ok());
}

#[test]
fn test_parse_format_micro_fail() {
    let date = DateTime::parse("YYYY-MM-DD hh:mm:ss.000000+00:00", "2022-12-13 12:12:12");
    assert!(date.is_err());
}

#[test]
fn test_parse_format_zone_fail() {
    let date = DateTime::parse(
        "YYYY-MM-DD hh:mm:ss.000000+00:00",
        "2022-12-13 12:12:12.123456+0",
    );
    assert!(date.is_err());
}

#[test]
fn test_week() {
    let date = DateTime::from_str("2022-07-27 09:27:11.000000+08:00").unwrap();
    println!("{}", date);
    println!("week,{}", date.week_day());
    assert_eq!(3, date.week_day());
}

#[test]
fn test_nano() {
    let date1 = DateTime::from_str("2019-04-28 00:00:00.023333333Z").unwrap();
    println!("{}", date1);
    assert_eq!(date1.to_string(), "2019-04-28T00:00:00.023333333Z");
}

#[test]
fn test_parse_date() {
    let date = DateTime::from_str("2013-10-06 00:00:00Z").unwrap();
    assert_eq!(date.to_string(), "2013-10-06T00:00:00Z");
}

#[test]
fn test_parse_micro() {
    let date = DateTime::from_str("2013-10-06 00:00:00.000001Z").unwrap();
    assert_eq!(date.to_string(), "2013-10-06T00:00:00.000001Z");
}

#[test]
fn test_ser_date() {
    let date = DateTime::from_str("2023-10-13 16:57:41.123926+08:00").unwrap();
    let js = serde_json::to_string(&date).unwrap();
    assert_eq!(js, "\"2023-10-13T16:57:41.123926+08:00\"");
}

#[test]
fn test_display_href() {
    //5*3600 +  3600/2
    let date = DateTime::from_str("2023-10-13 16:57:41.123926+05:30").unwrap();
    let js = serde_json::to_string(&date).unwrap();
    assert_eq!(js, "\"2023-10-13T16:57:41.123926+05:30\"");
}

#[test]
fn test_de_date() {
    let date = DateTime::from_str("2023-10-13 16:57:41.123926+08:00").unwrap();
    let js = serde_json::to_string(&date).unwrap();
    let new_date: DateTime = serde_json::from_str(&js).unwrap();
    assert_eq!(new_date, date);
}

#[test]
fn test_de_date_fail() {
    let js = serde_json::to_string(&"2023-1-13 16:57:41.123926+08:00").unwrap();
    let new_date: Result<DateTime, serde_json::Error> = serde_json::from_str(&js);
    assert!(new_date.is_err());
}

#[test]
fn test_de_date_offset() {
    let mut date = DateTime::from_str("2023-10-13 16:57:41.123926Z").unwrap();
    date = date.set_offset(28800);
    let js = serde_json::to_string(&date).unwrap();
    assert_eq!(js, "\"2023-10-14T00:57:41.123926+08:00\"");
    let new_date = serde_json::from_str::<DateTime>(&js).unwrap();
    assert_eq!(new_date.offset(), 28800);
}

#[test]
fn test_add_minute() {
    let date = DateTime::from_str("2013-10-06 00:00:00Z")
        .unwrap()
        .add_duration(Duration::from_minute(1));
    assert_eq!(date.to_string(), "2013-10-06T00:01:00Z");
}

#[test]
fn test_add_hour() {
    let date = DateTime::from_str("2013-10-06T01:00:00Z")
        .unwrap()
        .add_duration(Duration::from_hour(1));
    assert_eq!(date.to_string(), "2013-10-06T02:00:00Z");
}

#[test]
fn test_add_day() {
    let date = DateTime::from_str("2013-10-07T00:00:00Z")
        .unwrap()
        .add_duration(Duration::from_day(1));
    assert_eq!(date.to_string(), "2013-10-08T00:00:00Z");
}

#[test]
fn test_add_sub_sec() {
    let date = DateTime::from_str("2013-10-06 00:00:00Z")
        .unwrap()
        .add_sub_sec(1);
    assert_eq!(date.to_string(), "2013-10-06T00:00:01Z");
    let date = DateTime::from_str("2013-10-06 00:00:00Z")
        .unwrap()
        .add_sub_sec(-1);
    assert_eq!(date.to_string(), "2013-10-05T23:59:59Z");
}

#[test]
fn test_add_duration() {
    let date = DateTime::from_str("2013-10-06 00:00:00Z").unwrap() + Duration::from_minute(1);
    assert_eq!(date.to_string(), "2013-10-06T00:01:00Z");
}

#[test]
fn test_sub_duration() {
    let date = DateTime::from_str("2013-10-06 00:00:00Z").unwrap() - Duration::from_minute(1);
    assert_eq!(date.to_string(), "2013-10-05T23:59:00Z");
}

#[test]
fn test_add_duration_ref() {
    let date = DateTime::from_str("2013-10-06 00:00:00Z").unwrap() + &Duration::from_minute(1);
    assert_eq!(date.to_string(), "2013-10-06T00:01:00Z");
}

#[test]
fn test_sub_duration_ref() {
    let date = DateTime::from_str("2013-10-06 00:00:00Z").unwrap() - &Duration::from_minute(1);
    assert_eq!(date.to_string(), "2013-10-05T23:59:00Z");
}

#[test]
fn test_cmd() {
    let date = DateTime::from_str("2013-10-06 00:00:00Z").unwrap();
    let date2 = DateTime::from_str("2013-10-07 00:00:00Z").unwrap();
    assert_eq!(date2.cmp(&date), Ordering::Greater);
}

#[test]
fn test_from_systime() {
    let _date = DateTime::from(SystemTime::now());
}

#[test]
fn test_from_into() {
    let now = SystemTime::now();
    let date = DateTime::from(now);
    let s: SystemTime = date.into();
    assert_eq!(s, now);
}

#[test]
fn test_from_date_offset() {
    let dt = DateTime::from((
        Date {
            year: 0000,
            day: 1,
            mon: 1,
        },
        0,
    ));
    assert_eq!(dt.to_string(), "0000-01-01T00:00:00Z");
}

#[test]
fn test_from_time_offset() {
    let dt = DateTime::from(Time {
        hour: 0,
        minute: 0,
        sec: 0,
        nano: 0,
    });
    assert_eq!(dt.to_string(), "0000-01-01T00:00:00Z");
}

#[test]
fn test_1958_unix() {
    let date = DateTime::from_str("1958-01-01T00:00:00Z").unwrap();
    println!(
        "s={:?},date={}",
        SystemTime::from(date.clone()),
        DateTime::from_system_time(SystemTime::from(date.clone()), 0)
    );
    assert_eq!(date.unix_timestamp(), -378691200);
}

#[test]
fn test_1958_week() {
    let date = DateTime::from_str("1958-01-01T00:00:00Z").unwrap();
    assert_eq!(date.week_day(), 3);
}

#[test]
fn test_1968_unix() {
    let date = DateTime::from_str("1968-01-01T00:00:00Z").unwrap();
    assert_eq!(date.unix_timestamp(), -63158400);
}

#[test]
fn test_1928_unix() {
    let date = DateTime::from_str("1928-01-01T00:00:00Z").unwrap();
    assert_eq!(date.unix_timestamp(), -1325462400);
}

#[test]
fn test_from_unix() {
    let dt = DateTime::from_timestamp_millis(-708249600000);
    println!("{}", dt);
    assert_eq!(dt.to_string(), "1947-07-23T16:00:00Z");
}

#[test]
fn test_add() {
    let epoch = fastdate::DateTime::from(fastdate::Date {
        day: 1,
        mon: 1,
        year: 2000,
    });
    let us: u64 = 693484748000000;
    let v = epoch + Duration::from_micros(us);
    println!("{}", v); //2023-02-14 07:37:40
    assert_eq!(v.to_string(), "2021-12-22T10:39:08Z");
}

#[test]
fn test_display_date() {
    let epoch = fastdate::DateTime::from(Date {
        day: 1,
        mon: 1,
        year: 2000,
    });
    let v = epoch.display(false);
    assert_eq!(v, "2000-01-01T00:00:00");
}

#[test]
fn test_display_datetime() {
    let epoch = fastdate::DateTime::from((
        Date {
            day: 1,
            mon: 1,
            year: 2000,
        },
        Time {
            nano: 1233,
            sec: 11,
            minute: 1,
            hour: 1,
        },
    ));
    let v = epoch.display(false);
    assert_eq!(v, "2000-01-01T01:01:11.000001233");
}

#[test]
fn test_display_stand() {
    let epoch = fastdate::DateTime::from((
        Date {
            day: 1,
            mon: 1,
            year: 2000,
        },
        Time {
            nano: 1233,
            sec: 11,
            minute: 1,
            hour: 1,
        },
    ));
    let v = epoch.display_stand();
    assert_eq!(v, "2000-01-01 01:01:11.000001233");
}

#[test]
fn test_do_display() {
    let epoch = fastdate::DateTime::from(Date {
        day: 1,
        mon: 1,
        year: 2000,
    })
    .set_offset(1);
    let v = epoch.display(true);
    assert_eq!(v, "2000-01-01T00:00:01+00:00:01");
}

#[test]
fn test_do_display2() {
    let epoch = fastdate::DateTime::from(Date {
        day: 1,
        mon: 1,
        year: 2000,
    })
    .set_offset(-1);
    let v = epoch.display(true);
    assert_eq!(v, "1999-12-31T23:59:59-00:00:01");
}

#[test]
fn test_set_micro() {
    let mut dt = fastdate::DateTime::from((
        Date {
            day: 1,
            mon: 1,
            year: 2000,
        },
        Time {
            nano: 1233,
            sec: 11,
            minute: 1,
            hour: 1,
        },
    ));
    dt = dt.set_nano(0);
    assert_eq!(dt.display_stand(), "2000-01-01 01:01:11");

    dt = dt.set_nano(1);
    assert_eq!(dt.display_stand(), "2000-01-01 01:01:11.000001");
}

#[test]
fn test_format() {
    let dt = fastdate::DateTime::from((
        Date {
            day: 1,
            mon: 1,
            year: 2000,
        },
        Time {
            nano: 123456789,
            sec: 11,
            minute: 1,
            hour: 1,
        },
    ));
    let f = dt.format("YYYY-MM-DD/hh/mm/ss.000000");
    assert_eq!(f, "2000-01-01/01/01/11.123456");
    let f = dt.format("YYYY-MM-DD/hh/mm/ss.000000000");
    assert_eq!(f, "2000-01-01/01/01/11.123456789");
}

#[test]
fn test_format2() {
    let dt = fastdate::DateTime::from((
        Date {
            day: 1,
            mon: 1,
            year: 2000,
        },
        Time {
            nano: 123456000,
            sec: 11,
            minute: 1,
            hour: 1,
        },
    ))
    .set_offset(8 * 60 * 60);
    println!("dt={}", dt);
    let f = dt.format("YYYY-MM-DD/hh/mm/ss.000000/+00:00");
    assert_eq!(f, "2000-01-01/09/01/11.123456/+08:00");
}

#[test]
fn test_format3() {
    let dt = fastdate::DateTime::from((
        Date {
            day: 1,
            mon: 1,
            year: 2000,
        },
        Time {
            nano: 123456000,
            sec: 11,
            minute: 1,
            hour: 1,
        },
    ))
    .set_offset(-8 * 60 * 60);
    println!("dt={}", dt);
    let f = dt.format("YYYY-MM-DD/hh/mm/ss.000000/+00:00");
    assert_eq!(f, "1999-12-31/17/01/11.123456/-08:00");
}

#[test]
fn test_offset_sec_max() {
    let mut dt = fastdate::DateTime::from((
        Date {
            day: 1,
            mon: 1,
            year: 2000,
        },
        Time {
            nano: 123456000,
            sec: 11,
            minute: 1,
            hour: 1,
        },
    ));
    dt = dt.set_offset(86399);
    assert_eq!(dt.offset(), 86399);
}

#[test]
fn test_offset_sec_min() {
    let mut dt = fastdate::DateTime::from((
        Date {
            day: 1,
            mon: 1,
            year: 2000,
        },
        Time {
            nano: 123456000,
            sec: 11,
            minute: 1,
            hour: 1,
        },
    ));
    dt = dt.set_offset(-86399);
    assert_eq!(dt.offset(), -86399);
}

#[test]
fn test_get_nano() {
    let dt = fastdate::DateTime::from((
        Date {
            day: 1,
            mon: 1,
            year: 2000,
        },
        Time {
            nano: 123456000,
            sec: 11,
            minute: 1,
            hour: 1,
        },
    ));
    assert_eq!(dt.nano(), 123456000);
}

#[test]
fn test_get_ms() {
    let dt = fastdate::DateTime::from((
        Date {
            day: 1,
            mon: 1,
            year: 2000,
        },
        Time {
            nano: 123000000,
            sec: 11,
            minute: 1,
            hour: 1,
        },
    ));
    assert_eq!(dt.ms(), 123u16);
}

#[test]
fn test_get_micro() {
    let dt = fastdate::DateTime::from((
        Date {
            day: 1,
            mon: 1,
            year: 2000,
        },
        Time {
            nano: 123000000,
            sec: 11,
            minute: 1,
            hour: 1,
        },
    ));
    assert_eq!(dt.micro(), 123000);
}

#[test]
fn test_get_sec() {
    let dt = fastdate::DateTime::from((
        Date {
            day: 1,
            mon: 1,
            year: 2000,
        },
        Time {
            nano: 123000000,
            sec: 11,
            minute: 1,
            hour: 1,
        },
    ));
    assert_eq!(dt.sec(), 11);
}

#[test]
fn test_get_minute() {
    let dt = fastdate::DateTime::from((
        Date {
            day: 1,
            mon: 1,
            year: 2000,
        },
        Time {
            nano: 123000000,
            sec: 11,
            minute: 1,
            hour: 1,
        },
    ));
    assert_eq!(dt.minute(), 1);
}

#[test]
fn test_get_hour() {
    let dt = fastdate::DateTime::from((
        Date {
            day: 1,
            mon: 1,
            year: 2000,
        },
        Time {
            nano: 123000000,
            sec: 11,
            minute: 1,
            hour: 1,
        },
    ));
    assert_eq!(dt.hour(), 1);
}

#[test]
fn test_get_day() {
    let dt = fastdate::DateTime::from((
        Date {
            day: 1,
            mon: 1,
            year: 2000,
        },
        Time {
            nano: 123000000,
            sec: 11,
            minute: 1,
            hour: 1,
        },
    ));
    assert_eq!(dt.day(), 1);
}

#[test]
fn test_get_mon() {
    let dt = fastdate::DateTime::from((
        Date {
            day: 1,
            mon: 1,
            year: 2000,
        },
        Time {
            nano: 123000000,
            sec: 11,
            minute: 1,
            hour: 1,
        },
    ));
    assert_eq!(dt.mon(), 1);
}

#[test]
fn test_get_year() {
    let dt = fastdate::DateTime::from((
        Date {
            day: 1,
            mon: 1,
            year: 2000,
        },
        Time {
            nano: 123000000,
            sec: 11,
            minute: 1,
            hour: 1,
        },
    ));
    assert_eq!(dt.year(), 2000);
}

#[test]
fn test_get_week() {
    let dt = fastdate::DateTime::from((
        Date {
            day: 1,
            mon: 1,
            year: 2000,
        },
        Time {
            nano: 123000000,
            sec: 11,
            minute: 1,
            hour: 1,
        },
    ));
    assert_eq!(dt.week_day(), 6u8);
}

#[test]
fn test_from_system_time() {
    let _: DateTime = DateTime::from_system_time(SystemTime::now(), 0);
}
