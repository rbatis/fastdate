use fastdate::{Date, DateTime, DurationFrom, offset_sec, Time};
use std::str::FromStr;
use std::time::{Duration, SystemTime};

#[test]
fn test_other_space() {
    let d = DateTime::parse("YYYY-MM-DDTHH_MM_SS.000000", "1234_12_13_11_12_13.123456").unwrap();
    println!("{}", d);
    assert_eq!("1234-12-13T11:12:13.123456+08:00".to_string(), d.to_string());
}

#[test]
fn test_date() {
    let d = DateTime::from_str("1234-12-13 11:12:13.123456").unwrap();
    println!("{}", d);
    assert_eq!(d.to_string(), "1234-12-13T11:12:13.123456+08:00".to_string());
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
    assert_eq!(d.add(Duration::from_secs(1)), added);
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
fn test_timestamp_micros() {
    let now = DateTime::utc();
    let timestamp = now.unix_timestamp_nano();
    let new_time = DateTime::from_timestamp_nano(timestamp);
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
    let new_time = DateTime::from((Date {
        day: 12,
        mon: 12,
        year: 2023,
    }, Time {
        nano: 12,
        sec: 12,
        minute: 12,
        hour: 12,
    }));
    assert_eq!(new_time.to_string(), "2023-12-12T12:12:12.000000012Z");
}

#[test]
fn test_set_offset() {
    let new_time = DateTime::from((Date {
        day: 12,
        mon: 12,
        year: 2023,
    }, Time {
        nano: 12,
        sec: 12,
        minute: 12,
        hour: 12,
    }));
    let timestamp_nano = new_time.unix_timestamp_nano();
    assert_eq!(new_time.to_string(), "2023-12-12T12:12:12.000000012Z");
    let new_time = new_time.set_offset(offset_sec());
    assert_eq!(new_time.to_string(), "2023-12-12T20:12:12.000000012+08:00");
    assert_eq!(new_time.unix_timestamp_nano(), timestamp_nano);
}

#[test]
fn test_set_offset2() {
    let new_time = DateTime::from((Date {
        day: 12,
        mon: 12,
        year: 2023,
    }, Time {
        nano: 12,
        sec: 12,
        minute: 12,
        hour: 12,
    }, 8*3600));
    println!("{}", new_time.display_stand());
    assert_eq!(new_time.display(true), "2023-12-12T12:12:12.000000012+08:00");
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
    epoch = epoch.set_offset(8*3600).add_sub_sec(-8*3600);
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
    assert_eq!(date2.after(&date1), true);
    assert_eq!(date1.before(&date2), true);
}

#[test]
fn test_from_str_zone() {
    let date1 = DateTime::from_str("2022-12-12 00:00:00").unwrap();
    assert_eq!(date1.offset(), offset_sec());
    println!("{}", date1);
}

#[test]
fn test_parse_z() {
    let date = DateTime::from_str("2022-12-12 00:00:00.000000Z").unwrap();
    assert_eq!(date.to_string(), "2022-12-12T00:00:00Z");
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
fn test_set_offset_sub() {
    let mut date = DateTime::from_str("2022-12-12 09:00:00").unwrap();
    date = date.set_offset(-9 * 3600);
    assert_eq!(date.to_string(), "2022-12-11T16:00:00-09:00");
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
    let date = DateTime::parse("YYYY-MM-DD HH:MM:SS.000000", "2022-12-13 11:12:14.123456").unwrap();
    println!("{}", date);
}

#[test]
fn test_parse_format2() {
    let date = DateTime::parse("HH:MM:SS.000000,YYYY-MM-DD", "11:12:14.123456,2022-12-13").unwrap();
    println!("{}", date);
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
    let date1 = DateTime::from_str("2019-04-28 00:00:00.023333333").unwrap();
    println!("{}", date1.to_string());
    assert_eq!(date1.to_string(), "2019-04-28T00:00:00.023333333+08:00");
}


#[test]
fn test_parse_date() {
    let date = DateTime::from_str("2013-10-06").unwrap();
    assert_eq!(date.to_string(), "2013-10-06T00:00:00+08:00");
}

#[test]
fn test_ser_date() {
    let mut date = DateTime::from_str("2023-10-13 16:57:41.123926").unwrap();
    date = date.set_offset(offset_sec());
    let js = serde_json::to_string(&date).unwrap();
    assert_eq!(js, "\"2023-10-13T16:57:41.123926+08:00\"");
}

#[test]
fn test_de_date() {
    let mut date = DateTime::from_str("2023-10-13 16:57:41.123926").unwrap();
    date = date.set_offset(offset_sec());
    let js = serde_json::to_string(&date).unwrap();
    assert_eq!(js, "\"2023-10-13T16:57:41.123926+08:00\"");
    let new_date = serde_json::from_str::<DateTime>(&js).unwrap();
    assert_eq!(new_date.offset(), 28800);
}

#[test]
fn test_add_minute() {
    let date = DateTime::from_str("2013-10-06").unwrap().add(Duration::from_minute(1));
    assert_eq!(date.to_string(), "2013-10-06T00:01:00+08:00");
}

#[test]
fn test_add_hour() {
    let date = DateTime::from_str("2013-10-06").unwrap().add(Duration::from_hour(1));
    assert_eq!(date.to_string(), "2013-10-06T01:00:00+08:00");
}

#[test]
fn test_add_day() {
    let date = DateTime::from_str("2013-10-06").unwrap().add(Duration::from_day(1));
    assert_eq!(date.to_string(), "2013-10-07T00:00:00+08:00");
}

#[test]
fn test_add_sub_sec() {
    let date = DateTime::from_str("2013-10-06").unwrap().add_sub_sec(1);
    assert_eq!(date.to_string(), "2013-10-06T00:00:01+08:00");
    let date = DateTime::from_str("2013-10-06").unwrap().add_sub_sec(-1);
    assert_eq!(date.to_string(), "2013-10-05T23:59:59+08:00");
}

#[test]
fn test_1958_unix() {
    let date = DateTime::from_str("1958-01-01T00:00:00Z").unwrap();
    println!("s={:?},date={}", SystemTime::from(date.clone()), DateTime::from_system_time(SystemTime::from(date.clone()), 0));
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
    println!("{}", dt.to_string());
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
    let v = epoch + Duration::from_micros(us as u64);
    println!("{}", v);//2023-02-14 07:37:40
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
    let epoch = fastdate::DateTime::from((Date {
        day: 1,
        mon: 1,
        year: 2000,
    }, Time {
        nano: 1233,
        sec: 11,
        minute: 1,
        hour: 1,
    }));
    let v = epoch.display(false);
    assert_eq!(v, "2000-01-01T01:01:11.000001233");
}

#[test]
fn test_display_stand() {
    let epoch = fastdate::DateTime::from((Date {
        day: 1,
        mon: 1,
        year: 2000,
    }, Time {
        nano: 1233,
        sec: 11,
        minute: 1,
        hour: 1,
    }));
    let v = epoch.display_stand();
    assert_eq!(v, "2000-01-01 01:01:11.000001233");
}

#[test]
fn test_set_micro() {
    let mut dt = fastdate::DateTime::from((Date {
        day: 1,
        mon: 1,
        year: 2000,
    }, Time {
        nano: 1233,
        sec: 11,
        minute: 1,
        hour: 1,
    }));
    dt = dt.set_nano(0);
    assert_eq!(dt.display_stand(), "2000-01-01 01:01:11");

    dt = dt.set_nano(1);
    assert_eq!(dt.display_stand(), "2000-01-01 01:01:11.000001");
}
