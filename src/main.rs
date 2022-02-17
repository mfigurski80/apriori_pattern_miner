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
            .split(';')
            .map(|s| s.to_string())
            .collect();
        keywords.sort();
        // let key = keywords.clone();
        let count = itemsets.entry(keywords).or_insert(0);
        *count += 1;
    }
    Ok(itemsets)
}

fn filter_itemsets(itemsets: &mut SupportSet, min_support: u32) {
    itemsets.retain(|_, count| *count >= min_support);
}

fn build_1_itemsets(itemsets: &SupportSet, min_support: u32) -> SupportSet {
    let mut one_itemsets = SupportSet::new();
    for (keywords, count) in itemsets {
        for keyword in keywords {
            let keyword_vec = vec![keyword.clone()];
            let word_count = one_itemsets.entry(keyword_vec).or_insert(0);
            *word_count += count;
        }
    }
    filter_itemsets(&mut one_itemsets, min_support);
    one_itemsets
}

fn build_k_itemset(
    prev_itemsets: &SupportSet,
    one_itemsets: &SupportSet,
    original_itemsets: &mut SupportSet,
    min_support: u32,
) -> SupportSet {
    let mut k_itemsets = SupportSet::new();
    for (prev, prev_count) in prev_itemsets {
        println!("prev: {:?}", prev);
        for (keyword, word_count) in one_itemsets {
            if prev.contains(&keyword[0]) {
                // skip if keyword is already in prev
                continue;
            }
            // generate new potential itemset
            let mut new_set = prev.clone();
            new_set.push(keyword[0].clone());
            new_set.sort();
            if k_itemsets.contains_key(&new_set) {
                // skip if already checked this combination
                continue;
            }
            for (original, original_count) in &mut *original_itemsets {
                // check if new_set is a subset of original
                let mut original_iter = original.iter();
                if new_set.iter().all(|x| original_iter.any(|o| x == o)) {
                    // if so, add to k_itemsets
                    let count = k_itemsets.entry(new_set.clone()).or_insert(0);
                    *count += *original_count;
                }
            }
        }
    }
    filter_itemsets(&mut k_itemsets, min_support);
    k_itemsets
}

// holds all the logic for the Apriori algorithm
fn find_frequent_itemsets(filename: &str, threshold: u32, output: &str) {
    let mut base_itemsets = match read_itemsets(filename) {
        Ok(itemsets) => itemsets,
        Err(err) => {
            println!("Error reading itemsets from '{}': {}", filename, err);
            process::exit(1);
        }
    };
    println!("{:5} unique itemsets read", base_itemsets.len());
    // println!("Base itemsets: {:?}", base_itemsets);
    let one_itemsets = build_1_itemsets(&base_itemsets, threshold);
    println!("{:5} 1-itemsets found", one_itemsets.len());
    let two_itemsets = build_k_itemset(&one_itemsets, &one_itemsets, &mut base_itemsets, threshold);
    println!("{:5} 2-itemsets found", two_itemsets.len());
    // for (keywords, count) in &one_itemsets {
    //     if *count >= threshold {
    //         println!("{:?} {}", keywords, count);
    //     }
    // }
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

    find_frequent_itemsets(filename, threshold, output);
}
