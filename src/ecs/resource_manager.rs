use std::{
    any::{Any, TypeId},
    collections::HashMap,
    hash::BuildHasherDefault,
};

use super::type_id_hasher::TypeIdHasher;

pub struct ResourceManager(
    HashMap<TypeId, Box<dyn Any>, BuildHasherDefault<TypeIdHasher>>,
);

impl ResourceManager {
    #[must_use]
    pub fn new() -> Self {
        Self(HashMap::default())
    }

    pub fn add<T: 'static>(&mut self, resource: T) -> Option<T> {
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
        self.0.get(&TypeId::of::<T>())?.downcast_ref()
    }

    #[must_use]
    pub fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.0.get_mut(&TypeId::of::<T>())?.downcast_mut()
    }

    pub fn remove<T: 'static>(&mut self) {
        self.0.remove(&TypeId::of::<T>());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const I8_VALUE: i8 = i8::BITS as i8;
    const I16_VALUE: i16 = i16::BITS as i16;
    const I32_VALUE: i32 = i32::BITS as i32;
    const I64_VALUE: i64 = i64::BITS as i64;

    #[must_use]
    fn setup() -> ResourceManager {
        let mut resource_manager = ResourceManager::new();
        assert!(resource_manager.add::<i8>(I8_VALUE).is_none());
        assert!(resource_manager.add::<i16>(I16_VALUE).is_none());
        resource_manager
    }

    #[test]
    fn add() {
        let mut resource_manager = setup();
        assert_eq!(resource_manager.add::<i8>(I8_VALUE * 3), Some(I8_VALUE));
        assert_eq!(resource_manager.add::<i16>(I16_VALUE * 3), Some(I16_VALUE));
        assert!(resource_manager.add::<i32>(I32_VALUE).is_none());
        assert!(resource_manager.add::<i64>(I64_VALUE).is_none());
    }

    #[test]
    fn has() {
        let resource_manager = setup();
        assert!(resource_manager.has::<i8>());
        assert!(resource_manager.has::<i16>());
        assert!(!resource_manager.has::<i32>());
        assert!(!resource_manager.has::<i64>());
    }

    #[test]
    fn get() {
        let resource_manager = setup();
        assert_eq!(resource_manager.get::<i8>(), Some(&I8_VALUE));
        assert_eq!(resource_manager.get::<i16>(), Some(&I16_VALUE));
        assert!(resource_manager.get::<i32>().is_none());
        assert!(resource_manager.get::<i64>().is_none());
    }

    #[test]
    fn get_mut() {
        let mut resource_manager = setup();
        let mut value = I8_VALUE;
        assert_eq!(resource_manager.get_mut::<i8>(), Some(&mut value));
        let mut value = I16_VALUE;
        assert_eq!(resource_manager.get_mut::<i16>(), Some(&mut value));
        assert!(resource_manager.get_mut::<i32>().is_none());
        assert!(resource_manager.get_mut::<i64>().is_none());
    }

    #[test]
    fn remove() {
        let mut resource_manager = setup();
        resource_manager.remove::<i8>();
        assert!(resource_manager.get::<i8>().is_none());
        assert_eq!(resource_manager.get::<i16>(), Some(&I16_VALUE));
        resource_manager.remove::<i16>();
        assert!(resource_manager.get::<i16>().is_none());
        resource_manager.remove::<i8>();
    }
}
