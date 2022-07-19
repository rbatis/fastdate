#![feature(test)]
#![feature(bench_black_box)]
extern crate test;

use fastdate::DateTime;
use std::str::FromStr;
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

//test bench_date_display    ... bench:          40 ns/iter (+/- 1)
#[bench]
fn bench_date_display(b: &mut Bencher) {
    let now=   DateTime::now_local();
    b.iter(|| {
        std::hint::black_box({
            now.to_string();
        });
    });
}