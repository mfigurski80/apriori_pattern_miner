extern crate csv;
#[macro_use]
extern crate serde_derive;

use std::error::Error;
use std::io;
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
    if let Err(err) = read_csv("data/data.csv") {
        println!("error reading csv {}", err);
        process::exit(1);
    }
}
