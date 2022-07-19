#![feature(test)]
#![feature(bench_black_box)]
extern crate test;

use fastdate::DateTime;
use std::str::FromStr;
use std::time::Duration;
use test::Bencher;

//test bench_date_parse ... bench:          35 ns/iter (+/- 1)
#[bench]
fn bench_date_parse(b: &mut Bencher) {
    b.iter(|| {
        std::hint::black_box({
            DateTime::from_str("1234-12-13 11:12:13.123456").expect("TODO: panic message");
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
            format!("{}", now);
        });
    });
}

#[bench]
fn bench_add(b: &mut Bencher) {
    let now = DateTime::now();
    b.iter(|| {
        std::hint::black_box({
            let _= now + Duration::from_secs(24 * 3600);
        });
    });
}

#[bench]
fn bench_eq(b: &mut Bencher) {
    let now = DateTime::now();
    let now2 = DateTime::now();
    b.iter(|| {
        std::hint::black_box({
            let _=now.eq(&now2);
        });
    });
}

#[bench]
fn bench_set_offset(b: &mut Bencher) {
    let now = DateTime::utc();
    b.iter(|| {
        std::hint::black_box({
            let _=now.set_offset(8*3600);
        });
    });
}