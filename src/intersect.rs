pub trait IntersectSorted<T> {
    /// Intersect two sorted sequences
    fn intersect(&mut self, other: &Self);
}

impl<T: PartialEq + PartialOrd> IntersectSorted<T> for Vec<T> {
    fn intersect(&mut self, other: &Self) {
        let (mut i, mut j) = (0, 0);
        while i < self.len() && j < other.len() {
            if self[i] == other[j] {
                i += 1;
                j += 1;
                continue;
            }
            if self[i] < other[j] {
                self.remove(i);
            } else {
                j += 1;
            }
        }
    }
}
