use core::{
    any::{Any, TypeId},
    hash::BuildHasherDefault,
};
use std::collections::HashMap;

use super::noop_hasher::NoopHasher;

pub struct ResourceManager(
    HashMap<TypeId, Box<dyn Any>, BuildHasherDefault<NoopHasher>>,
);

impl ResourceManager {
    #[must_use]
    pub fn new() -> Self {
        Self(HashMap::default())
    }

    pub fn insert<T: 'static>(&mut self, resource: T) -> Option<T> {
        self.0
            .insert(TypeId::of::<T>(), Box::new(resource))
            .map(|r| *r.downcast().unwrap())
    }

    #[must_use]
    pub fn has<T: 'static>(&self) -> bool {
        self.0.contains_key(&TypeId::of::<T>())
    }

    #[must_use]
    pub fn get<T: 'static>(&self) -> Option<&T> {
        Some(
            (**self.0.get(&TypeId::of::<T>())?)
                .downcast_ref::<T>()
                .unwrap(),
        )
    }

    #[must_use]
    pub fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
        Some(
            (**self.0.get_mut(&TypeId::of::<T>())?)
                .downcast_mut()
                .unwrap(),
        )
    }

    pub fn remove<T: 'static>(&mut self) -> Option<T> {
        self.0
            .remove(&TypeId::of::<T>())
            .map(|r| *r.downcast().unwrap())
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const I8_VALUE: i8 = 8;
    const I16_VALUE: i16 = 16;
    const I32_VALUE: i32 = 32;

    #[must_use]
    fn setup() -> ResourceManager {
        let mut resource_manager = ResourceManager::new();
        assert!(resource_manager.insert(I8_VALUE).is_none());
        assert!(resource_manager.insert(I16_VALUE).is_none());
        resource_manager
    }

    #[test]
    fn insert() {
        let mut resource_manager = setup();
        let value = I8_VALUE * 3;
        assert_eq!(resource_manager.insert(value), Some(I8_VALUE));
        assert_eq!(resource_manager.get(), Some(&value));
        let value = I16_VALUE * 3;
        assert_eq!(resource_manager.insert(value), Some(I16_VALUE));
        assert_eq!(resource_manager.get(), Some(&value));
        assert!(resource_manager.insert(I32_VALUE).is_none());
        assert_eq!(resource_manager.get(), Some(&I32_VALUE));
    }

    #[test]
    fn has() {
        let resource_manager = setup();
        assert!(resource_manager.has::<i8>());
        assert!(resource_manager.has::<i16>());
        assert!(!resource_manager.has::<i32>());
    }

    #[test]
    fn get() {
        let resource_manager = setup();
        assert_eq!(resource_manager.get(), Some(&I8_VALUE));
        assert_eq!(resource_manager.get(), Some(&I16_VALUE));
        assert!(resource_manager.get::<i32>().is_none());
    }

    #[test]
    fn get_mut() {
        let mut resource_manager = setup();
        let mut value = I8_VALUE;
        assert_eq!(resource_manager.get_mut(), Some(&mut value));
        let mut value = I16_VALUE;
        assert_eq!(resource_manager.get_mut(), Some(&mut value));
        assert!(resource_manager.get_mut::<i32>().is_none());
    }

    #[test]
    fn remove() {
        let mut resource_manager = setup();
        assert!(resource_manager.remove::<i32>().is_none());
        assert_eq!(resource_manager.get(), Some(&I8_VALUE));
        assert_eq!(resource_manager.remove(), Some(I8_VALUE));
        assert!(!resource_manager.has::<i8>());
        assert!(resource_manager.remove::<i8>().is_none());
        assert_eq!(resource_manager.get(), Some(&I16_VALUE));
        assert_eq!(resource_manager.remove(), Some(I16_VALUE));
        assert!(!resource_manager.has::<i16>());
        assert!(resource_manager.remove::<i16>().is_none());
        assert!(resource_manager.remove::<i32>().is_none());
    }

    #[test]
    fn clear() {
        let mut resource_manager = setup();
        resource_manager.clear();
        assert!(!resource_manager.has::<i8>());
        assert!(!resource_manager.has::<i16>());
    }
}
