use std::{any::Any, mem};

use super::Entity;

pub trait AnyComponentPool: Any {
    #[must_use]
    fn owners(&self) -> &[Entity];

    fn destroy(&mut self, owner: Entity);

    fn clear(&mut self);
}

pub struct ComponentPool<T> {
    dense: Vec<T>,
    owners: Vec<Entity>,
    sparse: Vec<Option<usize>>,
}

impl<T> ComponentPool<T> {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            dense: Vec::new(),
            owners: Vec::new(),
            sparse: Vec::new(),
        }
    }

    #[must_use]
    pub fn new_with_initial(owner: Entity, component: T) -> Self {
        Self {
            dense: vec![component],
            owners: vec![owner],
            sparse: vec![Some(owner.id())],
        }
    }

    pub fn insert(&mut self, owner: Entity, component: T) -> Option<T> {
        if self.sparse.len() <= owner.id() {
            self.sparse.resize(owner.id() + 1, None);
        }
        if let Some(index) = self.sparse[owner.id()] {
            Some(mem::replace(&mut self.dense[index], component))
        } else {
            self.sparse[owner.id()] = Some(self.dense.len());
            self.dense.push(component);
            self.owners.push(owner);
            None
        }
    }

    #[must_use]
    pub fn has(&self, owner: Entity) -> bool {
        self.sparse.get(owner.id()).is_some_and(Option::is_some)
    }

    #[must_use]
    pub fn get(&self, owner: Entity) -> Option<&T> {
        Some(&self.dense[(*self.sparse.get(owner.id())?)?])
    }

    #[must_use]
    pub fn get_mut(&mut self, owner: Entity) -> Option<&mut T> {
        Some(&mut self.dense[(*self.sparse.get(owner.id())?)?])
    }

    #[must_use]
    pub fn all(&self) -> &[T] {
        &self.dense
    }

    #[must_use]
    pub fn all_mut(&mut self) -> &mut [T] {
        &mut self.dense
    }

    pub fn remove(&mut self, owner: Entity) -> Option<T> {
        let index = (*self.sparse.get(owner.id())?)?;
        self.sparse[owner.id()] = None;
        Some(if index == self.dense.len() - 1 {
            self.owners.pop().unwrap();
            self.dense.pop().unwrap()
        } else {
            self.owners.swap_remove(index);
            let swapped = self.owners[index].id();
            self.sparse[swapped] = Some(index);
            self.dense.swap_remove(index)
        })
    }
}

