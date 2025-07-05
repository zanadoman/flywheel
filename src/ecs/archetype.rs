use super::Entity;

type Segment = usize;

pub struct Archetype {
    owner: Option<Entity>,
    count: usize,
    segments: Vec<Segment>,
    dirty: bool,
}

impl Archetype {
    #[must_use]
    pub const fn new(owner: Option<Entity>) -> Self {
        Self {
            owner,
            count: 0,
            segments: Vec::new(),
            dirty: false,
        }
    }

    #[must_use]
    pub fn owner(&self) -> Option<Entity> {
        self.owner
    }

    pub fn insert(&mut self, id: usize) -> bool {
        let index = id / Segment::BITS as usize;
        let bitmask = 1 << (id % Segment::BITS as usize);
        if self.count <= id {
            self.count = id + 1;
            self.segments
                .resize(self.count.div_ceil(Segment::BITS as usize), 0);
            self.segments[index] |= bitmask;
            self.dirty = true;
            false
        } else if self.segments[index] & bitmask == 0 {
            self.segments[index] |= bitmask;
            self.dirty = true;
            false
        } else {
            true
        }
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
        let index = id / Segment::BITS as usize;
        let bitmask = 1 << (id % Segment::BITS as usize);
        if id < self.count && self.segments[index] & bitmask != 0 {
            self.segments[index] &= !bitmask;
            self.dirty = true;
            true
        } else {
            false
        }
    }

    pub fn clear(&mut self) {
        self.segments.fill(0);
        self.dirty = true;
    }

    #[must_use]
    pub fn dirty(&self) -> bool {
        self.dirty
    }

    pub fn flush(&mut self) {
        self.dirty = false;
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
            && other.segments.iter().skip(len).all(|o| *o == 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const OWNER: Entity = Entity::new(0);

    #[must_use]
    fn setup() -> Archetype {
        let mut archetype = Archetype::new(Some(OWNER));
        assert!(!archetype.dirty());
        assert!(!archetype.insert(0));
        assert!(archetype.dirty());
        archetype.flush();
        assert!(!archetype.dirty());
        assert!(!archetype.insert(1));
        assert!(archetype.dirty());
        archetype.flush();
        assert!(!archetype.dirty());
        archetype
    }

    #[test]
    fn owner() {
        assert_eq!(setup().owner(), Some(OWNER));
    }

    #[test]
    fn insert() {
        let mut archetype = setup();
        assert!(archetype.insert(0));
        assert!(!archetype.dirty());
        assert!(archetype.has(0));
        assert!(archetype.insert(1));
        assert!(!archetype.dirty());
        assert!(archetype.has(1));
        assert!(!archetype.insert(Segment::BITS as usize));
        assert!(archetype.dirty());
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
        assert!(!archetype.dirty());
        assert!(archetype.has(0));
        assert!(archetype.remove(0));
        assert!(archetype.dirty());
        archetype.flush();
        assert!(!archetype.has(0));
        assert!(!archetype.remove(0));
        assert!(!archetype.dirty());
        assert!(archetype.has(1));
        assert!(archetype.remove(1));
        assert!(archetype.dirty());
        archetype.flush();
        assert!(!archetype.has(1));
        assert!(!archetype.remove(1));
        assert!(!archetype.dirty());
        assert!(!archetype.remove(Segment::BITS as usize));
        assert!(!archetype.dirty());
    }

    #[test]
    fn clear() {
        let mut archetype = setup();
        archetype.clear();
        assert!(archetype.dirty());
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
