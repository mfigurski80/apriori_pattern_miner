#![feature(test)]

extern crate test;
use mining_hw2::report_frequent_itemsets;
use test::Bencher;

#[bench]
fn bench_smol(b: &mut Bencher) {
    b.iter(|| report_frequent_itemsets("data/smol.csv", 80, "/dev/null"))
    // report_frequent_itemsets("data/smol.csv", 40, "out.csv");
    // b.iter(|| vec![1, 2, 3].clone())
}

#[bench]
fn bench_chungus(b: &mut Bencher) {
    b.iter(|| report_frequent_itemsets("data/smol.csv", 4, "/dev/null"))
}
