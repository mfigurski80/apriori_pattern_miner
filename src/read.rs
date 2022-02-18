extern crate csv;

use std::collections::HashMap;
use std::io;

pub type KeywordSupport = HashMap<String, Vec<u32>>;

pub fn read_keyword_support(
    filename: &str,
    min_support: usize,
) -> Result<KeywordSupport, io::Error> {
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
