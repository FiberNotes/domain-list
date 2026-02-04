use crate::common::GeoSite;
use std::cell::RefCell;
use std::collections::HashMap;
use std::io;
use std::path::{Path, PathBuf};

pub struct Cache {
    data: RefCell<HashMap<PathBuf, GeoSite>>,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            data: RefCell::new(HashMap::new()),
        }
    }

    pub fn get_or_load<F>(&self, path: impl AsRef<Path>, loader: F) -> Result<GeoSite, io::Error>
    where
        F: FnOnce(&Path) -> Result<GeoSite, io::Error>,
    {
        let path = path.as_ref().to_path_buf();

        // Проверяем, есть ли уже в кеше
        {
            let cache = self.data.borrow();
            if let Some(cached) = cache.get(&path) {
                return Ok(cached.clone());
            }
        }

        // Если нет, загружаем
        let loaded = loader(&path)?;
        self.data.borrow_mut().insert(path, loaded.clone());
        Ok(loaded)
    }
}
