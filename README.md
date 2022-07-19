# fastdate
fastdate

this date cartes is very fast(including parse(),now(),now_local(),display <= 50ns)

```log
test bench_date_display   ... bench:          40 ns/iter (+/- 3)
test bench_date_now       ... bench:          39 ns/iter (+/- 1)
test bench_date_now_local ... bench:          40 ns/iter (+/- 0)
test bench_date_parse     ... bench:          36 ns/iter (+/- 1)
```

* how use?
```rust
use fastdate::DateTime;
fn main(){
    //now
    DateTime::now();
    //local now with local time zone
    DateTime::now_local();
    //from str
    let datetime=DateTime::from_str("1234-12-13 11:12:13.123456");
}
```