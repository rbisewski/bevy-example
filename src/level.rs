use bevy::prelude::{
    Commands,
    AssetServer,
    Transform,
    Res,
    Resource,
    Sprite,
    Visibility,
};

use crate::constants::{Z_VALUE_DECAL, Z_VALUE_TILE};
use crate::decal::Decal;
use crate::creature::Creature;
use crate::tile::{Tile, TILE_SIZE};
use crate::utils::random;

pub enum LevelBiome {
    Desert,
    Grass,
    Ice,
    Marsh,
    Snow,
    Space,
}

#[derive(Resource)]
pub struct Level {
    biome: LevelBiome,
    tiles: Vec<Tile>,
    creatures: Vec<Creature>,
    creature_types: Vec<String>,
    decals: Vec<Decal>,
    decal_types: Vec<String>,
}

impl Level {

    pub fn new(biome: LevelBiome) -> Level {

        let min = 0;
        let max = 22;
        let mut coords: Vec<(u32,u32)> = vec![];

        let (biome_max, biome_folder, creature_types, decal_types) = Level::set_biome(&biome);

        //
        // generate a grid of randomized tiles
        //
        let mut tiles: Vec<Tile> = Vec::new();
        for x in min..max {
            for y in min..max {
                let img_num = random(1, biome_max);
                let tile = Tile::new(x, y, [biome_folder, &img_num.to_string(), ".png"].concat());
                tiles.push(tile);
            }
        }

        //
        // generate 35 to 45 random decals
        //
        let mut decals: Vec<Decal> = Vec::new();
        for _ in 0..random(35,45) {

            let xy = (random(min,max), random(min,max));

            // some very basic logic to skip decals that exists in the same (x,y)
            if coords.contains(&xy) {
                continue;
            }
            coords.push(xy);

            let random_decal_type = random(0, decal_types.len() as u32) as usize;
            let decal_max = Decal::get_decal_type_max(
                decal_types[random_decal_type].as_str(),
            );

            let img_num = random(1, decal_max);
            let img = [
                "img/decals/",
                decal_types[random_decal_type].as_str(),
                &img_num.to_string(),
                ".png"
            ].concat();

            let decal = Decal::new(xy.0,xy.1,img);
            decals.push(decal);
        }

        //
        // generate 6 to 12 random creatures
        //
        let mut creatures: Vec<Creature> = Vec::new();
        for _ in 0..random(6,12) {

            let xy = (random(min,max), random(min,max));

            // some very basic logic to skip decals that exists in the same (x,y)
            if coords.contains(&xy) {
                continue;
            }
            coords.push(xy);

            let random_creature_type = random(0, creature_types.len() as u32) as usize;
            let creature_max = Creature::get_creature_type_max(
                creature_types[random_creature_type].as_str(),
            );

            let img_num = random(1, creature_max);
            let img = [
                "img/creatures/",
                creature_types[random_creature_type].as_str(),
                &img_num.to_string(),
                ".png"
            ].concat();

            creatures.push(
                Creature::new(xy.0,xy.1,img)
            );
        }

        Level { biome, tiles, creatures, creature_types, decals, decal_types }
    }

    /*
     * Set biome details and associated decals
     *
     * @param    LevelBiome  biome type, such as Grass or Desert
     *
     * @returns  i32         biome max number
     *           char*       biome folder location
     *           string[]    list of possible creature types
     *           string[]    list of possible decal types
     */
    pub fn set_biome(biome: &LevelBiome) -> (u32, &'static str, Vec<String>, Vec<String>) {
        let creature_types: Vec<String>;
        let decal_types: Vec<String>;
        let biome_max;

        let biome_folder = match biome {
            LevelBiome::Desert => {
                biome_max = 10;
                creature_types = vec![
                    "bobcat_".to_string(),
                    "pelican_".to_string(),
                    "skeleton_".to_string(),
                    "slime_".to_string(),
                    "wolf_".to_string(),
                ];
                decal_types = vec![
                    "bones_".to_string(),
                    "cactus_".to_string(),
                    "dead_vegetation_".to_string(),
                    "rock_".to_string(),
                ];
                "img/biomes/desert/"
            },
            LevelBiome::Grass => {
                biome_max = 9;
                creature_types = vec![
                    "bobcat_".to_string(),
                    "pelican_".to_string(),
                    "skeleton_".to_string(),
                    "slime_".to_string(),
                    "wolf_".to_string(),
                ];
                decal_types = vec![
                    "flower_".to_string(),
                    "plant_".to_string(),
                    "mushroom_".to_string(),
                    "rock_".to_string(),
                ];
                "img/biomes/grass/"
            },
            LevelBiome::Ice => {
                biome_max = 6;
                creature_types = vec![
                    "bobcat_".to_string(),
                    "pelican_".to_string(),
                    "skeleton_".to_string(),
                    "slime_".to_string(),
                    "wolf_".to_string(),
                ];
                decal_types = vec![
                    "rock_".to_string(),
                ];
                "img/biomes/ice/"
            },
            LevelBiome::Marsh => {
                biome_max = 14;
                creature_types = vec![
                    "bobcat_".to_string(),
                    "pelican_".to_string(),
                    "skeleton_".to_string(),
                    "slime_".to_string(),
                    "wolf_".to_string(),
                ];
                decal_types = vec![
                    "flower_".to_string(),
                    "plant_".to_string(),
                    "mushroom_".to_string(),
                    "rock_".to_string(),
                ];
                "img/biomes/marsh/"
            },
            LevelBiome::Snow => {
                biome_max = 6;
                creature_types = vec![
                    "bobcat_".to_string(),
                    "pelican_".to_string(),
                    "skeleton_".to_string(),
                    "slime_".to_string(),
                    "wolf_".to_string(),
                ];
                decal_types = vec![
                    "rock_".to_string(),
                ];
                "img/biomes/snow/"
            },
            LevelBiome::Space => {
                biome_max = 14;
                creature_types = vec![
                    "alien_creeper_".to_string(),
                    "robotic_probe_".to_string(),
                ];
                decal_types = vec![
                    "asteroid_".to_string(),
                    "planet_".to_string(),
                ];
                "img/biomes/space/"
            },
        };

        (biome_max, biome_folder, creature_types, decal_types)
    }

