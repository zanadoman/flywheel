/// ECS `Entity`.
#[repr(C)]
#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Entity(usize);

impl From<usize> for Entity {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl From<Entity> for usize {
    fn from(value: Entity) -> Self {
        value.0
    }
}
