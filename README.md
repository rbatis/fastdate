# fastdate
fastdate

this date cartes is very fast(including parse(),now(),now_local(),display <= 50ns)

```log
test bench_date_display   ... bench:          45 ns/iter (+/- 2)
test bench_date_now_local ... bench:          40 ns/iter (+/- 0)
test bench_date_parse     ... bench:          11 ns/iter (+/- 0)
test bench_date_utc       ... bench:          39 ns/iter (+/- 0)
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
    //from str
    let datetime=DateTime::from_str("1234-12-13 11:12:13.123456");
    //to_string()
    let s = datetime.to_string();//1234-12-13 11:12:13.123456
}
```