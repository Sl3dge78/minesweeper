use std::fs::{self, Metadata};
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::time::SystemTime;
use crate::renderer::Texture;

pub enum ResourceKind {
    Texture(Texture),
}

impl ResourceKind {
    pub fn as_texture(&self) -> &Texture {
        match self {
            ResourceKind::Texture(t) => t,
            _ => panic!(),
        }
    }
}

pub struct Entry {
    res: ResourceKind,
    time: SystemTime,
}

impl Entry {
    pub fn new(res: ResourceKind, meta: Metadata) -> Entry {
        Entry {
            res,
            time: meta.modified().unwrap()
        }
    }

    pub fn should_reload(&self, meta: Metadata) -> bool {
        self.time < meta.modified().unwrap()
    }
}

pub struct Resources {
    resources: HashMap<String, Entry>,
}

impl Resources {
    pub fn new() -> Resources {
        let mut result = Resources {
            resources: HashMap::new()
        };

        for entry in fs::read_dir("./res/")
                .unwrap()
                .filter_map(|e| e.ok())
        {  
            let Ok(metadata) = entry.metadata() else {
                continue;
            };
            let file = entry.path();
            let key = file.to_str().unwrap();
            let Some(res) = Resources::load_resource(&file) else { 
                println!("Unable to load resource from {}", key);
                continue;
            };
            result.resources.insert(key.to_string(), Entry::new(res, metadata));
        }

        result
    }

    fn load_resource(path: &Path) -> Option<ResourceKind> {
        let ext = path.extension().unwrap().to_str().unwrap();
        match ext {
            Texture::EXT => Texture::load_resource(path.to_str().unwrap()),
            _ => None
        }
    }

    pub fn get(&self, path: &str) -> &ResourceKind {
        &self.resources.get(path).unwrap().res
    }

    pub fn check_for_changes(&mut self) {
        for e in &mut self.resources {
            let path = PathBuf::from(e.0);
            let Ok(metadata) = fs::metadata(&path) else {
                println!("Unable to reopen file {}", path.display());
                continue;
            };
            if !e.1.should_reload(metadata) { continue; }
            let Some(res) = Resources::load_resource(&path) else {
                println!("Unable to reload file {}", path.display());
                continue;
            };
            e.1.res = res;
        }
    }
}

pub trait ResourceLoader {
    const EXT: &'static str;
    fn load_resource(path: &str) -> Option<ResourceKind>;
}

