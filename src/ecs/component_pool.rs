use super::Entity;

pub trait SparseSet {
    fn owners(&self) -> &[Entity];

    fn remove(&mut self, owner: Entity);
}

pub struct ComponentPool<T, const N: usize> {
    sparse: [Option<usize>; N],
    owners: Vec<Entity>,
    dense: Vec<T>,
}

impl<T, const N: usize> ComponentPool<T, N> {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            sparse: [None; N],
            owners: Vec::new(),
            dense: Vec::new(),
        }
    }

    pub fn push(&mut self, owner: Entity, component: T) -> Result<(), T> {
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
    #[allow(clippy::missing_const_for_fn)]
    pub fn all(&self) -> &[T] {
        &self.dense
    }

    #[must_use]
    #[allow(clippy::missing_const_for_fn)]
    pub fn all_mut(&mut self) -> &mut [T] {
        &mut self.dense
    }
}

impl<T, const N: usize> SparseSet for ComponentPool<T, N> {
    fn owners(&self) -> &[Entity] {
        &self.owners
    }

    fn remove(&mut self, owner: Entity) {
        if let Some(Some(index)) = self.sparse.get(owner.id()) {
            let last_index = self.dense.len() - 1;
            if *index != last_index {
                self.dense.swap(*index, last_index);
                self.owners.swap(*index, last_index);
                self.sparse[self.owners[*index].id()] = Some(*index);
            }
            self.dense.pop();
            self.owners.pop();
            self.sparse[owner.id()] = None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ENTITY0: Entity = Entity::new(0);
    const ENTITY1: Entity = Entity::new(1);
    const ENTITY2: Entity = Entity::new(2);

    fn setup() -> ComponentPool<usize, 2> {
        let mut component_pool = ComponentPool::new();
        component_pool.push(ENTITY0, ENTITY0.id()).unwrap();
        component_pool.push(ENTITY1, ENTITY1.id()).unwrap();
        component_pool
    }

    #[test]
    #[should_panic]
    fn push() {
        let mut component_pool = setup();
        component_pool.push(ENTITY2, ENTITY2.id()).unwrap();
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
        let mut component_pool = setup();
        let all = component_pool.all();
        assert_eq!(all.len(), 2);
        assert!(all.contains(&ENTITY0.id()));
        assert!(all.contains(&ENTITY1.id()));
        component_pool.remove(ENTITY0);
        let all = component_pool.all();
        assert_eq!(all.len(), 1);
        assert!(!all.contains(&ENTITY0.id()));
        assert!(all.contains(&ENTITY1.id()));
    }

    #[test]
    fn all_mut() {
        let mut component_pool = setup();
        let all = component_pool.all_mut();
        assert_eq!(all.len(), 2);
        assert!(all.contains(&ENTITY0.id()));
        assert!(all.contains(&ENTITY1.id()));
        component_pool.remove(ENTITY0);
        let all = component_pool.all_mut();
        assert_eq!(all.len(), 1);
        assert!(!all.contains(&ENTITY0.id()));
        assert!(all.contains(&ENTITY1.id()));
    }

    #[test]
    fn owners() {
        let mut component_pool = setup();
        let owners = component_pool.owners();
        assert_eq!(owners.len(), 2);
        assert!(owners.contains(&ENTITY0));
        assert!(owners.contains(&ENTITY1));
        component_pool.remove(ENTITY0);
        let owners = component_pool.owners();
        assert_eq!(owners.len(), 1);
        assert!(!owners.contains(&ENTITY0));
        assert!(owners.contains(&ENTITY1));
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
