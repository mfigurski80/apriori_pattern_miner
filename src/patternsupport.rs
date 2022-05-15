// THIS CODE IS MY OWN WORK, IT WAS WRITTEN
// WITHOUT CONSULTING CODE WRITTEN BY OTHER STUDENTS ~ Mikolaj Figurski

use crate::intersect;
use crate::read;

use hashbrown::HashMap;
use intersect::IntersectSorted;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

/// Keyword List that stores string at their encodings. Shortens string pointers to u32
pub type Keywords = Vec<String>;
type KeywordIds = Vec<u32>;
type SupportingRowIds = Vec<u32>;
/// Maps specific pattern to the column ids that support it
pub type PatternSupport = HashMap<KeywordIds, SupportingRowIds>;

pub trait RecursivelyBuildable {
    fn build_next(
        &self,
        keyword_list: &Keywords,
        k1_support: &PatternSupport,
        min_support: usize,
    ) -> PatternSupport;

    fn better_build_next(
        &mut self,
        keyword_list: &Keywords,
        k1_support: &PatternSupport,
        min_support: usize,
    ) -> PatternSupport;
}

impl RecursivelyBuildable for PatternSupport {
    fn build_next(
        &self,
        keyword_list: &Keywords,
        k1_support: &PatternSupport,
        min_support: usize,
    ) -> PatternSupport {
        // next PatternSupport
        let k_support = Arc::new(Mutex::new(PatternSupport::new())); // TODO: spend some time trying to predict size
                                                                     // let k_support = PatternSupport::new();
        self.par_iter().for_each(|(prev_pat, prev_sup)| {
            // we have a new pattern to build off of! From last elem value in
            // this one up to keyword_list.len(), check to see what the intersection
            // of support is (using k1_support) and, if above min, add new pattern
            // to k_support!
            // println!("{} -> {}", prev_pat.last().unwrap(), keyword_list.len());
            let last = *prev_pat.last().unwrap();
            for i in last + 1..keyword_list.len() as u32 {
                // for every keyword id after last
                let mut new_pat = prev_pat.clone();
                new_pat.push(i);
                let i_support = k1_support.get(&vec![i]).unwrap();
                let mut new_sup = prev_sup.clone(); // probably slow
                new_sup.intersect(&i_support); // custom sorted intersection
                if new_sup.len() >= min_support {
                    // println!("{} -> {} :: {}", last, keyword_list.len(), i);
                    k_support.as_ref().lock().unwrap().insert(new_pat, new_sup);
                    // k_support.insert(new_pat, new_sup);
                    // let v = k_support.get_key_value_mut(new_pat);
                }
            }
        });
        return Arc::try_unwrap(k_support)
            .expect("Failed to de-reference after multi-threading")
            .into_inner()
            .expect("Failed to unwrap after multi-threading");
    }
    fn better_build_next(
        &mut self,
        keyword_list: &Keywords,
        k1_support: &PatternSupport,
        min_support: usize,
    ) -> PatternSupport {
        PatternSupport::from_par_iter(
            self.par_iter()
                .map(|(prev_pat, prev_sup)| {
                    let last = *prev_pat.last().unwrap();
                    (last + 1..keyword_list.len() as u32)
                        .map(|i| {
                            let mut new_pat = prev_pat.clone();
                            new_pat.push(i);
                            let i_support = k1_support.get(&vec![i]).unwrap();
                            let mut new_sup = prev_sup.clone();
                            new_sup.intersect(&i_support);
                            (new_pat, new_sup)
                        })
                        .filter(|(_new_pat, new_sup)| new_sup.len() >= min_support)
                })
                .flatten_iter(),
        )
    }
}

/// Creates Keywords list and k=1 Pattern Support map from keyword support struct given
pub fn parse_keyword_support(base: read::KeywordSupport) -> (Keywords, PatternSupport) {
    // divide keyword support struct into keywords and pattern support
    let mut keywords = Keywords::new();
    let mut pattern = PatternSupport::new();

    for (key, sup) in base.iter() {
        let new_id = keywords.len();
        keywords.push(key.clone());
        pattern.insert(vec![new_id as u32], sup.to_vec());
    }
    (keywords, pattern)
}
