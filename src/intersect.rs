pub trait IntersectSorted<T> {
    /// Intersect two sorted sequences
    fn intersect(&mut self, other: &Self);
}

impl<T: PartialEq + PartialOrd + Copy> IntersectSorted<T> for Vec<T> {
    fn intersect(&mut self, other: &Self) {
        let (mut i, mut j, mut cursor) = (0, 0, 0);
        while cursor + i < self.len() && j < other.len() {
            if self[cursor + i] == other[j] {
                self[cursor] = self[cursor + i];
                cursor += 1;
                j += 1;
                continue;
            }
            if self[cursor + i] > other[j] {
                // self.remove(i);
                j += 1;
            } else {
                i += 1;
            }
        }
        self.resize(cursor, self[0]);
    }
}

#[cfg(test)]
mod tests {
    use crate::intersect::IntersectSorted;
    #[test]
    fn vector_intersect_works() {
        let mut a = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let b = vec![2, 5, 7];
        a.intersect(&b);
        assert_eq!(a, vec![2, 5, 7]);
    }
}
