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
use std::env;
use std::process;

use apriori_pattern_miner::report_frequent_itemsets;

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