impl<T: 'static> AnyComponentPool for ComponentPool<T> {
    fn owners(&self) -> &[Entity] {
        &self.owners
    }

    fn destroy(&mut self, owner: Entity) {
        self.remove(owner);
    }

    fn clear(&mut self) {
        self.sparse.fill(None);
        self.owners.clear();
        self.dense.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ENTITY0: Entity = Entity::new(0);
    const ENTITY1: Entity = Entity::new(1);
    const ENTITY2: Entity = Entity::new(2);

    #[must_use]
    fn setup() -> ComponentPool<usize> {
        let mut component_pool =
            ComponentPool::new_with_initial(ENTITY0, ENTITY0.id());
        assert!(component_pool.insert(ENTITY1, ENTITY1.id()).is_none());
        component_pool
    }

    #[test]
    fn new() {
        const COMPONENT_POOL: ComponentPool<usize> = ComponentPool::new();
        assert!(COMPONENT_POOL.all().is_empty());
        assert!(COMPONENT_POOL.owners().is_empty());
    }

    #[test]
    fn insert() {
        let mut component_pool = setup();
        let value = ENTITY0.id() + 3;
        assert_eq!(component_pool.insert(ENTITY0, value), Some(ENTITY0.id()));
        assert_eq!(component_pool.get(ENTITY0), Some(&value));
        let value = ENTITY1.id() + 3;
        assert_eq!(component_pool.insert(ENTITY1, value), Some(ENTITY1.id()));
        assert_eq!(component_pool.get(ENTITY1), Some(&value));
        assert!(component_pool.insert(ENTITY2, ENTITY2.id()).is_none());
        assert_eq!(component_pool.get(ENTITY2), Some(&ENTITY2.id()));
    }

    #[test]
    fn has() {
        let component_pool = setup();
        assert!(component_pool.has(ENTITY0));
        assert!(component_pool.has(ENTITY1));
        assert!(!component_pool.has(ENTITY2));
    }

    #[test]
    fn get() {
        let component_pool = setup();
        assert_eq!(component_pool.get(ENTITY0), Some(&ENTITY0.id()));
        assert_eq!(component_pool.get(ENTITY1), Some(&ENTITY1.id()));
        assert!(component_pool.get(ENTITY2).is_none());
    }

    #[test]
    fn get_mut() {
        let mut component_pool = setup();
        assert_eq!(component_pool.get_mut(ENTITY0), Some(&mut ENTITY0.id()));
        assert_eq!(component_pool.get_mut(ENTITY1), Some(&mut ENTITY1.id()));
        assert!(component_pool.get_mut(ENTITY2).is_none());
    }

    #[test]
    fn all() {
        let component_pool = setup();
        assert_eq!(component_pool.all().len(), 2);
        assert!(component_pool.all().contains(&ENTITY0.id()));
        assert!(component_pool.all().contains(&ENTITY1.id()));
    }

    #[test]
    fn all_mut() {
        let mut component_pool = setup();
        assert_eq!(component_pool.all_mut().len(), 2);
        assert!(component_pool.all_mut().contains(&ENTITY0.id()));
        assert!(component_pool.all_mut().contains(&ENTITY1.id()));
    }

    #[test]
    fn remove() {
        let mut component_pool = setup();
        assert!(component_pool.remove(ENTITY2).is_none());
        assert_eq!(component_pool.get(ENTITY0), Some(&ENTITY0.id()));
        assert_eq!(component_pool.remove(ENTITY0), Some(ENTITY0.id()));
        assert!(!component_pool.has(ENTITY0));
        assert!(component_pool.remove(ENTITY0).is_none());
        assert_eq!(component_pool.get(ENTITY1), Some(&ENTITY1.id()));
        assert_eq!(component_pool.remove(ENTITY1), Some(ENTITY1.id()));
        assert!(!component_pool.has(ENTITY1));
        assert!(component_pool.remove(ENTITY1).is_none());
        assert!(component_pool.remove(ENTITY2).is_none());

        let mut component_pool = setup();
        assert!(component_pool.remove(ENTITY2).is_none());
        assert_eq!(component_pool.get(ENTITY1), Some(&ENTITY1.id()));
        assert_eq!(component_pool.remove(ENTITY1), Some(ENTITY1.id()));
        assert!(!component_pool.has(ENTITY1));
        assert!(component_pool.remove(ENTITY1).is_none());
        assert_eq!(component_pool.get(ENTITY0), Some(&ENTITY0.id()));
        assert_eq!(component_pool.remove(ENTITY0), Some(ENTITY0.id()));
        assert!(!component_pool.has(ENTITY0));
        assert!(component_pool.remove(ENTITY0).is_none());
        assert!(component_pool.remove(ENTITY2).is_none());
    }

    #[test]
    fn owners() {
        let component_pool = setup();
        assert_eq!(component_pool.owners().len(), 2);
        assert!(component_pool.owners().contains(&ENTITY0));
        assert!(component_pool.owners().contains(&ENTITY1));
    }

    #[test]
    fn destroy() {
        let mut component_pool = setup();
        component_pool.destroy(ENTITY2);
        assert_eq!(component_pool.get(ENTITY0), Some(&ENTITY0.id()));
        component_pool.destroy(ENTITY0);
        assert!(!component_pool.has(ENTITY0));
        component_pool.destroy(ENTITY0);
        assert_eq!(component_pool.get(ENTITY1), Some(&ENTITY1.id()));
        component_pool.destroy(ENTITY1);
        assert!(!component_pool.has(ENTITY1));
        component_pool.destroy(ENTITY1);
        component_pool.destroy(ENTITY2);

        let mut component_pool = setup();
        component_pool.destroy(ENTITY2);
        assert_eq!(component_pool.get(ENTITY1), Some(&ENTITY1.id()));
        component_pool.destroy(ENTITY1);
        assert!(!component_pool.has(ENTITY1));
        component_pool.destroy(ENTITY1);
        assert_eq!(component_pool.get(ENTITY0), Some(&ENTITY0.id()));
        component_pool.destroy(ENTITY0);
        assert!(!component_pool.has(ENTITY0));
        component_pool.destroy(ENTITY0);
        component_pool.destroy(ENTITY2);
    }

    #[test]
    fn clear() {
        let mut component_pool = setup();
        component_pool.clear();
        assert!(!component_pool.has(ENTITY0));
        assert!(!component_pool.has(ENTITY1));
        assert!(component_pool.all().is_empty());
        assert!(component_pool.owners().is_empty());
    }
}
