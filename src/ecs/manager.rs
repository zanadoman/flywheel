use super::{
    Entity, archetype::Archetype, component_manager::ComponentManager,
    entity_manager::EntityManager,
};

pub struct Manager {
    entities: EntityManager,
    components: ComponentManager,
}

impl Manager {
    #[must_use]
    pub(super) fn new() -> Self {
        Self {
            entities: EntityManager::new(),
            components: ComponentManager::new(),
        }
    }

    #[must_use]
    pub(super) fn component_id_or_register<T: 'static>(&mut self) -> usize {
        self.components.id_or_register::<T>()
    }

    #[must_use]
    pub(super) fn entity_archetype(&self, owner: Entity) -> Option<&Archetype> {
        self.entities.archetype(owner)
    }

    #[must_use]
    pub(super) fn poll_dirty(&mut self) -> Option<Entity> {
        self.entities.poll_dirty()
    }

    #[must_use]
    pub fn spawn_entity(&mut self) -> Entity {
        self.entities.spawn()
    }

    #[must_use]
    pub fn is_entity_alive(&self, entity: Entity) -> bool {
        self.entities.archetype(entity).is_some()
    }

    #[must_use]
    pub fn is_entity_has_common_with(
        &self,
        entity: Entity,
        other: Entity,
    ) -> Option<bool> {
        Some(
            self.entities
                .archetype(entity)?
                .has_common_with(self.entities.archetype(other)?),
        )
    }

    #[must_use]
    pub fn is_entity_subset_of(
        &self,
        entity: Entity,
        other: Entity,
    ) -> Option<bool> {
        Some(
            self.entities
                .archetype(entity)?
                .is_subset_of(self.entities.archetype(other)?),
        )
    }

    #[must_use]
    pub fn is_entity_superset_of(
        &self,
        entity: Entity,
        other: Entity,
    ) -> Option<bool> {
        Some(
            self.entities
                .archetype(entity)?
                .is_superset_of(self.entities.archetype(other)?),
        )
    }

    #[must_use]
    pub fn has_component<T: 'static>(&self, owner: Entity) -> bool {
        let Some(entity_archetype) = self.entities.archetype(owner) else {
            return false;
        };
        let Some(component_id) = self.components.id::<T>() else {
            return false;
        };
        entity_archetype.has(component_id)
    }

    pub fn add_component<T: 'static>(
        mut self,
        owner: Entity,
        component: T,
    ) -> Result<(), T> {
        let Some(owner_archetype) = self.entities.archetype_mut(owner) else {
            return Err(component);
        };
        let component_id = self.components.id_or_register::<T>();
        self.components.add(owner, component)?;
        owner_archetype.add(component_id);
        Ok(())
    }

    #[must_use]
    pub fn component<T: 'static>(&self, owner: Entity) -> Option<&T> {
        self.components.get(owner)
    }

    #[must_use]
    pub fn component_mut<T: 'static>(
        &mut self,
        owner: Entity,
    ) -> Option<&mut T> {
        self.components.get_mut(owner)
    }

    #[must_use]
    pub fn all_component<T: 'static>(&self) -> &[T] {
        self.components.all()
    }

    #[must_use]
    pub fn all_component_mut<T: 'static>(&mut self) -> &mut [T] {
        self.components.all_mut()
    }

    #[must_use]
    pub fn component_owners<T: 'static>(&self) -> &[Entity] {
        self.components.owners::<T>()
    }

    pub fn remove_component<T: 'static>(&mut self, owner: Entity) {
        let Some(owner_archetype) = self.entities.archetype_mut(owner) else {
            return;
        };
        let Some(component_id) = self.components.id::<T>() else {
            return;
        };
        if !owner_archetype.has(component_id) {
            return;
        }
        self.components.remove::<T>(owner);
        owner_archetype.remove(component_id);
    }

    pub fn destroy_entity(&mut self, entity: Entity) {
        self.components.remove_all(entity);
        self.entities.destroy(entity);
    }
}

impl Default for Manager {
    fn default() -> Self {
        Self::new()
    }
}
