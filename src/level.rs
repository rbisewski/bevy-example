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
use crate::tile::{Tile, TILE_SIZE};

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
    decal_types: Vec<String>,
    decals: Vec<Decal>,
}

impl Level {

    pub fn new(biome: LevelBiome) -> Level {

        let min = 0;
        let max = 22;
        let mut decals: Vec<Decal> = Vec::new();
        let mut tiles: Vec<Tile> = Vec::new();

        let (biome_max, biome_folder, decal_types) = Level::set_biome(&biome);

        for x in min..max {
            for y in min..max {
                let img_num = Level::random(1, biome_max);
                let tile = Tile::new(x, y, [biome_folder, &img_num.to_string(), ".png"].concat());
                tiles.push(tile);
            }
        }

        // generate 35 to 45 random decals
        let mut coords: Vec<(u32,u32)> = vec![];
        for _ in 0..Level::random(35,45) {

            let xy = (Level::random(0,22), Level::random(0,22));

            // some very basic logic to skip decals that exists in the same (x,y)
            if coords.contains(&xy) {
                continue;
            }
            coords.push(xy);

            let random_decal_type = Level::random(0, decal_types.len() as u32) as usize;
            let decal_max = Decal::get_decal_type_max(
                decal_types[random_decal_type].as_str(),
            );

            let img_num = Level::random(1, decal_max);
            let img = [
                "img/decals/",
                decal_types[random_decal_type].as_str(),
                &img_num.to_string(),
                ".png"
            ].concat();

            let decal = Decal::new(xy.0,xy.1,img);
            decals.push(decal);
        }

        Level { biome, tiles, decal_types, decals }
    }

    /*
     * Changes the biome type and associated details
     *
     * @param    LevelBiome  biome type, such as Grass or Desert
     *
     * @returns  i32         biome max number
     *           char*       biome folder location
     *           string[]    list of possible decal types
     */
    pub fn set_biome(biome: &LevelBiome) -> (u32, &'static str, Vec<String>) {
        let decal_types: Vec<String>;
        let biome_max;
        let biome_folder = match biome {
            LevelBiome::Desert => {
                biome_max = 10;
                decal_types = vec![
                    "bones_".to_string(),
                    "cactus_".to_string(),
                    "dead_vegetation_".to_string(),
                    "rock_".to_string(),
                ];
                "img/desert/"
            },
            LevelBiome::Grass => {
                biome_max = 9;
                decal_types = vec![
                    "flower_".to_string(),
                    "plant_".to_string(),
                    "mushroom_".to_string(),
                    "rock_".to_string(),
                ];
                "img/grass/"
            },
            LevelBiome::Ice => {
                biome_max = 6;
                decal_types = vec![
                    "rock_".to_string(),
                ];
                "img/ice/"
            },
            LevelBiome::Marsh => {
                biome_max = 14;
                decal_types = vec![
                    "flower_".to_string(),
                    "plant_".to_string(),
                    "mushroom_".to_string(),
                    "rock_".to_string(),
                ];
                "img/marsh/"
            },
            LevelBiome::Snow => {
                biome_max = 6;
                decal_types = vec![
                    "rock_".to_string(),
                ];
                "img/snow/"
            },
            LevelBiome::Space => {
                biome_max = 14;
                decal_types = vec![
                    "asteroid_".to_string(),
                    "planet_".to_string(),
                ];
                "img/space/"
            },
        };

        (biome_max, biome_folder, decal_types)
    }

    pub fn get_biome(&mut self) -> &LevelBiome {
        &self.biome
    }

    pub fn change(&mut self, biome: LevelBiome) {

        let (biome_max, biome_folder, decal_types) = Level::set_biome(&biome);

        self.biome = biome;
        self.decal_types = decal_types;

        for tile in self.tiles.iter_mut() {
            let img_num = Level::random(1, biome_max);
            tile.set_image(
                [biome_folder, &img_num.to_string(), ".png"].concat()
            );
        }

        for decal in self.decals.iter_mut() {
            let random_decal_type = Level::random(0, self.decal_types.len() as u32) as usize;
            let decal_max = Decal::get_decal_type_max(
                &self.decal_types[random_decal_type],
            );
            let img_num = match decal_max {
                1 => 1,
                _ => Level::random(1, decal_max+1),
            };
            decal.set_image([
                "img/decals/",
                self.decal_types[random_decal_type].as_str(),
                &img_num.to_string(),
                ".png"
            ].concat());
        }
    }

    pub fn random(min: u32, max: u32) -> u32 {
        if min == 1 && max == 1 {
            return 1;
        } else if min == max {
            return max;
        }
        fastrand::u32(min..max)
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
            let path_to_texture = asset_server.load(&tile.img);

            tile.set_entity(commands.spawn((
                    Sprite::from_image(path_to_texture),
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
            let path_to_texture = asset_server.load(&decal.img);

            decal.set_entity(commands.spawn((
                    Sprite::from_image(path_to_texture),
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
    }
}
