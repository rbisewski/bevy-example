use bevy::prelude::Entity;

pub static TILE_SIZE: f32 = 32.0;

pub struct Tile {
    x: u32,
    y: u32,
    pub img: String,
    initialized: bool,
    entity: Entity,
}

impl Tile {
    pub fn new(x: u32, y: u32, img: String) -> Tile {
        Tile {
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
}