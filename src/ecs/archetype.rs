type Segment = usize;

pub struct Archetype {
    count: usize,
    segments: Vec<Segment>,
}

impl Archetype {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            count: 0,
            segments: Vec::new(),
        }
    }

    pub fn insert(&mut self, id: usize) {
        if self.count <= id {
            self.count = id + 1;
            self.segments
                .resize(self.count.div_ceil(Segment::BITS as usize), 0);
        }
        self.segments[id / Segment::BITS as usize] |=
            1 << (id % Segment::BITS as usize);
    }

    #[must_use]
    pub fn has(&self, id: usize) -> bool {
        id < self.count
            && self.segments[id / Segment::BITS as usize]
                & 1 << (id % Segment::BITS as usize)
                != 0
    }

    #[must_use]
    pub fn has_common_with(&self, other: &Self) -> bool {
        self.segments
            .iter()
            .zip(&other.segments)
            .any(|(s, o)| s & o != 0)
    }

    #[must_use]
    pub fn is_subset_of(&self, other: &Self) -> bool {
        self.count <= other.count
            && self
                .segments
                .iter()
                .zip(&other.segments)
                .all(|(s, o)| s & o == *s)
    }

    #[must_use]
    pub fn is_superset_of(&self, other: &Self) -> bool {
        other.count <= self.count
            && other
                .segments
                .iter()
                .zip(&self.segments)
                .all(|(o, s)| o & s == *o)
    }

    pub fn remove(&mut self, id: usize) {
        if id < self.count {
            self.segments[id / Segment::BITS as usize] &=
                !(1 << (id % Segment::BITS as usize));
        }
    }

    pub fn clear(&mut self) {
        self.segments.fill(0);
    }
}

impl PartialEq for Archetype {
    fn eq(&self, other: &Self) -> bool {
        let len = self.segments.len().min(other.segments.len());
        self.segments
            .iter()
            .take(len)
            .eq(other.segments.iter().take(len))
            && self.segments.iter().skip(len).all(|s| *s == 0)
            && other.segments.iter().skip(len).all(|s| *s == 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_has_remove() {
        const N: usize = 100;
        let mut archetype = Archetype::new();
        for i in 0..N {
            assert!(!archetype.has(i));
            archetype.insert(i);
            assert!(archetype.has(i));
            archetype.remove(i);
            assert!(!archetype.has(i));
        }
        let mut archetype = Archetype::new();
        for i in (0..N).rev() {
            assert!(!archetype.has(i));
            archetype.insert(i);
            assert!(archetype.has(i));
            archetype.remove(i);
            assert!(!archetype.has(i));
        }
        let mut archetype = Archetype::new();
        archetype.remove(0);
    }

    #[test]
    fn has_common_with() {
        let mut archetype1 = Archetype::new();
        archetype1.insert(0);
        let mut archetype2 = Archetype::new();
        archetype2.insert(2);
        assert!(!archetype1.has_common_with(&archetype2));
        assert!(!archetype2.has_common_with(&archetype1));
        archetype1.insert(1);
        archetype2.insert(1);
        assert!(archetype1.has_common_with(&archetype2));
        assert!(archetype2.has_common_with(&archetype1));
    }

    #[test]
    fn is_subset_of() {
        let mut archetype1 = Archetype::new();
        archetype1.insert(0);
        archetype1.insert(1);
        archetype1.insert(2);
        let mut archetype2 = Archetype::new();
        archetype2.insert(0);
        archetype2.insert(1);
        archetype2.insert(2);
        assert!(archetype1.is_subset_of(&archetype2));
        assert!(archetype2.is_subset_of(&archetype1));
        let mut archetype1 = Archetype::new();
        archetype1.insert(0);
        archetype1.insert(2);
        let mut archetype2 = Archetype::new();
        archetype2.insert(0);
        archetype2.insert(1);
        archetype2.insert(2);
        assert!(archetype1.is_subset_of(&archetype2));
        assert!(!archetype2.is_subset_of(&archetype1));
        let mut archetype1 = Archetype::new();
        archetype1.insert(0);
        archetype1.insert(1);
        archetype1.insert(2);
        let mut archetype2 = Archetype::new();
        archetype2.insert(0);
        archetype2.insert(2);
        assert!(!archetype1.is_subset_of(&archetype2));
        assert!(archetype2.is_subset_of(&archetype1));
    }

    #[test]
    fn is_superset_of() {
        let mut archetype1 = Archetype::new();
        archetype1.insert(0);
        archetype1.insert(1);
        archetype1.insert(2);
        let mut archetype2 = Archetype::new();
        archetype2.insert(0);
        archetype2.insert(1);
        archetype2.insert(2);
        assert!(archetype1.is_superset_of(&archetype2));
        assert!(archetype2.is_superset_of(&archetype1));
        let mut archetype1 = Archetype::new();
        archetype1.insert(0);
        archetype1.insert(1);
        archetype1.insert(2);
        let mut archetype2 = Archetype::new();
        archetype2.insert(0);
        archetype2.insert(2);
        assert!(archetype1.is_superset_of(&archetype2));
        assert!(!archetype2.is_superset_of(&archetype1));
        let mut archetype1 = Archetype::new();
        archetype1.insert(0);
        archetype1.insert(2);
        let mut archetype2 = Archetype::new();
        archetype2.insert(0);
        archetype2.insert(1);
        archetype2.insert(2);
        assert!(!archetype1.is_superset_of(&archetype2));
        assert!(archetype2.is_superset_of(&archetype1));
    }

    #[test]
    fn reset() {
        const N: usize = 100;
        let mut archetype = Archetype::new();
        for i in 0..N {
            archetype.insert(i);
            assert!(archetype.has(i));
        }
        archetype.clear();
        for i in 0..N {
            assert!(!archetype.has(i));
        }
    }

    #[test]
    fn eq() {
        let mut archetype1 = Archetype::new();
        archetype1.insert(0);
        archetype1.insert(1);
        archetype1.insert(2);
        let mut archetype2 = Archetype::new();
        archetype2.insert(0);
        archetype2.insert(2);
        assert!(archetype1 != archetype2);
        assert!(archetype2 != archetype1);
        archetype2.insert(1);
        assert!(archetype1 == archetype2);
        assert!(archetype2 == archetype1);
        let mut archetype1 = Archetype::new();
        archetype1.insert(0);
        let mut archetype2 = Archetype::new();
        archetype2.insert(0);
        archetype2.insert(Segment::BITS as usize);
        assert!(archetype1 != archetype2);
        assert!(archetype2 != archetype1);
        archetype2.remove(Segment::BITS as usize);
        assert!(archetype1 == archetype2);
        assert!(archetype2 == archetype1);
    }
}
