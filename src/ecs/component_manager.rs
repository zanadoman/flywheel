use core::any::{Any, TypeId};
use std::collections::HashMap;

use super::{
    Entity,
    component_pool::{ComponentPool, SparseSet},
};

/// Manages the storage and retrieval of components of different types for
/// entities in an ECS system.
///
/// Each component type is stored in its own `ComponentPool`, enabling O(1)
/// operations for adding, retrieving, and removing components. The efficiency
/// of accessing a `ComponentPool` of type `T` is also O(1).
pub struct ComponentManager {
    ids: HashMap<TypeId, usize>,
    pools: Vec<Box<dyn SparseSet>>,
}

impl ComponentManager {
    /// Constructs a new `ComponentManager`.
    #[must_use]
    pub fn new() -> Self {
        Self {
            ids: HashMap::new(),
            pools: Vec::new(),
        }
    }

    /// Returns the ID of the component of type `T`.
    #[must_use]
    pub fn id<T: 'static>(&self) -> Option<usize> {
        self.ids.get(&TypeId::of::<T>()).copied()
    }

    /// Adds a component of type `T` for the given `Entity`.
    ///
    /// # Errors
    ///
    /// Returns the component of type `T` if the given `Entity` already has a
    /// component of type `T` in the set.
    pub fn add<T: 'static>(
        &mut self,
        entity: Entity,
        component: T,
    ) -> Result<(), T> {
        if let Some(pool) = self.pool_mut() {
            pool.add(entity, component)
        } else if self
            .ids
            .insert(TypeId::of::<T>(), self.pools.len())
            .is_none()
        {
            let mut pool = Box::new(ComponentPool::new());
            let result = pool.add(entity, component);
            self.pools.push(pool);
            result
        } else {
            Err(component)
        }
    }

    /// Returns a reference to the component of type `T` associated with the
    /// given `Entity`.
    #[must_use]
    pub fn get<T: 'static>(&self, entity: Entity) -> Option<&T> {
        self.pool()?.get(entity)
    }

    /// Returns a mutable reference to the component of type `T` associated with
    /// the given `Entity`.
    #[must_use]
    pub fn get_mut<T: 'static>(&mut self, entity: Entity) -> Option<&mut T> {
        self.pool_mut()?.get_mut(entity)
    }

    /// Returns a slice of all components of type `T` in the set.
    #[must_use]
    pub fn all<T: 'static>(&self) -> &[T] {
        self.pool().map_or(&[], |p| p.all())
    }

    /// Returns a mutable slice of all components of type `T` in the set.
    #[must_use]
    pub fn all_mut<T: 'static>(&mut self) -> &mut [T] {
        self.pool_mut().map_or(&mut [], |p| p.all_mut())
    }

    /// Returns a slice of all entities that have a component of type `T` in the
    /// set.
    #[must_use]
    pub fn owners<T: 'static>(&self) -> &[Entity] {
        self.pool::<T>().map_or(&[], |p| p.owners())
    }

    /// Removes the component of type `T` associated with the given `Entity`.
    pub fn remove<T: 'static>(&mut self, entity: Entity) {
        if let Some(pool) = self.pool_mut::<T>() {
            pool.remove(entity);
        }
    }

    /// Removes every component associated with the given `Entity`.
    pub fn remove_all(&mut self, entity: Entity) {
        for pool in &mut self.pools {
            pool.remove(entity);
        }
    }

    #[must_use]
    fn pool<T: 'static>(&self) -> Option<&ComponentPool<T>> {
        (self.pools[self.id::<T>()?].as_ref() as &dyn Any).downcast_ref()
    }

    #[must_use]
    fn pool_mut<T: 'static>(&mut self) -> Option<&mut ComponentPool<T>> {
        let id = self.id::<T>()?;
        (self.pools[id].as_mut() as &mut dyn Any).downcast_mut()
    }
}

