use std::sync::Mutex;

use crate::cache_backend::dummy_backend::DummyCacheBackend;
use crate::settings::Settings;

pub struct AppState {
    pub tera: tera::Tera,
    pub settings: Settings,
    pub cache: Mutex<DummyCacheBackend>,
}

impl AppState {
    pub fn new(settings: Settings) -> Self {
        let tera = crate::templates::init_templates();
        let cache = DummyCacheBackend::new();
        AppState {
            tera,
            settings,
            cache: Mutex::new(cache),
        }
    }
}
