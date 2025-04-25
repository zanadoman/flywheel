use super::{Entity, archetype::Archetype};

struct EntityData {
    alive: bool,
    archetype: Archetype,
    dirty: bool,
}

pub(super) struct EntityManager {
    entities: Vec<EntityData>,
    destroyed: Vec<usize>,
    dirty: Vec<Entity>,
    next_dirty: usize,
}

impl EntityManager {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            entities: Vec::new(),
            destroyed: Vec::new(),
            dirty: Vec::new(),
            next_dirty: 0,
        }
    }

    #[must_use]
    pub fn poll_dirty(&mut self) -> Option<Entity> {
        if self.dirty.len() <= self.next_dirty {
            self.dirty.clear();
            self.next_dirty = 0;
            None
        } else {
            let entity = self.dirty[self.next_dirty];
            self.entities[entity.id()].dirty = false;
            self.next_dirty += 1;
            Some(entity)
        }
    }

    #[must_use]
    pub fn spawn(&mut self) -> Entity {
        if let Some(id) = self.destroyed.pop() {
            self.entities[id].archetype.reset();
            self.entities[id].alive = true;
            Entity::new(id)
        } else {
            let entity = Entity::new(self.entities.len());
            self.entities.push(EntityData {
                alive: true,
                archetype: Archetype::new(),
                dirty: false,
            });
            entity
        }
    }

    #[must_use]
    pub fn archetype(&self, owner: Entity) -> Option<&Archetype> {
        let data = self.entities.get(owner.id())?;
        if data.alive {
            Some(&data.archetype)
        } else {
            None
        }
    }

    #[must_use]
    pub fn archetype_mut(&mut self, owner: Entity) -> Option<&mut Archetype> {
        if owner.id() < self.entities.len() && self.entities[owner.id()].alive {
            self.make_dirty(owner);
            Some(&mut self.entities[owner.id()].archetype)
        } else {
            None
        }
    }

    pub fn destroy(&mut self, entity: Entity) {
        let Some(data) = self.entities.get_mut(entity.id()) else {
            return;
        };
        if data.alive {
            data.alive = false;
            self.destroyed.push(entity.id());
            self.make_dirty(entity);
        }
    }

    fn make_dirty(&mut self, entity: Entity) {
        if !self.entities[entity.id()].dirty {
            self.entities[entity.id()].dirty = true;
            self.dirty.push(entity);
        }
    }
}

impl Default for EntityManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[must_use]
    fn dirty_archetype() -> Archetype {
        let mut archetype = Archetype::new();
        archetype.add(0);
        archetype
    }

    #[must_use]
    fn clean_archetype() -> Archetype {
        Archetype::new()
    }

    #[test]
    fn spawn_archetype_archetype_mut_destroy() {
        let mut entity_manager = EntityManager::new();
        let entity1_1 = entity_manager.spawn();
        assert!(
            *entity_manager.archetype(entity1_1).unwrap() == clean_archetype()
        );
        assert!(entity_manager.poll_dirty().is_none());
        *entity_manager.archetype_mut(entity1_1).unwrap() = dirty_archetype();
        assert_eq!(entity_manager.poll_dirty(), Some(entity1_1));
        assert!(entity_manager.poll_dirty().is_none());
        let entity2_1 = entity_manager.spawn();
        assert!(
            *entity_manager.archetype(entity2_1).unwrap() == clean_archetype()
        );
        assert!(entity_manager.poll_dirty().is_none());
        *entity_manager.archetype_mut(entity2_1).unwrap() = dirty_archetype();
        assert_eq!(entity_manager.poll_dirty(), Some(entity2_1));
        assert!(entity_manager.poll_dirty().is_none());
        entity_manager.destroy(entity1_1);
        assert_eq!(entity_manager.poll_dirty(), Some(entity1_1));
        assert!(entity_manager.archetype(entity1_1).is_none());
        assert!(entity_manager.poll_dirty().is_none());
        entity_manager.destroy(entity2_1);
        assert_eq!(entity_manager.poll_dirty(), Some(entity2_1));
        assert!(entity_manager.archetype(entity2_1).is_none());
        assert!(entity_manager.poll_dirty().is_none());
        let entity2_2 = entity_manager.spawn();
        assert!(entity2_2 == entity2_1);
        assert!(
            *entity_manager.archetype(entity2_2).unwrap() == clean_archetype()
        );
        *entity_manager.archetype_mut(entity2_2).unwrap() = dirty_archetype();
        let entity1_2 = entity_manager.spawn();
        assert!(entity1_2 == entity1_1);
        assert!(
            *entity_manager.archetype(entity1_2).unwrap() == clean_archetype()
        );
        *entity_manager.archetype_mut(entity1_2).unwrap() = dirty_archetype();
        entity_manager.destroy(entity2_2);
        assert!(entity_manager.archetype(entity2_2).is_none());
        entity_manager.destroy(entity1_2);
        assert!(entity_manager.archetype(entity1_2).is_none());
        assert_eq!(entity_manager.poll_dirty(), Some(entity2_2));
        assert_eq!(entity_manager.poll_dirty(), Some(entity1_2));
        assert_eq!(entity_manager.poll_dirty(), None);
    }
}
