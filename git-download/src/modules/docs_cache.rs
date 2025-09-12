use std::{env::current_dir, fs::remove_dir_all};

use crate::modules::logger::Logger;
use anyhow::Result;

#[derive(Debug)]
pub struct DocsCache {
    pub name: String,
}

impl DocsCache {
    fn new() -> Self {
        Self {
            name: "docs_cache".to_string(),
        }
    }

    pub fn clean(self) -> Result<Self> {
        let name = &self.name;
        let project_dir = current_dir().unwrap();
        let path = project_dir.join(name);
        Logger::log(format!("Limpando {path:?} directory").as_str());
        if path.exists() {
            remove_dir_all(path)?;
        }

        Ok(self)
    }
}

impl Default for DocsCache {
    fn default() -> Self {
        Self::new()
    }
}
