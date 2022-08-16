# fastdate
fastdate of Any RFC3339Micro

this date cartes is very fast(<= 50ns) including 
* from_str("2022-12-13 11:12:14.123456")
* now()
* utc()
* week_day()
* to_string()/format!()
* eq()/==
* add()/sub()
* set_offset()
* parse("hh:mm:ss.000000,YYYY-MM-DD","11:12:14.123456,2022-12-13")
* unix_timestamp()
* unix_timestamp_millis()
* unix_timestamp_nano()
* from_timestamp()
* from_timestamp_millis()
* from_timestamp_nano()
* before(&date) -> bool
* after(&date1) -> bool
* from(v: SystemTime)
* from(v: DateTime)
* from(arg: Date)
* from(arg: Time)
* cmp(&self, other: &DateTime)/>/</>=/<= and more....

```log
test bench_add                         ... bench:          22 ns/iter (+/- 4)
test bench_date_display                ... bench:          40 ns/iter (+/- 1)
test bench_date_from_str_iso_8601      ... bench:          42 ns/iter (+/- 1)
test bench_date_from_str_iso_8601_time ... bench:          42 ns/iter (+/- 1)
test bench_date_now_local              ... bench:          40 ns/iter (+/- 1)
test bench_date_parse_format           ... bench:          61 ns/iter (+/- 4)
test bench_date_utc                    ... bench:          40 ns/iter (+/- 1)
test bench_datetime_from_str           ... bench:          13 ns/iter (+/- 0)
test bench_eq                          ... bench:           0 ns/iter (+/- 0)
test bench_set_offset                  ... bench:          25 ns/iter (+/- 4)
test bench_timestamp                   ... bench:           8 ns/iter (+/- 0)
```

* it can from_str for any RFC3339
for example:
```log
1234_12_13_11_12_13.123456
1234-12-13T11:12:13
1234-12-13 11:12:13.123456
1234-12-13T11:12:13.123456
```

* how use?
```rust
use fastdate::DateTime;
fn main(){
    //now with local time zone
    DateTime::now();
    //utc time now
    DateTime::utc();
    // add
    DateTime::now() + Duration::from_secs(1);
    // sub
    DateTime::now() - Duration::from_secs(1);
    //befor,after
    let date1 = DateTime::from_str("2022-12-12 00:00:00").unwrap();
    let date2 = DateTime::from_str("2022-12-12 01:00:00").unwrap();
    assert_eq!(date2.after(&date1), true);
    assert_eq!(date1.before(&date2), true);
    //from str
    let datetime=DateTime::from_str("1234-12-13 11:12:13.123456");
    //from str time zone
    let datetime=DateTime::from_str("1234-12-13 11:12:13.123456+08:00");
    let datetime=DateTime::from_str("1234-12-13 11:12:13.123456Z");
    //parse by format str
    let date = DateTime::parse("hh:mm:ss.000000,YYYY-MM-DD","11:12:14.123456,2022-12-13").unwrap();
    //to_string()
    let s = datetime.to_string();//1234-12-13 11:12:13.123456
    //unix_timestamp
    let timestamp = DateTime::now().unix_timestamp();
    //from unix_timestamp
    let datetime = DateTime::from_timestamp(timestamp);
    //unix_timestamp_millis
    let timestamp = DateTime::now().unix_timestamp_millis();
    //from unix millis
    let datetime = DateTime::from_timestamp_millis(timestamp);
    //unix_timestamp_nano
    let timestamp = DateTime::now().unix_timestamp_nano();
    //from unix_timestamp_nano
    let datetime = DateTime::from_timestamp_nano(timestamp);
}
```
