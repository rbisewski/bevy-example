use std::fs;

use bevy::prelude::Entity;

pub struct Decal {
    x: u32,
    y: u32,
    img: String,
    initialized: bool,
    entity: Entity,
}

impl Decal {
    pub fn new(x: u32, y: u32, img: String) -> Decal {
        Decal {
            x,
            y,
            img,
            initialized: false,
            entity: Entity::from_raw(0),
        }
    }

    pub fn get_entity(&mut self) -> Entity {
        self.entity
    }
    pub fn set_entity(&mut self, entity: Entity) {
        self.entity = entity;
    }

    pub fn get_image_as_str(&mut self) -> &str {
        self.img.as_str()
    }
    pub fn get_initialized(&mut self) -> bool {
        self.initialized
    }

    pub fn set_image(&mut self, img: String) {
        self.img = img;
    }
    pub fn set_initialized(&mut self, initialized: bool) {
        self.initialized = initialized;
    }

    pub fn get_x(&mut self) -> u32 {
        self.x
    }
    pub fn get_y(&mut self) -> u32 {
        self.y
    }

    pub fn get_decal_type_max(dir: String, type_str: String) -> u32 {
        let reader = match fs::read_dir(dir) {
            Ok(v) => v,
            _ => return 0,
        };

        let mut count = 0;

        for entry in reader {

            let file = match entry {
                Ok(f) => f,
                _ => continue,
            };

            let filename = match file.file_name().to_str() {
                Some(s) => s.to_string(),
                _ => continue,
            };

            if filename.contains(&type_str) {
                count += 1;
            }
        }

        count
    }
}