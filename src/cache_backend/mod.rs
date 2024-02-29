pub mod dummy_backend;
pub mod local_memory_backend;
mod test_local_memory_backend;

pub trait CacheBackend {
    fn set(&mut self, id: &str, value: &str);
    fn get(&mut self, id: &str) -> Option<&String>;
}
