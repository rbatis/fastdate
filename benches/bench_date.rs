#![feature(test)]
#![feature(bench_black_box)]
extern crate test;

use fastdate::DateTime;
use std::str::FromStr;
use test::Bencher;

#[bench]
fn bench_date_parse(b: &mut Bencher) {
    b.iter(|| {
        std::hint::black_box({
            DateTime::from_str("1234-12-13 11:12:13.123456").expect("TODO: panic message");
        });
    });
}
