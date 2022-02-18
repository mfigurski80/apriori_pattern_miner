pub trait IntersectSorted<T> {
    fn intersect(&mut self, other: &Self);
}

impl<T: PartialEq + PartialOrd> IntersectSorted<T> for Vec<T> {
    fn intersect(&mut self, other: &Self) {
        let (mut i, mut j) = (0, 0);
        while i < self.len() && j < other.len() {
            if self[i] == other[j] {
                self.remove(i);
                j += 1;
            } else if self[i] < other[j] {
                i += 1;
            } else {
                j += 1;
            }
        }
    }
}
