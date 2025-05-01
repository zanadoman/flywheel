use std::hash::Hasher;

#[derive(Default)]
pub struct NoopHasher(u64);

impl Hasher for NoopHasher {
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
        let mut noop_hasher = NoopHasher::default();
        TypeId::of::<i32>().hash(&mut noop_hasher);
        let _ = noop_hasher.finish();
    }
}
