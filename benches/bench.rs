#![feature(test)]

extern crate test;
use test::Bencher;
extern crate mining_hw2;

#[bench]
fn bench_smol(b: &mut Bencher) {
    b.iter(|| vec![1, 2, 3, 4].clone())
}
