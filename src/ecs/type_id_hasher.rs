use std::hash::Hasher;

#[derive(Default)]
pub struct TypeIdHasher(u64);

impl Hasher for TypeIdHasher {
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, bytes: &[u8]) {
        self.0 = u64::from_ne_bytes(bytes.try_into().unwrap());
    }
}

#[cfg(test)]
pub mod tests {
    use std::{any::TypeId, hash::Hash};

    use super::*;

    #[test]
    fn test() {
        let mut type_id_hasher = TypeIdHasher::default();
        TypeId::of::<i32>().hash(&mut type_id_hasher);
        let _ = type_id_hasher.finish();
    }
}
