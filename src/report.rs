// THIS CODE IS MY OWN WORK, IT WAS WRITTEN
// WITHOUT CONSULTING CODE WRITTEN BY OTHER STUDENTS ~ Mikolaj Figurski

use crate::patternsupport;

use patternsupport::{Keywords, PatternSupport};
use std::fs::File;
use std::io::{BufWriter, Error, Write};

/// Stores report to be written to file (and hopefully easily sorted)
pub type PatternReport = Vec<(Vec<u32>, u32)>;

pub trait PatternAppendable {
    fn append_pattern(&mut self, pattern_support: &PatternSupport);
}

impl PatternAppendable for PatternReport {
    fn append_pattern(&mut self, pattern_support: &PatternSupport) {
        pattern_support.iter().for_each(|(pattern, sup)| {
            self.push((pattern.clone(), sup.len() as u32));
        });
    }
}

pub trait Reportable {
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
