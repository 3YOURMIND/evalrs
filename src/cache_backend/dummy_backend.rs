use crate::cache_backend::CacheBackend;

pub struct DummyCacheBackend {}

impl DummyCacheBackend {
    pub fn new() -> Self {
        DummyCacheBackend {}
    }
}

impl CacheBackend for DummyCacheBackend {
    fn set(self: &mut DummyCacheBackend, id: &str, value: &str) {}

    fn get(self: &mut DummyCacheBackend, id: &str) -> Option<&String> {
        None
    }
}