    pub fn get_biome(&mut self) -> &LevelBiome {
        &self.biome
    }

    /*
     * Changes the biome type and associated details
     *
     * @param    LevelBiome  biome type, such as Grass or Desert
     */
    pub fn change(&mut self, biome: LevelBiome) {

        let (biome_max, biome_folder, creature_types, decal_types) = Level::set_biome(&biome);

        self.biome = biome;
        self.creature_types = creature_types;
        self.decal_types = decal_types;

        for tile in self.tiles.iter_mut() {
            let img_num = random(1, biome_max);
            tile.set_image(
                [biome_folder, &img_num.to_string(), ".png"].concat()
            );
        }

        for creature in self.creatures.iter_mut() {
            let random_creature_type = random(0, self.creature_types.len() as u32) as usize;
            let creature_max = Creature::get_creature_type_max(
                &self.creature_types[random_creature_type],
            );
            let img_num = random(1, creature_max);

            creature.set_image([
                "img/creatures/",
                self.creature_types[random_creature_type].as_str(),
                &img_num.to_string(),
                ".png"
            ].concat());
        }

        for decal in self.decals.iter_mut() {
            let random_decal_type = random(0, self.decal_types.len() as u32) as usize;
            let decal_max = Decal::get_decal_type_max(
                &self.decal_types[random_decal_type],
            );
            let img_num = match decal_max {
                1 => 1,
                _ => random(1, decal_max+1),
            };
            decal.set_image([
                "img/decals/",
                self.decal_types[random_decal_type].as_str(),
                &img_num.to_string(),
                ".png"
            ].concat());
        }
    }

    pub fn get_creature_positions(&mut self) -> Vec<(u32,u32)> {
        let mut positions: Vec<(u32,u32)> = vec![];

        for c in self.creatures.iter_mut() {
            let pos = (c.get_x(), c.get_y());
            positions.push(pos);
        }

        positions
    }

    pub fn next_turn(&mut self) {
        let mut positions = self.get_creature_positions();

        for c in self.creatures.iter_mut() {
            let x = c.get_x();
            let y = c.get_y();
            let (nx, ny) = c.next_turn();

            if positions.contains(&(nx,ny)) {
                continue;
            }

            c.set_x(nx);
            c.set_y(ny);

            positions.retain(|value| *value != (x,y));
            positions.push((nx,ny));
        }
    }

    pub fn render(&mut self,
                  commands: &mut Commands,
                  asset_server: &Res<AssetServer>) {

        //
        // TILES
        //
        for tile in self.tiles.iter_mut() {

            if tile.get_initialized() {
                commands.entity(tile.get_entity()).despawn();
                tile.set_initialized(false);
            }

            let x = tile.get_x();
            let y = tile.get_y();

            tile.set_entity(commands.spawn((
                    Sprite::from_image(
                        asset_server.load(&tile.img)
                    ),
                    Transform::from_xyz(
                       TILE_SIZE * x as f32,
                       TILE_SIZE * y as f32,
                       Z_VALUE_TILE
                    ),
                )).id()
            );

            tile.set_initialized(true);
        }

        //
        // DECALS
        //
        for decal in self.decals.iter_mut() {

            if decal.get_initialized() {
                commands.entity(decal.get_entity()).despawn();
                decal.set_initialized(false);
            }

            let x = decal.get_x();
            let y = decal.get_y();

            decal.set_entity(commands.spawn((
                    Sprite::from_image(
                        asset_server.load(&decal.img)
                    ),
                    Visibility::Visible,
                    Transform::from_xyz(
                        TILE_SIZE * x as f32,
                        TILE_SIZE * y as f32,
                        Z_VALUE_DECAL
                    ),
                )).id()
            );

            decal.set_initialized(true);
        }

        //
        // CREATURES
        //
        for creature in self.creatures.iter_mut() {

            if creature.get_initialized() {
                commands.entity(creature.get_entity()).despawn();
                creature.set_initialized(false);
            }

            let x = creature.get_x();
            let y = creature.get_y();

            creature.set_entity(commands.spawn((
                    Sprite::from_image(
                        asset_server.load(&creature.img)
                    ),
                    Visibility::Visible,
                    Transform::from_xyz(
                        TILE_SIZE * x as f32,
                        TILE_SIZE * y as f32,
                        Z_VALUE_DECAL
                    ),
                )).id()
            );

            creature.set_initialized(true);
        }
    }
}
