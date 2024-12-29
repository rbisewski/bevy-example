use bevy::prelude::Entity;

use crate::utils::random;

pub struct Creature {
    x: u32,
    y: u32,
    pub img: String,
    initialized: bool,
    entity: Entity,
}

impl Creature {
    pub fn new(x: u32, y: u32, img: String) -> Creature {
        Creature {
            x,
            y,
            img,
            initialized: false,
            entity: Entity::from_raw(0),
        }
    }

    pub fn next_turn(&mut self) -> (u32,u32) {
        let neighbours = [
            [self.x-1,self.y-1],[self.x,self.y-1],[self.x+1,self.y-1],
            [self.x-1,self.y  ],                  [self.x+1,self.y  ],
            [self.x-1,self.y+1],[self.x,self.y+1],[self.x+1,self.y+1]
        ];

        let random_neighbour = random(0, 8) as usize;
        let new_location = neighbours[random_neighbour];

        let nx = new_location[0];
        let ny = new_location[1];

        // if overflow, return existing location
        if nx == 4294967295 || ny == 4294967295 {
            return (self.get_x(),self.get_y());
        }

        (nx,ny)
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

    pub fn set_x(&mut self, x: u32) {
        self.x = x;
    }
    pub fn set_y(&mut self, y: u32) {
        self.y = y;
    }

    pub fn get_creature_type_max(type_str: &str) -> u32 {
        match type_str {
            _ => 1,
        }
    }
}