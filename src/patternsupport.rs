// THIS CODE IS MY OWN WORK, IT WAS WRITTEN
// WITHOUT CONSULTING CODE WRITTEN BY OTHER STUDENTS ~ Mikolaj Figurski

use crate::intersect;
use crate::read;

use hashbrown::HashMap;
use intersect::IntersectSorted;
use rayon::prelude::*;

/// Keyword List that stores string at their encodings. Shortens string pointers to u32
pub type Keywords = Vec<String>;
type KeywordIds = Vec<u32>;
type SupportingRowIds = Vec<u32>;
/// Maps specific pattern to the column ids that support it
pub type PatternSupport = HashMap<KeywordIds, SupportingRowIds>;

pub trait RecursivelyBuildable {
    fn build_next(&self, k1_support: &PatternSupport, min_support: usize) -> PatternSupport;
}

impl RecursivelyBuildable for PatternSupport {
    fn build_next(&self, k1_support: &PatternSupport, min_support: usize) -> PatternSupport {
        PatternSupport::from_par_iter(
            self.par_iter()
                .map(|(prev_pat, prev_sup)| {
                    let last = *prev_pat.last().unwrap();
                    (last + 1..k1_support.len() as u32)
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

#[cfg(test)]
mod tests {
    use crate::patternsupport::*;

    #[test]
    fn it_correctly_builds_next() {
        let mut k1 = PatternSupport::new();
        k1.insert_unique_unchecked(vec![0], vec![0, 1, 2, 3, 4, 5]);
        k1.insert_unique_unchecked(vec![1], vec![1, 2, 3, 4, 5]);
        k1.insert_unique_unchecked(vec![2], vec![2, 3, 4, 5]);
        k1.insert_unique_unchecked(vec![3], vec![3, 4, 5]);
        let min_sup = 3;
        let k2 = k1.build_next(&k1, min_sup);
        assert_ne!(k1, k2);
        assert_eq!(k2.len(), 6);
        for (pat, sup) in k2.into_iter() {
            assert_eq!(pat.len(), 2);
            assert!(sup.len() >= min_sup);
        }
    }
}
