extern crate csv;
#[macro_use]
extern crate serde_derive;

// use std::collections::HashSet;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::process;

type SupportSet = HashMap<Vec<String>, u32>;

fn read_itemsets(filename: &str) -> Result<SupportSet, Box<dyn Error>> {
    #[derive(Deserialize)]
    struct Record {
        text_keywords: String,
    }
    let mut rdr = csv::Reader::from_path(filename)?;
    let mut itemsets = SupportSet::new();
    for result in rdr.deserialize() {
        let record: Record = result?;
        let mut keywords: Vec<String> = record
            .text_keywords
            .split(',')
            .map(|s| s.to_string())
            .collect();
        keywords.sort();
        // let key = keywords.clone();
        let count = itemsets.entry(keywords).or_insert(0);
        *count += 1;
    }
    Ok(itemsets)
}

fn find_frequent_itemsets(filename: &str, threshold: u32, output: &str) {
    let base_itemsets = match read_itemsets(filename) {
        Ok(itemsets) => itemsets,
        Err(err) => {
            println!("Error reading itemsets from '{}': {}", filename, err);
            process::exit(1);
        }
    };
    println!("{} unique itemsets read", base_itemsets.len());
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
    println!("All above {}: '{}' -> '{}' ", threshold, filename, output);

    find_frequent_itemsets(filename, threshold, output);
}
