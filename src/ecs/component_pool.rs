use core::any::Any;

use super::Entity;

pub(super) trait SparseSet: Any {
    #[must_use]
    fn owners(&self) -> &[Entity];

    fn remove(&mut self, owner: Entity);
}

pub(super) struct ComponentPool<T> {
    sparse: Vec<Option<usize>>,
    owners: Vec<Entity>,
    dense: Vec<T>,
}

impl<T> ComponentPool<T> {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            sparse: Vec::new(),
            owners: Vec::new(),
            dense: Vec::new(),
        }
    }

    pub fn add(&mut self, owner: Entity, component: T) -> Result<(), T> {
        if self.sparse.len() <= owner.id() {
            self.sparse.resize(owner.id() + 1, None);
        }
        if self.sparse.get(owner.id()) == Some(&None) {
            self.sparse[owner.id()] = Some(self.dense.len());
            self.owners.push(owner);
            self.dense.push(component);
            Ok(())
        } else {
            Err(component)
        }
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
}

impl<T> Default for ComponentPool<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: 'static> SparseSet for ComponentPool<T> {
    fn owners(&self) -> &[Entity] {
        &self.owners
    }

    fn remove(&mut self, owner: Entity) {
        let Some(Some(index)) = self.sparse.get(owner.id()) else {
            return;
        };
        let last_index = self.dense.len() - 1;
        if *index != last_index {
            self.dense.swap(*index, last_index);
            self.owners.swap(*index, last_index);
            let swapped = self.owners[*index].id();
            self.sparse[swapped] = Some(*index);
        }
        self.dense.pop();
        self.owners.pop();
        self.sparse[owner.id()] = None;
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
        let mut component_pool = ComponentPool::new();
        component_pool.add(ENTITY0, ENTITY0.id()).unwrap();
        component_pool.add(ENTITY1, ENTITY1.id()).unwrap();
        component_pool
    }

    #[test]
    fn add() {
        let mut component_pool = setup();
        assert_eq!(
            component_pool.add(ENTITY0, ENTITY0.id()),
            Err(ENTITY0.id())
        );
        assert_eq!(
            component_pool.add(ENTITY1, ENTITY1.id()),
            Err(ENTITY1.id())
        );
        assert!(component_pool.add(ENTITY2, ENTITY2.id()).is_ok());
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
    fn owners() {
        let component_pool = setup();
        assert_eq!(component_pool.owners().len(), 2);
        assert!(component_pool.owners().contains(&ENTITY0));
        assert!(component_pool.owners().contains(&ENTITY1));
    }

    #[test]
    fn remove() {
        let mut component_pool = setup();
        component_pool.remove(ENTITY0);
        assert!(component_pool.get(ENTITY0).is_none());
        assert_eq!(component_pool.get(ENTITY1), Some(&ENTITY1.id()));
        component_pool.remove(ENTITY1);
        assert!(component_pool.get(ENTITY1).is_none());
        component_pool.remove(ENTITY0);
        let mut component_pool = setup();
        component_pool.remove(ENTITY1);
        assert_eq!(component_pool.get(ENTITY0), Some(&ENTITY0.id()));
        assert!(component_pool.get(ENTITY1).is_none());
        component_pool.remove(ENTITY0);
        assert!(component_pool.get(ENTITY0).is_none());
        component_pool.remove(ENTITY1);
    }
}
