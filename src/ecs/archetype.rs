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

    pub fn insert(&mut self, id: usize) -> bool {
        if self.has(id) {
            return true;
        }
        if self.count <= id {
            self.count = id + 1;
            self.segments
                .resize(self.count.div_ceil(Segment::BITS as usize), 0);
        }
        self.segments[id / Segment::BITS as usize] |=
            1 << (id % Segment::BITS as usize);
        false
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

    pub fn remove(&mut self, id: usize) -> bool {
        if self.has(id) {
            self.destroy(id);
            true
        } else {
            false
        }
    }

    pub fn destroy(&mut self, id: usize) {
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

    #[must_use]
    fn setup() -> Archetype {
        let mut archetype = Archetype::new();
        assert!(!archetype.insert(0));
        assert!(!archetype.insert(1));
        archetype
    }

    #[test]
    fn insert() {
        let mut archetype = setup();
        assert!(archetype.insert(0));
        assert!(archetype.has(0));
        assert!(archetype.insert(1));
        assert!(archetype.has(1));
        assert!(!archetype.insert(Segment::BITS as usize));
        assert!(archetype.has(Segment::BITS as usize));
    }

    #[test]
    fn has() {
        let archetype = setup();
        for i in 0..2 {
            assert!(archetype.has(i));
        }
        for i in 2..=Segment::BITS as usize {
            assert!(!archetype.has(i));
        }
    }

    #[test]
    fn has_common_with() {
        let mut archetype = setup();
        let other = setup();
        assert!(archetype.has_common_with(&other));
        assert!(other.has_common_with(&archetype));
        archetype.clear();
        assert!(!archetype.has_common_with(&other));
        assert!(!other.has_common_with(&archetype));
        assert!(!archetype.insert(0));
        assert!(archetype.has_common_with(&other));
        assert!(other.has_common_with(&archetype));
    }

    #[test]
    fn is_subset_of() {
        let mut archetype = setup();
        let other = setup();
        assert!(archetype.is_subset_of(&other));
        assert!(other.is_subset_of(&archetype));
        archetype.insert(Segment::BITS as usize);
        assert!(!archetype.is_subset_of(&other));
        assert!(other.is_subset_of(&archetype));
    }

    #[test]
    fn is_superset_of() {
        let mut archetype = setup();
        let other = setup();
        assert!(archetype.is_superset_of(&other));
        assert!(other.is_superset_of(&archetype));
        archetype.insert(Segment::BITS as usize);
        assert!(archetype.is_superset_of(&other));
        assert!(!other.is_superset_of(&archetype));
    }

    #[test]
    fn remove() {
        let mut archetype = setup();
        assert!(!archetype.remove(Segment::BITS as usize));
        assert!(archetype.has(0));
        assert!(archetype.remove(0));
        assert!(!archetype.has(0));
        assert!(!archetype.remove(0));
        assert!(archetype.has(1));
        assert!(archetype.remove(1));
        assert!(!archetype.has(1));
        assert!(!archetype.remove(1));
        assert!(!archetype.remove(Segment::BITS as usize));
    }

    #[test]
    fn destroy() {
        let mut archetype = setup();
        archetype.destroy(Segment::BITS as usize);
        assert!(archetype.has(0));
        archetype.destroy(0);
        assert!(!archetype.has(0));
        archetype.destroy(0);
        assert!(archetype.has(1));
        archetype.destroy(1);
        assert!(!archetype.has(1));
        archetype.destroy(1);
        archetype.destroy(Segment::BITS as usize);
    }

    #[test]
    fn clear() {
        let mut archetype = setup();
        archetype.clear();
        for i in 0..=Segment::BITS as usize {
            assert!(!archetype.has(i));
        }
    }

    #[test]
    fn eq() {
        let mut archetype = setup();
        let other = setup();
        assert!(archetype == other);
        assert!(other == archetype);
        assert!(!archetype.insert(Segment::BITS as usize));
        assert!(archetype != other);
        assert!(other != archetype);
        assert!(archetype.remove(Segment::BITS as usize));
        assert!(archetype == other);
        assert!(other == archetype);
    }
}
