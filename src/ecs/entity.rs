use std::fmt::{Display, Formatter, Result};

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

impl Display for Entity {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ID: usize = 5;
    const ENTITY: Entity = Entity::new(ID);

    #[test]
    fn id() {
        assert_eq!(ENTITY.id(), ID);
    }

    #[test]
    fn fmt() {
        assert_eq!(format!("{ENTITY}"), format!("{ID}"));
    }
}
