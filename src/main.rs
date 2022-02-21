#[macro_use]
extern crate serde_derive;

mod intersect;
mod read;

use intersect::IntersectSorted;
// use std::collections::HashMap;
use hashbrown::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufWriter, Error, Write};
use std::process;

/// Keyword List that stores string at their encodings. Shortens string pointers to u32
type Keywords = Vec<String>;
/// Maps specific pattern to the column ids that support it
type PatternSupport = HashMap<Vec<u32>, Vec<u32>>;
/// Stores report to be written to file (and hopefully easily sorted)
type PatternReport = Vec<(Vec<u32>, u32)>;

trait PatternAppendable {
    fn append_pattern(&mut self, pattern_support: &PatternSupport);
}

impl PatternAppendable for PatternReport {
    fn append_pattern(&mut self, pattern_support: &PatternSupport) {
        pattern_support.iter().for_each(|(pattern, sup)| {
            self.push((pattern.clone(), sup.len() as u32));
        });
    }
}

trait Reportable {
    fn write_report(&self, words: &Keywords, output: &mut BufWriter<File>) -> Result<(), Error>;
}

impl Reportable for PatternReport {
    fn write_report(&self, words: &Keywords, output: &mut BufWriter<File>) -> Result<(), Error> {
        self.iter()
            .try_for_each(|(pattern, sup)| -> Result<(), Error> {
                let pat_str: Vec<&str> = pattern
                    .iter()
                    .map(|&id| words[id as usize].as_str())
                    .collect();
                writeln!(output, "{} ({})", pat_str.join(" "), sup)?;
                Ok(())
            })?;
        Ok(())
    }
}

/// Creates Keywords list and k=1 Pattern Support map from keywrod support struct given
fn parse_keyword_support(base: read::KeywordSupport) -> (Keywords, PatternSupport) {
    // divide keyword support struct into keywords and pattern support
    let mut keywords = Keywords::new();
    let mut pattern = PatternSupport::new();

    for (key, sup) in base.iter() {
        let new_id = keywords.len();
        keywords.push(key.clone());
        pattern.insert(vec![new_id as u32], sup.to_vec());
    }
    (keywords, pattern)
}

/// From given k-1 pattern support map (and a few constant variables), generate k pattern support
fn read_k_support(
    keyword_list: &Keywords,
    k1_support: &PatternSupport, // TODO: remove from here once entry fails to contribute to any k pattern
    prev_patterns_support: &PatternSupport,
    min_support: usize,
) -> PatternSupport {
    let mut k_support = PatternSupport::new(); // TODO: spend some time trying to predict size
    prev_patterns_support
        .iter()
        .for_each(|(prev_pat, prev_sup)| {
            // we have a new pattern to build off of! From last elem value in
            // this one up to keyword_list.len(), check to see what the intersection
            // of support is (using k1_support) and, if above min, add new pattern to k_support!
            // println!("{} -> {}", prev_pat.last().unwrap(), keyword_list.len());
            let last = *prev_pat.last().unwrap();
            for i in last + 1..keyword_list.len() as u32 {
                let mut new_pat = prev_pat.clone();
                new_pat.push(i);
                let i_support = k1_support.get(&vec![i]).unwrap();
                let mut new_sup = prev_sup.clone(); // probably slow
                new_sup.intersect(&i_support); // custom sorted intersection
                if new_sup.len() >= min_support {
                    // println!("{} -> {} :: {}", last, keyword_list.len(), i);
                    k_support.insert(new_pat, new_sup);
                }
            }
        });
    k_support
}

/// holds all the logic for the Apriori algorithm
fn report_frequent_itemsets(filename: &str, threshold: usize, output: &str) {
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
        k_support = read_k_support(&keywords, &k1_support, &k_support, threshold);
        println!("{:5} passing {}-itemsets found", k_support.len(), k,);
        // println!("\tEx: {:?}", k_support.get(&vec![0, 1]).unwrap());
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

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        // ensure 3 args
        println!(
            "Usage: {} <csv_file> <support_threshold> <output_file>",
            args[0]
        );
        process::exit(1);
    }
    let filename = &args[1];
    let threshold: u32 = args[2].parse().unwrap();
    let output = &args[3];
    // println!("All above {}: '{}' -> '{}' ", threshold, filename, output);

    report_frequent_itemsets(filename, threshold as usize, output);
}
