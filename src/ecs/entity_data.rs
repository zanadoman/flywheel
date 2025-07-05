use super::{Entity, archetype::Archetype};

pub(super) struct EntityData {
    owner: Entity,
    archetype: Archetype,
    parent: Option<Entity>,
    sparse: Vec<Option<usize>>,
    dense: Vec<Entity>,
}

impl EntityData {
    #[must_use]
    pub const fn new(owner: Entity) -> Self {
        Self {
            owner,
            archetype: Archetype::new(),
            parent: None,
            sparse: Vec::new(),
            dense: Vec::new(),
        }
    }

    #[must_use]
    pub const fn owner(&self) -> Entity {
        self.owner
    }

    #[must_use]
    pub const fn archetype(&self) -> &Archetype {
        &self.archetype
    }

    #[must_use]
    pub const fn archetype_mut(&mut self) -> &mut Archetype {
        &mut self.archetype
    }

    #[must_use]
    pub const fn parent(&self) -> Option<Entity> {
        self.parent
    }

    pub fn set_parent(&mut self, value: Option<Entity>) -> Result<(), Entity> {
        if let Some(value) = value
            && self.has_child(value)
        {
            Err(value)
        } else {
            self.parent = value;
            Ok(())
        }
    }

    pub fn insert_child(&mut self, child: Entity) -> Result<(), Entity> {
        if let Some(parent) = self.parent
            && child == parent
        {
            return Err(child);
        }
        if self.sparse.len() <= child.id() {
            self.sparse.resize(child.id() + 1, None);
        }
        if self.sparse[child.id()].is_none() {
            self.sparse[child.id()] = Some(self.dense.len());
            self.dense.push(child);
        }
        Ok(())
    }

    pub fn has_child(&self, child: Entity) -> bool {
        self.sparse.get(child.id()).is_some_and(Option::is_some)
    }

    pub fn children(&self) -> &[Entity] {
        &self.dense
    }

    pub fn remove_child(&mut self, child: Entity) {
        if let Some(Some(index)) = self.sparse.get(child.id()).copied() {
            let last_index = self.dense.len() - 1;
            if index != last_index {
                self.dense.swap(index, last_index);
                self.sparse[self.dense[index].id()] = Some(index);
            }
            self.dense.pop();
            self.sparse[child.id()] = None;
        }
    }

    pub fn clear(&mut self) {
        self.parent = None;
        self.archetype.clear();
        self.sparse.clear();
        self.dense.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const OWNER: Entity = Entity::new(0);
    const PARENT: Entity = Entity::new(1);
    const ENTITY2: Entity = Entity::new(2);
    const ENTITY3: Entity = Entity::new(3);
    const ENTITY4: Entity = Entity::new(4);

    #[must_use]
    fn setup() -> EntityData {
        let mut entity_data = EntityData::new(OWNER);
        assert!(entity_data.set_parent(Some(PARENT)).is_ok());
        assert!(entity_data.insert_child(ENTITY2).is_ok());
        assert!(entity_data.insert_child(ENTITY3).is_ok());
        entity_data
    }

    #[test]
    fn owner() {
        assert_eq!(setup().owner(), OWNER);
    }

    #[test]
    fn archetype() {
        assert!(!setup().archetype().is_dirty());
    }

    #[test]
    fn archetype_mut() {
        assert!(!setup().archetype_mut().is_dirty());
    }

    #[test]
    fn parent() {
        assert_eq!(setup().parent(), Some(PARENT));
    }

    #[test]
    fn set_parent() {
        let mut entity_data = setup();
        assert!(entity_data.set_parent(Some(ENTITY4)).is_ok());
        assert_eq!(entity_data.parent(), Some(ENTITY4));
        assert_eq!(entity_data.set_parent(Some(ENTITY2)), Err(ENTITY2));
        assert_eq!(entity_data.parent(), Some(ENTITY4));
        entity_data.remove_child(ENTITY2);
        assert!(entity_data.set_parent(Some(ENTITY2)).is_ok());
        assert_eq!(entity_data.parent(), Some(ENTITY2));
    }

    #[test]
    fn insert_child() {
        let mut entity_data = setup();
        assert!(entity_data.insert_child(ENTITY4).is_ok());
        assert!(entity_data.has_child(ENTITY4));
        assert_eq!(entity_data.insert_child(PARENT), Err(PARENT));
        assert!(!entity_data.has_child(PARENT));
        assert!(entity_data.set_parent(None).is_ok());
        assert!(entity_data.insert_child(PARENT).is_ok());
        assert!(entity_data.has_child(PARENT));
    }

    #[test]
    fn has_child() {
        let entity_data = setup();
        assert!(entity_data.has_child(ENTITY2));
        assert!(entity_data.has_child(ENTITY3));
        assert!(!entity_data.has_child(ENTITY4));
    }

    #[test]
    fn children() {
        let entity_data = setup();
        assert_eq!(entity_data.children().len(), 2);
        assert!(entity_data.children().contains(&ENTITY2));
        assert!(entity_data.children().contains(&ENTITY3));
    }

    #[test]
    fn remove_child() {
        let mut entity_data = setup();
        entity_data.remove_child(ENTITY4);
        assert!(entity_data.has_child(ENTITY2));
        entity_data.remove_child(ENTITY2);
        assert!(!entity_data.has_child(ENTITY2));
        entity_data.remove_child(ENTITY2);
        assert!(entity_data.has_child(ENTITY3));
        entity_data.remove_child(ENTITY3);
        assert!(!entity_data.has_child(ENTITY3));
        entity_data.remove_child(ENTITY3);
        entity_data.remove_child(ENTITY4);
    }

    #[test]
    fn clear() {
        let mut entity_data = setup();
        entity_data.clear();
        assert_eq!(entity_data.owner(), OWNER);
        assert!(entity_data.archetype.is_dirty());
        assert!(entity_data.parent().is_none());
        assert!(!entity_data.has_child(ENTITY2));
        assert!(!entity_data.has_child(ENTITY3));
        assert!(entity_data.children().is_empty());
    }
}
