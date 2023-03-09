use std::fs;
use std::path::Path;

use crate::renderer::Texture;

pub struct Resources {
    all: Vec<Resource>,
}

pub enum ResourceData {
    Texture(Texture),
}

pub struct Resource {
    kind: ResourceData,
    key: String,
}

impl Resource {
    pub fn new(path: &Path) -> Option<Resource> {
        let ext = path.extension()?.to_str()?;
        let path_str = path.to_str()?;
        let kind = match ext {
            "png" => {
                ResourceData::Texture(Texture::from_image(path.to_str()?).expect(&format!("Unable to load {}", path.display())))
            },
            _ => return None,
        };
        Some(Resource {
            kind,
            key: path_str.to_owned()
        })
    }
}

impl Resources {
    pub fn new() -> Resources {
        let mut result = Resources {
            all : Vec::new()
        };

        for entry in fs::read_dir("./res/")
                .unwrap()
                .filter_map(|e| e.ok())
                .map(|e| e.path())
                .filter(|e| e.is_file()) {
            println!("{}", entry.display());
            if let Some(r) = Resource::new(&entry) {
                result.all.push(r);
            }
        }

        result
    }

    pub fn get_texture(&self) -> &Texture {
        match &self.all[0].kind {
            ResourceData::Texture(tex) => &tex,
            _ => todo!()

        }
    }
}
