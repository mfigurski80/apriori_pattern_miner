#[macro_use]
extern crate serde_derive;

mod intersect;
mod patternsupport;
mod read;
mod report;

use patternsupport::{parse_keyword_support, PatternSupport, RecursivelyBuildable};
use report::{PatternAppendable, PatternReport, Reportable};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::process;

/// holds all the logic for the Apriori algorithm
pub fn report_frequent_itemsets(filename: &str, threshold: usize, output: &str) {
    let mut report = PatternReport::new();
    let (keywords, k1_support) = match read::read_keyword_support(filename, threshold) {
        Ok(set) => parse_keyword_support(set),
        Err(err) => {
            println!("Error reading base itemset from '{}': {}", filename, err);
            process::exit(1);
        }
    };
    println!("{:5} passing 1-itemsets found", keywords.len());
    report.append_pattern(&k1_support);

    let mut k_support: PatternSupport = k1_support.clone(); // TODO: find a better way to do this
    let mut k = 2;
    while k_support.len() > 0 {
        k_support = k_support.build_next(&k1_support, threshold);
        println!("{:5} passing {}-itemsets found", k_support.len(), k,);
        k += 1;
        report.append_pattern(&k_support);
    }

    // report.sort_by_key(|k| k.1);
    report.sort_by(|a, b| a.1.cmp(&b.1).reverse());
    let mut out = BufWriter::new(File::create(output).expect("Unable to create file"));
    report
        .write_report(&keywords, &mut out)
        .expect("Failed to write to file");
    out.flush().expect("Failed to flush buffer to file");
}