impl Default for ComponentManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Eq, PartialEq)]
    struct Health(u8);

    #[derive(Debug, Eq, PartialEq)]
    struct Damage(u8);

    #[derive(Debug, Eq, PartialEq)]
    struct Arrows(u8);

    #[derive(Debug, Eq, PartialEq)]
    struct Points(u8);

    const ENTITY0: Entity = Entity::new(0);
    const ENTITY0_HEALTH: u8 = 1;
    const ENTITY0_DAMAGE: u8 = 2;
    const ENTITY0_ARROWS: u8 = 3;
    const ENTITY0_POINTS: u8 = 4;
    const ENTITY1: Entity = Entity::new(1);
    const ENTITY1_HEALTH: u8 = 5;
    const ENTITY1_DAMAGE: u8 = 6;
    const ENTITY1_ARROWS: u8 = 7;
    const ENTITY1_POINTS: u8 = 8;
    const ENTITY2: Entity = Entity::new(2);
    const ENTITY2_HEALTH: u8 = 9;
    const ENTITY2_DAMAGE: u8 = 10;
    const ENTITY2_ARROWS: u8 = 11;
    const ENTITY2_POINTS: u8 = 12;

    fn setup() -> ComponentManager {
        let mut component_manager = ComponentManager::new();
        component_manager
            .add(ENTITY0, Health(ENTITY0_HEALTH))
            .unwrap();
        component_manager
            .add(ENTITY0, Damage(ENTITY0_DAMAGE))
            .unwrap();
        component_manager
            .add(ENTITY1, Health(ENTITY1_HEALTH))
            .unwrap();
        component_manager
            .add(ENTITY1, Damage(ENTITY1_DAMAGE))
            .unwrap();
        component_manager
    }

    #[test]
    fn id() {
        let component_manager = setup();
        assert_eq!(component_manager.id::<Health>().unwrap(), 0);
        assert_eq!(component_manager.id::<Damage>().unwrap(), 1);
        assert!(component_manager.id::<Arrows>().is_none());
        assert!(component_manager.id::<Points>().is_none());
    }

    #[test]
    fn add() {
        let mut component_manager = setup();
        assert_eq!(
            component_manager.add(ENTITY0, Health(ENTITY0_HEALTH)),
            Err(Health(ENTITY0_HEALTH))
        );
        assert_eq!(
            component_manager.add(ENTITY0, Damage(ENTITY0_DAMAGE)),
            Err(Damage(ENTITY0_DAMAGE))
        );
        assert!(
            component_manager
                .add(ENTITY0, Arrows(ENTITY0_ARROWS))
                .is_ok(),
        );
        assert!(
            component_manager
                .add(ENTITY0, Points(ENTITY0_POINTS))
                .is_ok()
        );
        assert_eq!(
            component_manager.add(ENTITY1, Health(ENTITY1_HEALTH)),
            Err(Health(ENTITY1_HEALTH))
        );
        assert_eq!(
            component_manager.add(ENTITY1, Damage(ENTITY1_DAMAGE)),
            Err(Damage(ENTITY1_DAMAGE))
        );
        assert!(
            component_manager
                .add(ENTITY1, Arrows(ENTITY1_ARROWS))
                .is_ok(),
        );
        assert!(
            component_manager
                .add(ENTITY1, Points(ENTITY1_POINTS))
                .is_ok()
        );
        assert!(
            component_manager
                .add(ENTITY2, Health(ENTITY2_HEALTH))
                .is_ok(),
        );
        assert!(
            component_manager
                .add(ENTITY2, Damage(ENTITY2_DAMAGE))
                .is_ok()
        );
        assert!(
            component_manager
                .add(ENTITY2, Arrows(ENTITY2_ARROWS))
                .is_ok(),
        );
        assert!(
            component_manager
                .add(ENTITY2, Points(ENTITY2_POINTS))
                .is_ok()
        );
    }

    #[test]
    fn get() {
        let component_manager = setup();
        assert_eq!(
            component_manager.get(ENTITY0),
            Some(&Health(ENTITY0_HEALTH))
        );
        assert_eq!(
            component_manager.get(ENTITY0),
            Some(&Damage(ENTITY0_DAMAGE))
        );
        assert!(component_manager.get::<Arrows>(ENTITY0).is_none());
        assert!(component_manager.get::<Points>(ENTITY0).is_none());
        assert_eq!(
            component_manager.get(ENTITY1),
            Some(&Health(ENTITY1_HEALTH))
        );
        assert_eq!(
            component_manager.get(ENTITY1),
            Some(&Damage(ENTITY1_DAMAGE))
        );
        assert!(component_manager.get::<Arrows>(ENTITY1).is_none());
        assert!(component_manager.get::<Points>(ENTITY1).is_none());
        assert!(component_manager.get::<Health>(ENTITY2).is_none());
        assert!(component_manager.get::<Damage>(ENTITY2).is_none());
        assert!(component_manager.get::<Arrows>(ENTITY2).is_none());
        assert!(component_manager.get::<Points>(ENTITY2).is_none());
    }

    #[test]
    fn get_mut() {
        let mut component_manager = setup();
        assert_eq!(
            component_manager.get_mut(ENTITY0),
            Some(&mut Health(ENTITY0_HEALTH))
        );
        assert_eq!(
            component_manager.get_mut(ENTITY0),
            Some(&mut Damage(ENTITY0_DAMAGE))
        );
        assert!(component_manager.get_mut::<Arrows>(ENTITY0).is_none());
        assert!(component_manager.get_mut::<Points>(ENTITY0).is_none());
        assert_eq!(
            component_manager.get_mut(ENTITY1),
            Some(&mut Health(ENTITY1_HEALTH))
        );
        assert_eq!(
            component_manager.get_mut(ENTITY1),
            Some(&mut Damage(ENTITY1_DAMAGE))
        );
        assert!(component_manager.get_mut::<Arrows>(ENTITY1).is_none());
        assert!(component_manager.get_mut::<Points>(ENTITY1).is_none());
        assert!(component_manager.get_mut::<Health>(ENTITY2).is_none());
        assert!(component_manager.get_mut::<Damage>(ENTITY2).is_none());
        assert!(component_manager.get_mut::<Arrows>(ENTITY2).is_none());
        assert!(component_manager.get_mut::<Points>(ENTITY2).is_none());
    }

    #[test]
    fn all() {
        let component_manager = setup();
        let all = component_manager.all();
        assert_eq!(all.len(), 2);
        assert!(all.contains(&Health(ENTITY0_HEALTH)));
        assert!(all.contains(&Health(ENTITY1_HEALTH)));
        let all = component_manager.all();
        assert_eq!(all.len(), 2);
        assert!(all.contains(&Damage(ENTITY0_DAMAGE)));
        assert!(all.contains(&Damage(ENTITY1_DAMAGE)));
        assert!(component_manager.all::<Arrows>().is_empty());
        assert!(component_manager.all::<Points>().is_empty());
    }

    #[test]
    fn all_mut() {
        let mut component_manager = setup();
        let all = component_manager.all_mut();
        assert_eq!(all.len(), 2);
        assert!(all.contains(&Health(ENTITY0_HEALTH)));
        assert!(all.contains(&Health(ENTITY1_HEALTH)));
        let all = component_manager.all_mut();
        assert_eq!(all.len(), 2);
        assert!(all.contains(&Damage(ENTITY0_DAMAGE)));
        assert!(all.contains(&Damage(ENTITY1_DAMAGE)));
        assert!(component_manager.all_mut::<Arrows>().is_empty());
        assert!(component_manager.all_mut::<Points>().is_empty());
    }

    #[test]
    fn owners() {
        let component_manager = setup();
        let owners = component_manager.owners::<Health>();
        assert_eq!(owners.len(), 2);
        assert!(owners.contains(&ENTITY0));
        assert!(owners.contains(&ENTITY1));
        let owners = component_manager.owners::<Damage>();
        assert_eq!(owners.len(), 2);
        assert!(owners.contains(&ENTITY0));
        assert!(owners.contains(&ENTITY1));
        assert!(component_manager.owners::<Arrows>().is_empty());
        assert!(component_manager.owners::<Points>().is_empty());
    }

    #[test]
    fn remove() {
        let mut component_manager = setup();
        component_manager.remove::<Health>(ENTITY0);
        assert!(component_manager.get::<Health>(ENTITY0).is_none());
        assert!(component_manager.get::<Damage>(ENTITY0).is_some());
        component_manager.remove::<Damage>(ENTITY0);
        assert!(component_manager.get::<Health>(ENTITY0).is_none());
        assert!(component_manager.get::<Damage>(ENTITY0).is_none());
        component_manager.remove::<Arrows>(ENTITY0);
        component_manager.remove::<Points>(ENTITY0);
        component_manager.remove::<Health>(ENTITY1);
        assert!(component_manager.get::<Health>(ENTITY1).is_none());
        assert!(component_manager.get::<Damage>(ENTITY1).is_some());
        component_manager.remove::<Damage>(ENTITY1);
        assert!(component_manager.get::<Health>(ENTITY1).is_none());
        assert!(component_manager.get::<Damage>(ENTITY1).is_none());
        component_manager.remove::<Arrows>(ENTITY1);
        component_manager.remove::<Points>(ENTITY1);
        component_manager.remove::<Health>(ENTITY2);
        component_manager.remove::<Damage>(ENTITY2);
        component_manager.remove::<Arrows>(ENTITY2);
        component_manager.remove::<Points>(ENTITY2);
    }

    #[test]
    fn remove_all() {
        let mut component_manager = setup();
        component_manager.remove_all(ENTITY0);
        assert!(component_manager.get::<Health>(ENTITY0).is_none());
        assert!(component_manager.get::<Damage>(ENTITY0).is_none());
        assert!(component_manager.get::<Arrows>(ENTITY0).is_none());
        assert!(component_manager.get::<Points>(ENTITY0).is_none());
        component_manager.remove_all(ENTITY1);
        assert!(component_manager.get::<Health>(ENTITY1).is_none());
        assert!(component_manager.get::<Damage>(ENTITY1).is_none());
        assert!(component_manager.get::<Arrows>(ENTITY1).is_none());
        assert!(component_manager.get::<Points>(ENTITY1).is_none());
        component_manager.remove_all(ENTITY2);
    }
}
