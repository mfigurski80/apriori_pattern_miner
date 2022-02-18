extern crate csv;
#[macro_use]
extern crate serde_derive;

// use std::collections::HashSet;
use std::collections::HashMap;
use std::env;
use std::io;
use std::mem;
use std::process;

type Keywords = Vec<String>;
/// Maps specific pattern to the column ids that support it
type PatternSupport = HashMap<Vec<u32>, Vec<u32>>;

/// Creates Keywords list and k=1 Pattern Support map from file given
fn read_keyword_support(
    filename: &str,
    min_support: usize,
) -> Result<(Keywords, PatternSupport), io::Error> {
    #[derive(Deserialize)]
    struct Record {
        text_keywords: String,
    }
    type KeywordSupport = HashMap<String, Vec<u32>>;

    let mut rdr = csv::Reader::from_path(filename)?;
    let mut base = KeywordSupport::new();
    let mut i: u32 = 0;
    for result in rdr.deserialize() {
        let record: Record = result?;
        for keyword in record.text_keywords.split(';') {
            // insert keyword into base
            base.entry(keyword.to_string()).or_insert(vec![]).push(i);
        }
        i += 1;
    }
    // clear items below threshold support
    base.retain(|_, v| v.len() >= min_support);

    // divide keyword support struct into keywords and pattern support
    let mut keywords = Keywords::new();
    let mut pattern = PatternSupport::new();

    for (key, sup) in base.iter() {
        let new_id = keywords.len();
        keywords.push(key.clone());
        pattern.insert(vec![new_id as u32], sup.to_vec());
    }
    Ok((keywords, pattern))
}

/// From given k-1 pattern support map (and a few global variables), generate k pattern support
fn read_k_support(
    keyword_list: &Keywords,
    k1_support: &PatternSupport,
    prev_patterns_support: &PatternSupport,
    min_support: usize,
) -> PatternSupport {
    let mut k_support = PatternSupport::new();
    prev_patterns_support
        .iter()
        .for_each(|(prev_pat, prev_sup)| {
            // we have a new pattern to build off of! From last elem value in
            // this one up to keyword_list.len(), check to see what the intersection
            // of support is (using k1_support) and, if above min, add new pattern to k_support!
            println!("{}->{}", prev_pat.last().unwrap(), keyword_list.len());
            // iterate from
        });
    k_support.retain(|_, v| v.len() >= min_support);
    k_support
}

// holds all the logic for the Apriori algorithm
fn find_frequent_itemsets(filename: &str, threshold: usize, output: &str) {
    let (keywords, k1_support) = match read_keyword_support(filename, threshold) {
        Ok(set) => set,
        Err(err) => {
            println!("Error reading base itemset from '{}': {}", filename, err);
            process::exit(1);
        }
    };
    println!("{:5} passing 1-itemsets found", keywords.len());
    let k2_support = read_k_support(&keywords, &k1_support, &k1_support, threshold);
    println!("{:5} passing 2-itemsets found", k2_support.len());
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

    find_frequent_itemsets(filename, threshold as usize, output);
}
