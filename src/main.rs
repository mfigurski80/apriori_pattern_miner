extern crate csv;
#[macro_use]
extern crate serde_derive;

// use std::collections::HashSet;
use std::collections::HashMap;
use std::env;
use std::io;
use std::mem;
use std::process;

type KeywordSupport = HashMap<String, Vec<u32>>;
type PatternSupport = HashMap<Vec<u32>, Vec<u32>>;

fn read_k1_support(filename: &str, min_support: usize) -> Result<KeywordSupport, io::Error> {
    #[derive(Deserialize)]
    struct Record {
        text_keywords: String,
    }
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
    Ok(base)
}

// fn read_k_support(k1_support: &KeywordSupport)

// holds all the logic for the Apriori algorithm
fn find_frequent_itemsets(filename: &str, threshold: usize, output: &str) {
    let base_set = match read_k1_support(filename, threshold) {
        Ok(set) => set,
        Err(err) => {
            println!("Error reading base itemset from '{}': {}", filename, err);
            process::exit(1);
        }
    };
    println!("{:5} unique itemsets read", base_set.len());
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
