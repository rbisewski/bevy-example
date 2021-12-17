use bevy::prelude::{
    Commands,
    Assets,
    AssetServer,
    ColorMaterial,
    Transform,
    Res,
    ResMut,
    SpriteBundle,
    Vec3,
};

use rand::Rng;

use crate::tile::{Tile, TILE_SIZE};

#[allow(dead_code)]
pub enum LevelBiome {
    Desert,
    Grass,
    Ice,
    Marsh,
    Snow,
}

pub struct Level {
    biome: LevelBiome,
    tiles: Vec<Tile>,
}

impl Level {

    pub fn new(biome: LevelBiome) -> Level {

        let min = 0;
        let max = 22;
        let mut tiles: Vec<Tile> = Vec::new();

        let biome_max;
        let biome_folder = match biome {
            LevelBiome::Desert => {
                biome_max = 10;
                "img/desert/"
            },
            LevelBiome::Grass => {
                biome_max = 8;
                "img/grass/"
            },
            LevelBiome::Ice => {
                biome_max = 6;
                "img/ice/"
            },
            LevelBiome::Marsh => {
                biome_max = 14;
                "img/marsh/"
            },
            LevelBiome::Snow => {
                biome_max = 6;
                "img/snow/"
            },
        };

        let mut rng = rand::thread_rng();
        for x in min..max {
            for y in min..max {
                let img_num = rng.gen_range(1..biome_max);
                let tile = Tile::new(x, y, [biome_folder, &img_num.to_string(), ".png"].concat());
                tiles.push(tile);
            }
        }

        Level { biome, tiles }
    }

    pub fn get_biome(&mut self) -> &LevelBiome {
        &self.biome
    }

    pub fn change(&mut self, biome: LevelBiome) {

        let biome_max;

        let biome_folder = match biome {
            LevelBiome::Desert => {
                biome_max = 10;
                "img/desert/"
            },
            LevelBiome::Grass => {
                biome_max = 8;
                "img/grass/"
            },
            LevelBiome::Ice => {
                biome_max = 6;
                "img/ice/"
            },
            LevelBiome::Marsh => {
                biome_max = 14;
                "img/marsh/"
            },
            LevelBiome::Snow => {
                biome_max = 6;
                "img/snow/"
            },
        };

        self.biome = biome;

        let mut rng = rand::thread_rng();
        for tile in self.tiles.iter_mut() {
            let img_num = rng.gen_range(1..biome_max);
            tile.set_image(
                [biome_folder, &img_num.to_string(), ".png"].concat()
            );
        }
    }

    pub fn render(&mut self, 
                  commands: &mut Commands, 
                  asset_server: &Res<AssetServer>, 
                  materials: &mut ResMut<Assets<ColorMaterial>>) {

        let mut texture_handle;
        for tile in self.tiles.iter_mut() {

            if tile.get_initialized() {
                commands.entity(tile.get_entity()).despawn();
                tile.set_initialized(false);
            } 

            texture_handle = asset_server.load(tile.get_image_as_str());

            let x = tile.get_x();
            let y = tile.get_y();
            tile.set_entity(
                commands
                    .spawn()
                    .insert_bundle(SpriteBundle {
                        material: materials.add(texture_handle.into()),
                        transform: Transform {
                            translation: Vec3::new(TILE_SIZE * x as f32, TILE_SIZE * y as f32, 0.0),
                            ..Default::default()
                    },
                    ..Default::default()
                })
                .id()
            );

            tile.set_initialized(true);
        }
    }
}
