use std::{
    any::{Any, TypeId},
    collections::HashMap,
    hash::BuildHasherDefault,
};

use super::{
    Entity,
    component_pool::{AnyComponentPool, ComponentPool},
    noop_hasher::NoopHasher,
};

pub struct ComponentManager {
    ids: HashMap<TypeId, usize, BuildHasherDefault<NoopHasher>>,
    pools: Vec<Box<dyn AnyComponentPool>>,
}

impl ComponentManager {
    #[must_use]
    pub fn new() -> Self {
        Self {
            ids: HashMap::default(),
            pools: Vec::new(),
        }
    }

    #[must_use]
    pub fn id<T: 'static>(&self) -> Option<usize> {
        self.ids.get(&TypeId::of::<T>()).copied()
    }

    #[must_use]
    pub fn register<T: 'static>(&mut self) -> usize {
        *self.ids.entry(TypeId::of::<T>()).or_insert_with(|| {
            let id = self.pools.len();
            self.pools.push(Box::new(ComponentPool::<T>::new()));
            id
        })
    }

    pub fn insert<T: 'static>(
        &mut self,
        owner: Entity,
        component: T,
    ) -> Option<T> {
        if let Some(pool) = self.pool_mut() {
            pool.insert(owner, component)
        } else {
            self.ids
                .try_insert(TypeId::of::<T>(), self.pools.len())
                .unwrap();
            self.pools.push(Box::new(ComponentPool::new_with_initial(
                owner, component,
            )));
            None
        }
    }

    #[must_use]
    pub fn has<T: 'static>(&self, owner: Entity) -> bool {
        self.pool::<T>().is_some_and(|p| p.has(owner))
    }

    #[must_use]
    pub fn get<T: 'static>(&self, owner: Entity) -> Option<&T> {
        self.pool()?.get(owner)
    }

    #[must_use]
    pub fn get_mut<T: 'static>(&mut self, owner: Entity) -> Option<&mut T> {
        self.pool_mut()?.get_mut(owner)
    }

    #[must_use]
    pub fn all<T: 'static>(&self) -> &[T] {
        self.pool().map_or(&[], |p| p.all())
    }

    #[must_use]
    pub fn all_mut<T: 'static>(&mut self) -> &mut [T] {
        self.pool_mut().map_or(&mut [], |p| p.all_mut())
    }

    pub fn remove<T: 'static>(&mut self, owner: Entity) -> Option<T> {
        self.pool_mut()?.remove(owner)
    }

    #[must_use]
    pub fn owners<T: 'static>(&self) -> &[Entity] {
        self.pool::<T>().map_or(&[], |p| p.owners())
    }

    pub fn destroy(&mut self, owner: Entity) {
        for pool in &mut self.pools {
            pool.destroy(owner);
        }
    }

    pub fn clear(&mut self) {
        for pool in &mut self.pools {
            pool.clear();
        }
    }

    #[must_use]
    fn pool<T: 'static>(&self) -> Option<&ComponentPool<T>> {
        Some(
            (self.pools[self.id::<T>()?].as_ref() as &dyn Any)
                .downcast_ref()
                .unwrap(),
        )
    }

    #[must_use]
    fn pool_mut<T: 'static>(&mut self) -> Option<&mut ComponentPool<T>> {
        let id = self.id::<T>()?;
        Some(
            (self.pools[id].as_mut() as &mut dyn Any)
                .downcast_mut()
                .unwrap(),
        )
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
    struct Shield(u8);

    const ENTITY0: Entity = Entity::new(0);
    const ENTITY0_HEALTH: Health = Health(1);
    const ENTITY0_DAMAGE: Damage = Damage(2);
    const ENTITY0_SHIELD: Shield = Shield(3);
    const ENTITY1: Entity = Entity::new(1);
    const ENTITY1_HEALTH: Health = Health(4);
    const ENTITY1_DAMAGE: Damage = Damage(5);
    const ENTITY1_SHIELD: Shield = Shield(6);
    const ENTITY2: Entity = Entity::new(2);
    const ENTITY2_HEALTH: Health = Health(7);
    const ENTITY2_DAMAGE: Damage = Damage(8);
    const ENTITY2_SHIELD: Shield = Shield(9);

    #[must_use]
    fn setup() -> ComponentManager {
        let mut component_manager = ComponentManager::new();
        assert!(component_manager.insert(ENTITY0, ENTITY0_HEALTH).is_none());
        assert!(component_manager.insert(ENTITY0, ENTITY0_DAMAGE).is_none());
        assert!(component_manager.insert(ENTITY1, ENTITY1_HEALTH).is_none());
        assert!(component_manager.insert(ENTITY1, ENTITY1_DAMAGE).is_none());
        component_manager
    }

    #[test]
    fn id() {
        let component_manager = setup();
        assert_eq!(component_manager.id::<Health>().unwrap(), 0);
        assert_eq!(component_manager.id::<Damage>().unwrap(), 1);
        assert!(component_manager.id::<Shield>().is_none());
    }

    #[test]
    fn register() {
        let mut component_manager = setup();
        assert_eq!(component_manager.register::<Health>(), 0);
        assert_eq!(component_manager.register::<Damage>(), 1);
        assert_eq!(component_manager.register::<Shield>(), 2);
    }

    #[test]
    fn insert() {
        let mut component_manager = setup();

        let value = ENTITY0_HEALTH.0 * 10;
        assert_eq!(
            component_manager.insert(ENTITY0, Health(value)),
            Some(ENTITY0_HEALTH)
        );
        assert_eq!(component_manager.get(ENTITY0), Some(&Health(value)));
        let value = ENTITY0_DAMAGE.0 * 10;
        assert_eq!(
            component_manager.insert(ENTITY0, Damage(value)),
            Some(ENTITY0_DAMAGE)
        );
        assert_eq!(component_manager.get(ENTITY0), Some(&Damage(value)));
        assert!(component_manager.insert(ENTITY0, ENTITY0_SHIELD).is_none());
        assert_eq!(component_manager.get(ENTITY0), Some(&ENTITY0_SHIELD));
        let value = ENTITY1_HEALTH.0 * 10;
        assert_eq!(
            component_manager.insert(ENTITY1, Health(value)),
            Some(ENTITY1_HEALTH)
        );
        assert_eq!(component_manager.get(ENTITY1), Some(&Health(value)));
        let value = ENTITY1_DAMAGE.0 * 10;
        assert_eq!(
            component_manager.insert(ENTITY1, Damage(value)),
            Some(ENTITY1_DAMAGE)
        );
        assert_eq!(component_manager.get(ENTITY1), Some(&Damage(value)));
        assert!(component_manager.insert(ENTITY1, ENTITY1_SHIELD).is_none());
        assert_eq!(component_manager.get(ENTITY1), Some(&ENTITY1_SHIELD));
        assert!(component_manager.insert(ENTITY2, ENTITY2_HEALTH).is_none());
        assert_eq!(component_manager.get(ENTITY2), Some(&ENTITY2_HEALTH));
        assert!(component_manager.insert(ENTITY2, ENTITY2_DAMAGE).is_none());
        assert_eq!(component_manager.get(ENTITY2), Some(&ENTITY2_DAMAGE));
        assert!(component_manager.insert(ENTITY2, ENTITY2_SHIELD).is_none());
        assert_eq!(component_manager.get(ENTITY2), Some(&ENTITY2_SHIELD));
    }

    #[test]
    fn has() {
        let component_manager = setup();
        assert!(component_manager.has::<Health>(ENTITY0));
        assert!(component_manager.has::<Damage>(ENTITY0));
        assert!(!component_manager.has::<Shield>(ENTITY0));
        assert!(component_manager.has::<Health>(ENTITY1));
        assert!(component_manager.has::<Damage>(ENTITY1));
        assert!(!component_manager.has::<Shield>(ENTITY1));
        assert!(!component_manager.has::<Health>(ENTITY2));
        assert!(!component_manager.has::<Damage>(ENTITY2));
        assert!(!component_manager.has::<Shield>(ENTITY2));
    }

    #[test]
    fn get() {
        let component_manager = setup();
        assert_eq!(component_manager.get(ENTITY0), Some(&ENTITY0_HEALTH));
        assert_eq!(component_manager.get(ENTITY0), Some(&ENTITY0_DAMAGE));
        assert!(component_manager.get::<Shield>(ENTITY0).is_none());
        assert_eq!(component_manager.get(ENTITY1), Some(&ENTITY1_HEALTH));
        assert_eq!(component_manager.get(ENTITY1), Some(&ENTITY1_DAMAGE));
        assert!(component_manager.get::<Shield>(ENTITY1).is_none());
        assert!(component_manager.get::<Health>(ENTITY2).is_none());
        assert!(component_manager.get::<Damage>(ENTITY2).is_none());
        assert!(component_manager.get::<Shield>(ENTITY2).is_none());
    }

    #[test]
    fn get_mut() {
        let mut component_manager = setup();
        let mut component = ENTITY0_HEALTH;
        assert_eq!(component_manager.get_mut(ENTITY0), Some(&mut component));
        let mut component = ENTITY0_DAMAGE;
        assert_eq!(component_manager.get_mut(ENTITY0), Some(&mut component));
        assert!(component_manager.get_mut::<Shield>(ENTITY0).is_none());
        let mut component = ENTITY1_HEALTH;
        assert_eq!(component_manager.get_mut(ENTITY1), Some(&mut component));
        let mut component = ENTITY1_DAMAGE;
        assert_eq!(component_manager.get_mut(ENTITY1), Some(&mut component));
        assert!(component_manager.get_mut::<Shield>(ENTITY1).is_none());
        assert!(component_manager.get_mut::<Health>(ENTITY2).is_none());
        assert!(component_manager.get_mut::<Damage>(ENTITY2).is_none());
        assert!(component_manager.get_mut::<Shield>(ENTITY2).is_none());
    }

    #[test]
    fn all() {
        let component_manager = setup();
        assert_eq!(component_manager.all::<Health>().len(), 2);
        assert!(component_manager.all().contains(&ENTITY0_HEALTH));
        assert!(component_manager.all().contains(&ENTITY1_HEALTH));
        assert_eq!(component_manager.all::<Damage>().len(), 2);
        assert!(component_manager.all().contains(&ENTITY0_DAMAGE));
        assert!(component_manager.all().contains(&ENTITY1_DAMAGE));
        assert!(component_manager.all::<Shield>().is_empty());
    }

    #[test]
    fn all_mut() {
        let mut component_manager = setup();
        assert_eq!(component_manager.all_mut::<Health>().len(), 2);
        assert!(component_manager.all_mut().contains(&ENTITY0_HEALTH));
        assert!(component_manager.all_mut().contains(&ENTITY1_HEALTH));
        assert_eq!(component_manager.all_mut::<Damage>().len(), 2);
        assert!(component_manager.all_mut().contains(&ENTITY0_DAMAGE));
        assert!(component_manager.all_mut().contains(&ENTITY1_DAMAGE));
        assert!(component_manager.all_mut::<Shield>().is_empty());
    }

    #[test]
    #[allow(clippy::cognitive_complexity)]
    fn remove() {
        let mut component_manager = setup();
        assert!(component_manager.remove::<Health>(ENTITY2).is_none());
        assert!(component_manager.remove::<Damage>(ENTITY2).is_none());
        assert!(component_manager.remove::<Shield>(ENTITY2).is_none());
        assert_eq!(component_manager.get(ENTITY0), Some(&ENTITY0_HEALTH));
        assert_eq!(component_manager.get(ENTITY0), Some(&ENTITY0_DAMAGE));
        assert!(!component_manager.has::<Shield>(ENTITY0));
        assert_eq!(component_manager.remove(ENTITY0), Some(ENTITY0_HEALTH));
        assert_eq!(component_manager.remove(ENTITY0), Some(ENTITY0_DAMAGE));
        assert!(component_manager.remove::<Shield>(ENTITY0).is_none());
        assert!(!component_manager.has::<Health>(ENTITY0));
        assert!(!component_manager.has::<Damage>(ENTITY0));
        assert!(!component_manager.has::<Shield>(ENTITY0));
        assert!(component_manager.remove::<Health>(ENTITY0).is_none());
        assert!(component_manager.remove::<Damage>(ENTITY0).is_none());
        assert!(component_manager.remove::<Shield>(ENTITY0).is_none());
        assert_eq!(component_manager.get(ENTITY1), Some(&ENTITY1_HEALTH));
        assert_eq!(component_manager.get(ENTITY1), Some(&ENTITY1_DAMAGE));
        assert!(!component_manager.has::<Shield>(ENTITY1));
        assert_eq!(component_manager.remove(ENTITY1), Some(ENTITY1_HEALTH));
        assert_eq!(component_manager.remove(ENTITY1), Some(ENTITY1_DAMAGE));
        assert!(component_manager.remove::<Shield>(ENTITY1).is_none());
        assert!(!component_manager.has::<Health>(ENTITY1));
        assert!(!component_manager.has::<Damage>(ENTITY1));
        assert!(!component_manager.has::<Shield>(ENTITY1));
        assert!(component_manager.remove::<Health>(ENTITY1).is_none());
        assert!(component_manager.remove::<Damage>(ENTITY1).is_none());
        assert!(component_manager.remove::<Shield>(ENTITY1).is_none());
        assert!(component_manager.remove::<Health>(ENTITY2).is_none());
        assert!(component_manager.remove::<Damage>(ENTITY2).is_none());
        assert!(component_manager.remove::<Shield>(ENTITY2).is_none());
    }

    #[test]
    fn owners() {
        let component_manager = setup();
        assert_eq!(component_manager.owners::<Health>().len(), 2);
        assert!(component_manager.owners::<Health>().contains(&ENTITY0));
        assert!(component_manager.owners::<Health>().contains(&ENTITY1));
        assert_eq!(component_manager.owners::<Damage>().len(), 2);
        assert!(component_manager.owners::<Damage>().contains(&ENTITY0));
        assert!(component_manager.owners::<Damage>().contains(&ENTITY1));
        assert!(component_manager.owners::<Shield>().is_empty());
    }

    #[test]
    fn destroy() {
        let mut component_manager = setup();
        component_manager.destroy(ENTITY2);
        assert_eq!(component_manager.get(ENTITY0), Some(&ENTITY0_HEALTH));
        assert_eq!(component_manager.get(ENTITY0), Some(&ENTITY0_DAMAGE));
        assert!(!component_manager.has::<Shield>(ENTITY0));
        component_manager.destroy(ENTITY0);
        assert!(!component_manager.has::<Health>(ENTITY0));
        assert!(!component_manager.has::<Damage>(ENTITY0));
        assert!(!component_manager.has::<Shield>(ENTITY0));
        component_manager.destroy(ENTITY0);
        assert_eq!(component_manager.get(ENTITY1), Some(&ENTITY1_HEALTH));
        assert_eq!(component_manager.get(ENTITY1), Some(&ENTITY1_DAMAGE));
        assert!(!component_manager.has::<Shield>(ENTITY1));
        component_manager.destroy(ENTITY1);
        assert!(!component_manager.has::<Health>(ENTITY1));
        assert!(!component_manager.has::<Damage>(ENTITY1));
        assert!(!component_manager.has::<Shield>(ENTITY1));
        component_manager.destroy(ENTITY1);
        component_manager.destroy(ENTITY2);
    }

    #[test]
    fn clear() {
        let mut component_manager = setup();
        component_manager.clear();
        assert!(component_manager.all::<Health>().is_empty());
        assert!(component_manager.all::<Damage>().is_empty());
        assert!(component_manager.all::<Shield>().is_empty());
    }
}
