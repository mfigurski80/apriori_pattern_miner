extern crate csv;
#[macro_use]
extern crate serde_derive;

use std::env;
use std::error::Error;
use std::process;

#[derive(Debug, Deserialize)]
struct Record {
    text_keywords: String,
}

fn read_csv(filename: &str) -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(filename)?;
    for result in rdr.deserialize() {
        let record: Record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // print out all args after first arg
    if args.len() != 4 {
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

    if let Err(err) = read_csv(filename) {
        println!("error reading csv '{}': {}", filename, err);
        process::exit(1);
    }
}
