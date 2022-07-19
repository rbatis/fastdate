# fastdate
fastdate

this date cartes is very fast

```rust
//test bench_date_parse ... bench:          35 ns/iter (+/- 1)
#[bench]
fn bench_date_parse(b: &mut Bencher) {
    b.iter(|| {
        std::hint::black_box({
            DateTime::from_str("1234-12-13 11:12:13.123456");
        });
    });
}

//test bench_date_now   ... bench:          40 ns/iter (+/- 1)
#[bench]
fn bench_date_now(b: &mut Bencher) {
    b.iter(|| {
        std::hint::black_box({
            DateTime::now();
        });
    });
}

//test bench_date_now_local ... bench:          40 ns/iter (+/- 1)
#[bench]
fn bench_date_now_local(b: &mut Bencher) {
    b.iter(|| {
        std::hint::black_box({
            DateTime::now_local();
        });
    });
}
```