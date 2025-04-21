type Byte = usize;

/// ECS `Archetype` representing the set of component IDs associated with an
/// `Entity`.
#[derive(Debug, Eq, Clone)]
pub struct Archetype {
    count: usize,
    bytes: Vec<Byte>,
}

impl Archetype {
    const BITS: usize = Byte::BITS as usize;

    /// Constructs a new empty `Archetype`.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            count: 0,
            bytes: Vec::new(),
        }
    }

    /// Adds a component ID to the `Archetype`.
    pub fn add(&mut self, id: usize) {
        if self.count <= id {
            self.count = id + 1;
            self.bytes.resize(self.count.div_ceil(Self::BITS), 0);
        }
        self.bytes[id / Self::BITS] |= 1 << (id % Self::BITS);
    }

    /// Returns whether the `Archetype` has a component ID.
    #[must_use]
    pub fn has(&self, id: usize) -> bool {
        id < self.count
            && self.bytes[id / Self::BITS] & 1 << (id % Self::BITS) != 0
    }

    /// Returns whether the `Archetype` has any common component ID with another
    /// `Archetype`.
    #[must_use]
    pub fn has_common_with(&self, other: &Self) -> bool {
        self.bytes.iter().zip(&other.bytes).any(|(s, o)| s & o != 0)
    }

    /// Returns whether the `Archetype` is the subset of another `Archetype`.
    #[must_use]
    pub fn is_subset_of(&self, other: &Self) -> bool {
        self.count <= other.count
            && self
                .bytes
                .iter()
                .zip(&other.bytes)
                .all(|(s, o)| s & o == *s)
    }

    /// Returns whether the `Archetype` is the superset of another `Archetype`.
    #[must_use]
    pub fn is_superset_of(&self, other: &Self) -> bool {
        other.count <= self.count
            && other
                .bytes
                .iter()
                .zip(&self.bytes)
                .all(|(o, s)| o & s == *o)
    }

    /// Removes a component ID from the `Archetype`.
    pub fn remove(&mut self, id: usize) {
        if id < self.count {
            self.bytes[id / Self::BITS] &= !(1 << (id % Self::BITS));
        }
    }

    /// Removes every component ID from the `Archetype`.
    pub fn reset(&mut self) {
        self.bytes.fill(0);
    }
}

impl PartialEq for Archetype {
    fn eq(&self, other: &Self) -> bool {
        let len = self.bytes.len().min(other.bytes.len());
        self.bytes.iter().take(len).eq(other.bytes.iter().take(len))
            && self.bytes.iter().skip(len).all(|b| *b == 0)
            && other.bytes.iter().skip(len).all(|b| *b == 0)
    }
}

impl Default for Archetype {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_has_remove() {
        const N: usize = 100;
        let mut archetype = Archetype::new();
        for i in 0..N {
            assert!(!archetype.has(i));
            archetype.add(i);
            assert!(archetype.has(i));
            archetype.remove(i);
            assert!(!archetype.has(i));
        }
        let mut archetype = Archetype::new();
        for i in (0..N).rev() {
            assert!(!archetype.has(i));
            archetype.add(i);
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
        archetype1.add(0);
        let mut archetype2 = Archetype::new();
        archetype2.add(2);
        assert!(!archetype1.has_common_with(&archetype2));
        assert!(!archetype2.has_common_with(&archetype1));
        archetype1.add(1);
        archetype2.add(1);
        assert!(archetype1.has_common_with(&archetype2));
        assert!(archetype2.has_common_with(&archetype1));
    }

    #[test]
    fn is_subset_of() {
        let mut archetype1 = Archetype::new();
        archetype1.add(0);
        archetype1.add(1);
        archetype1.add(2);
        let mut archetype2 = Archetype::new();
        archetype2.add(0);
        archetype2.add(1);
        archetype2.add(2);
        assert!(archetype1.is_subset_of(&archetype2));
        assert!(archetype2.is_subset_of(&archetype1));
        let mut archetype1 = Archetype::new();
        archetype1.add(0);
        archetype1.add(2);
        let mut archetype2 = Archetype::new();
        archetype2.add(0);
        archetype2.add(1);
        archetype2.add(2);
        assert!(archetype1.is_subset_of(&archetype2));
        assert!(!archetype2.is_subset_of(&archetype1));
        let mut archetype1 = Archetype::new();
        archetype1.add(0);
        archetype1.add(1);
        archetype1.add(2);
        let mut archetype2 = Archetype::new();
        archetype2.add(0);
        archetype2.add(2);
        assert!(!archetype1.is_subset_of(&archetype2));
        assert!(archetype2.is_subset_of(&archetype1));
    }

    #[test]
    fn is_superset_of() {
        let mut archetype1 = Archetype::new();
        archetype1.add(0);
        archetype1.add(1);
        archetype1.add(2);
        let mut archetype2 = Archetype::new();
        archetype2.add(0);
        archetype2.add(1);
        archetype2.add(2);
        assert!(archetype1.is_superset_of(&archetype2));
        assert!(archetype2.is_superset_of(&archetype1));
        let mut archetype1 = Archetype::new();
        archetype1.add(0);
        archetype1.add(1);
        archetype1.add(2);
        let mut archetype2 = Archetype::new();
        archetype2.add(0);
        archetype2.add(2);
        assert!(archetype1.is_superset_of(&archetype2));
        assert!(!archetype2.is_superset_of(&archetype1));
        let mut archetype1 = Archetype::new();
        archetype1.add(0);
        archetype1.add(2);
        let mut archetype2 = Archetype::new();
        archetype2.add(0);
        archetype2.add(1);
        archetype2.add(2);
        assert!(!archetype1.is_superset_of(&archetype2));
        assert!(archetype2.is_superset_of(&archetype1));
    }

    #[test]
    fn reset() {
        const N: usize = 100;
        let mut archetype = Archetype::new();
        for i in 0..N {
            archetype.add(i);
            assert!(archetype.has(i));
        }
        archetype.reset();
        for i in 0..N {
            assert!(!archetype.has(i));
        }
    }

    #[test]
    fn eq() {
        let mut archetype1 = Archetype::new();
        archetype1.add(0);
        archetype1.add(1);
        archetype1.add(2);
        let mut archetype2 = Archetype::new();
        archetype2.add(0);
        archetype2.add(2);
        assert!(archetype1 != archetype2);
        assert!(archetype2 != archetype1);
        archetype2.add(1);
        assert!(archetype1 == archetype2);
        assert!(archetype2 == archetype1);
        let mut archetype1 = Archetype::new();
        archetype1.add(0);
        let mut archetype2 = Archetype::new();
        archetype2.add(0);
        archetype2.add(Archetype::BITS);
        assert!(archetype1 != archetype2);
        assert!(archetype2 != archetype1);
        archetype2.remove(Archetype::BITS);
        assert!(archetype1 == archetype2);
        assert!(archetype2 == archetype1);
    }
}
