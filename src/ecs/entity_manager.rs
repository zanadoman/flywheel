use super::{Archetype, Entity};

pub struct EntityManager {
    entities: Vec<(bool, Archetype)>,
    destroyed: Vec<usize>,
}

impl EntityManager {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            entities: Vec::new(),
            destroyed: Vec::new(),
        }
    }

    #[must_use]
    pub fn spawn(&mut self) -> Entity {
        if let Some(id) = self.destroyed.pop() {
            self.entities[id].1.reset();
            self.entities[id].0 = true;
            Entity::new(id)
        } else {
            let entity = Entity::new(self.entities.len());
            self.entities.push((true, Archetype::new()));
            entity
        }
    }

    #[must_use]
    pub fn archetype(&self, owner: Entity) -> Option<&Archetype> {
        let (alive, archetype) = self.entities.get(owner.id())?;
        if *alive { Some(archetype) } else { None }
    }

    #[must_use]
    pub fn archetype_mut(&mut self, owner: Entity) -> Option<&mut Archetype> {
        let (alive, archetype) = self.entities.get_mut(owner.id())?;
        if *alive { Some(archetype) } else { None }
    }

    pub fn destroy(&mut self, entity: Entity) {
        let Some((alive, _)) = self.entities.get_mut(entity.id()) else {
            return;
        };
        if *alive {
            *alive = false;
            self.destroyed.push(entity.id());
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

    fn dirty_archetype() -> Archetype {
        let mut archetype = Archetype::new();
        archetype.add(0);
        archetype
    }

    fn clean_archetype() -> Archetype {
        Archetype::new()
    }

    #[test]
    fn new_spawn_destroy_archetype_archetype_mut() {
        let mut entity_manager = EntityManager::new();
        let entity1_1 = entity_manager.spawn();
        assert!(
            *entity_manager.archetype_mut(entity1_1).unwrap()
                == clean_archetype()
        );
        *entity_manager.archetype_mut(entity1_1).unwrap() = dirty_archetype();
        let entity2_1 = entity_manager.spawn();
        assert!(
            *entity_manager.archetype_mut(entity2_1).unwrap()
                == clean_archetype()
        );
        *entity_manager.archetype_mut(entity2_1).unwrap() = dirty_archetype();
        entity_manager.destroy(entity1_1);
        assert!(entity_manager.archetype(entity1_1).is_none());
        entity_manager.destroy(entity2_1);
        assert!(entity_manager.archetype(entity2_1).is_none());
        let entity2_2 = entity_manager.spawn();
        assert!(entity2_2 == entity2_1);
        assert!(
            *entity_manager.archetype_mut(entity2_2).unwrap()
                == clean_archetype()
        );
        let entity1_2 = entity_manager.spawn();
        assert!(entity1_2 == entity1_1);
        assert!(
            *entity_manager.archetype_mut(entity1_2).unwrap()
                == clean_archetype()
        );
        entity_manager.destroy(entity2_2);
        assert!(entity_manager.archetype(entity2_2).is_none());
        entity_manager.destroy(entity1_2);
        assert!(entity_manager.archetype(entity1_2).is_none());
        let entity1_1 = entity_manager.spawn();
        assert!(entity1_1 == entity1_2);
        assert!(
            *entity_manager.archetype_mut(entity1_1).unwrap()
                == clean_archetype()
        );
        let entity2_1 = entity_manager.spawn();
        assert!(entity2_1 == entity2_2);
        assert!(
            *entity_manager.archetype_mut(entity2_1).unwrap()
                == clean_archetype()
        );
    }
}
