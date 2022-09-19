#![feature(test)]
#![feature(bench_black_box)]
extern crate test;

use fastdate::DateTime;
use std::str::FromStr;
use std::time::Duration;
use test::Bencher;

//test bench_datetime_from_str     ... bench:          11 ns/iter (+/- 0)
#[bench]
fn bench_datetime_from_str(b: &mut Bencher) {
    b.iter(|| {
        std::hint::black_box({
            DateTime::from_str("1997-12-13 11:12:13.123456").expect("TODO: panic message");
        });
    });
}

//test bench_date_from_str_iso_8601   ... bench:          41 ns/iter (+/- 2)
#[bench]
fn bench_date_from_str_iso_8601(b: &mut Bencher) {
    b.iter(|| {
        std::hint::black_box({
            DateTime::from_str("1997-12-13 11:12:13.123456Z").expect("TODO: panic message");
        });
    });
}

//test bench_date_from_str_iso_8601_time   ... bench:          41 ns/iter (+/- 2)
#[bench]
fn bench_date_from_str_iso_8601_time(b: &mut Bencher) {
    b.iter(|| {
        std::hint::black_box({
            DateTime::from_str("1997-12-13 11:12:13.123456+09").expect("TODO: panic message");
        });
    });
}

//test bench_date_parse_format ... bench:          58 ns/iter (+/- 1)
#[bench]
fn bench_date_parse_format(b: &mut Bencher) {
    b.iter(|| {
        std::hint::black_box({
            DateTime::parse("YYYY-MM-DD hh:mm:ss.000000", "2022-12-13 11:12:14.123456")
                .expect("TODO: panic message");
        });
    });
}

//test bench_date_now   ... bench:          40 ns/iter (+/- 1)
#[bench]
fn bench_date_utc(b: &mut Bencher) {
    b.iter(|| {
        std::hint::black_box({
            DateTime::utc();
        });
    });
}

//test bench_date_now_local ... bench:          40 ns/iter (+/- 1)
#[bench]
fn bench_date_now_local(b: &mut Bencher) {
    b.iter(|| {
        std::hint::black_box({
            DateTime::now();
        });
    });
}

//test bench_date_display    ... bench:          40 ns/iter (+/- 1)
#[bench]
fn bench_date_display(b: &mut Bencher) {
    let now = DateTime::now();
    b.iter(|| {
        std::hint::black_box({
            now.to_string();
        });
    });
}

#[bench]
fn bench_add(b: &mut Bencher) {
    let now = DateTime::now();
    b.iter(|| {
        std::hint::black_box({
            let _ = now.clone() + Duration::from_secs(24 * 3600);
        });
    });
}

#[bench]
fn bench_eq(b: &mut Bencher) {
    let now = DateTime::now();
    let now2 = DateTime::now();
    b.iter(|| {
        std::hint::black_box({
            let _ = now.eq(&now2);
        });
    });
}

#[bench]
fn bench_set_offset(b: &mut Bencher) {
    let now = DateTime::utc();
    b.iter(|| {
        std::hint::black_box({
            let _ = now.clone().set_offset(8 * 3600);
        });
    });
}

#[bench]
fn bench_timestamp(b: &mut Bencher) {
    let now = DateTime::utc();
    b.iter(|| {
        std::hint::black_box({
            let _ = now.unix_timestamp();
        });
    });
}

#[bench]
fn bench_get_micro(b: &mut Bencher) {
    let now = DateTime::utc();
    b.iter(|| {
        std::hint::black_box({
            let _ = now.get_micro();
        });
    });
}
