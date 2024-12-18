use bevy::prelude::Entity;

pub struct Decal {
    x: u32,
    y: u32,
    pub img: String,
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

    pub fn get_decal_type_max(type_str: &str) -> u32 {
        match type_str {
            "asteroid_"        => 6,
            "bones_"           => 3,
            "cactus_"          => 5,
            "dead_vegatation_" => 5,
            "egg_"             => 1,
            "flower_"          => 2,
            "leaves_"          => 1,
            "mushroom_"        => 5,
            "planet_"          => 14,
            "plant_"           => 14,
            "rock_"            => 4,
            _ => 1,
        }
    }
}