/// ECS `Entity`.
#[repr(C)]
#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub struct Entity(usize);

impl Entity {
    #[must_use]
    pub(super) const fn new(id: usize) -> Self {
        Self(id)
    }

    #[must_use]
    pub(super) const fn id(self) -> usize {
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
