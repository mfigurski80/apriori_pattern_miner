// THIS CODE IS MY OWN WORK, IT WAS WRITTEN
// WITHOUT CONSULTING CODE WRITTEN BY OTHER STUDENTS ~ Mikolaj Figurski

// COLLABORATION STATMENT: The following websites were significantly referenced to helping me write this code.
// [https://stackoverflow.com/questions/26643688/how-do-i-split-a-string-in-rust](https://stackoverflow.com/questions/26643688/how-do-i-split-a-string-in-rust)
// [https://doc.rust-lang.org/std/collections/struct.HashSet.html](https://doc.rust-lang.org/std/collections/struct.HashSet.html)
// [https://stackoverflow.com/questions/67594182/reading-csv-with-list-valued-columns-in-rust](https://stackoverflow.com/questions/67594182/reading-csv-with-list-valued-columns-in-rust)
// [https://stackoverflow.com/questions/26643688/how-do-i-split-a-string-in-rust](https://stackoverflow.com/questions/26643688/how-do-i-split-a-string-in-rust)
// [https://www.geeksforgeeks.org/apriori-algorithm/](https://www.geeksforgeeks.org/apriori-algorithm/)
// [https://stackoverflow.com/questions/64226562/check-if-vec-contains-all-elements-from-another-vec](https://stackoverflow.com/questions/64226562/check-if-vec-contains-all-elements-from-another-vec)
// [https://docs.rs/array_tool/1.0.3/array_tool/vec/index.html](https://docs.rs/array_tool/1.0.3/array_tool/vec/index.html)
// [https://crates.io/crates/rayon](https://crates.io/crates/rayon)
// [https://docs.rs/chashmap/latest/chashmap/struct.CHashMap.html](https://docs.rs/chashmap/latest/chashmap/struct.CHashMap.html)
// [https://blog.yoshuawuyts.com/optimizing-hashmaps-even-more/](https://blog.yoshuawuyts.com/optimizing-hashmaps-even-more/)
// [https://www.cyberciti.biz/faq/unix-linux-time-command-examples-usage-syntax/](https://www.cyberciti.biz/faq/unix-linux-time-command-examples-usage-syntax/)
// [https://stackoverflow.com/questions/66032586/how-do-i-use-hashbrown-data-types-with-rayon-parallel-iterators](https://stackoverflow.com/questions/66032586/how-do-i-use-hashbrown-data-types-with-rayon-parallel-iterators)
// [https://stackoverflow.com/questions/30559073/cannot-borrow-captured-outer-variable-in-an-fn-closure-as-mutable](https://stackoverflow.com/questions/30559073/cannot-borrow-captured-outer-variable-in-an-fn-closure-as-mutable)
// [https://stackoverflow.com/questions/29177449/how-to-take-ownership-of-t-from-arcmutext](https://stackoverflow.com/questions/29177449/how-to-take-ownership-of-t-from-arcmutext)

#[macro_use]
extern crate serde_derive;

mod intersect;
mod patternsupport;
mod read;
mod report;

use patternsupport::{parse_keyword_support, PatternSupport, RecursivelyBuildable};
use report::{PatternAppendable, PatternReport, Reportable};
use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::process;

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
        k_support = k_support.build_next(&keywords, &k1_support, threshold);
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
