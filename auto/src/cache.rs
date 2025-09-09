use std::{
    collections::HashMap, error::Error, fs::{self, File}
};

use ron::de::from_reader;
use serde::{Deserialize, Serialize};

use crate::{
    comandline::dotfiles_relative,
    step::{Step, StepStatus},
};

#[derive(Default, Serialize, Deserialize)]
pub struct ProgressCache {
    table: HashMap<Step, CacheEntry>,
}

impl ProgressCache {
    pub fn insert(&mut self, step: Step, value: CacheEntry) -> &Self {
        self.table.insert(step, value);
        self
    }

    pub fn entry(&self, step: Step) -> CacheEntry {
        self.table.get(&step).cloned().unwrap_or_default()
    }

    pub fn load() -> Result<ProgressCache, Box<dyn Error>> {
        let path = dotfiles_relative("auto/.cache/progress.ron");

        if !fs::exists(&path)? {
            let cache = ProgressCache::default();
            cache.save()?;
            return Ok(cache);
        }

        let f = File::open(path)?;

        let table: HashMap<Step, CacheEntry> = from_reader(f)?;

        Ok(ProgressCache { table })
    }

    pub fn save(&self) -> Result<(), Box<dyn Error>>{
        let parent = dotfiles_relative("auto/.cache");
        std::fs::create_dir_all(&parent)?;
        
        let path = parent.join("progress.ron");
        let file = File::options()
            .create(true)
            .truncate(true)
            .write(true)
            .open(path)?;

        let table = &self.table;
        ron::Options::default().to_io_writer_pretty(file, table, ron::ser::PrettyConfig::new())?;
        
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct CacheEntry {
    pub status: StepStatus,
}

impl CacheEntry {
    pub fn with_status(mut self, status: StepStatus) -> Self {
        self.status = status;
        self
    }
}
