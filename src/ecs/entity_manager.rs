use super::{Archetype, Entity};

pub struct EntityManager {
    archetypes: Vec<Option<Archetype>>,
    destroyed: Vec<Entity>,
}

impl EntityManager {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            archetypes: Vec::new(),
            destroyed: Vec::new(),
        }
    }

    #[must_use]
    pub fn spawn(&mut self) -> Entity {
        if let Some(entity) = self.destroyed.pop() {
            self.archetypes[entity.id()] = Some(Archetype::new());
            entity
        } else {
            let entity = Entity::new(self.archetypes.len());
            self.archetypes.push(Some(Archetype::new()));
            entity
        }
    }

    #[must_use]
    pub fn archetype(&self, owner: Entity) -> Option<&Archetype> {
        self.archetypes.get(owner.id())?.as_ref()
    }

    #[must_use]
    pub fn archetype_mut(&mut self, owner: Entity) -> Option<&mut Archetype> {
        self.archetypes.get_mut(owner.id())?.as_mut()
    }

    pub fn destroy(&mut self, entity: Entity) {
        let Some(element) = self.archetypes.get_mut(entity.id()) else {
            return;
        };
        if element.is_some() {
            self.destroyed.push(entity);
            *element = None;
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

    #[test]
    fn new_spawn_destroy_archetype() {
        let mut entity_manager = EntityManager::new();
        let entity1_1 = entity_manager.spawn();
        assert!(entity_manager.archetype(entity1_1).is_some());
        let entity2_1 = entity_manager.spawn();
        assert!(entity_manager.archetype(entity2_1).is_some());
        entity_manager.destroy(entity1_1);
        assert!(entity_manager.archetype(entity1_1).is_none());
        entity_manager.destroy(entity2_1);
        assert!(entity_manager.archetype(entity2_1).is_none());
        let entity2_2 = entity_manager.spawn();
        assert!(entity2_2 == entity2_1);
        assert!(entity_manager.archetype(entity2_2).is_some());
        let entity1_2 = entity_manager.spawn();
        assert!(entity1_2 == entity1_1);
        assert!(entity_manager.archetype(entity1_2).is_some());
    }

    #[test]
    fn new_spawn_destroy_archetype_mut() {
        let mut entity_manager = EntityManager::new();
        let entity1_1 = entity_manager.spawn();
        assert!(entity_manager.archetype_mut(entity1_1).is_some());
        let entity2_1 = entity_manager.spawn();
        assert!(entity_manager.archetype_mut(entity2_1).is_some());
        entity_manager.destroy(entity1_1);
        assert!(entity_manager.archetype_mut(entity1_1).is_none());
        entity_manager.destroy(entity2_1);
        assert!(entity_manager.archetype_mut(entity2_1).is_none());
        let entity1_2 = entity_manager.spawn();
        assert!(entity1_2 != entity1_1);
        assert!(entity_manager.archetype_mut(entity1_2).is_some());
        let entity2_2 = entity_manager.spawn();
        assert!(entity2_2 != entity2_1);
        assert!(entity_manager.archetype_mut(entity2_2).is_some());
    }
}
