/// ECS `Entity`.
#[repr(C)]
#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Entity(usize);

impl Entity {
    /// Constructs a new `Entity` from the given ID.
    #[must_use]
    pub const fn new(id: usize) -> Self {
        Self(id)
    }

    /// Returns the ID of the `Entity`.
    #[must_use]
    pub const fn id(&self) -> usize {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        const ID: usize = 5;
        const ENTITY: Entity = Entity::new(ID);
        assert_eq!(ENTITY.id(), ID);
    }
}
