type Byte = usize;

/// ECS `Archetype` representing the set of component IDs associated with an
/// `Entity`.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

    /// Returns whether the `Archetype` is the subset of another `Archetype`.
    #[must_use]
    pub fn is_subset_of(&self, other: &Self) -> bool {
        if other.count < self.count {
            return false;
        }
        for i in 0..self.bytes.len() {
            if self.bytes[i] & other.bytes[i] != self.bytes[i] {
                return false;
            }
        }
        true
    }

    /// Returns whether the `Archetype` is the superset of another `Archetype`.
    #[must_use]
    pub fn is_superset_of(&self, other: &Self) -> bool {
        if self.count < other.count {
            return false;
        }
        for i in 0..other.bytes.len() {
            if other.bytes[i] & self.bytes[i] != other.bytes[i] {
                return false;
            }
        }
        true
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
        archetype1.add(2);
        let mut archetype2 = Archetype::new();
        archetype2.add(0);
        archetype2.add(1);
        archetype2.add(2);
        assert!(!archetype1.is_superset_of(&archetype2));
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
}
