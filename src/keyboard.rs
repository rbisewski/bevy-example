use bevy::prelude::{
    AssetServer,
    Commands,
    EventReader,
    Res,
    ResMut,
    Windows,
};

use bevy::{
    input::keyboard::KeyboardInput,
    input::ElementState::Pressed,
    input::ElementState::Released,
    input::keyboard::KeyCode::Key1,
    input::keyboard::KeyCode::Key2,
    input::keyboard::KeyCode::W,
    input::keyboard::KeyCode::S,
    input::keyboard::KeyCode::A,
    input::keyboard::KeyCode::D,
    input::keyboard::KeyCode::Up,
    input::keyboard::KeyCode::Down,
    input::keyboard::KeyCode::Right,
    input::keyboard::KeyCode::Left,
    input::keyboard::KeyCode::Escape,
};

use crate::camera::Camera;
use crate::cursor::Cursor;
use crate::menu::Menu;
use crate::level::{Level, LevelBiome};

pub fn keyboard_event_handler(mut commands: Commands,
                          asset_server: Res<AssetServer>,
                          mut event_reader: EventReader<KeyboardInput>,
                          cam: ResMut<Camera>,
                          mut cursor: ResMut<Cursor>,
                          mut menu: ResMut<Menu>,
                          mut lvl: ResMut<Level>,
                          windows: ResMut<Windows>) {

    for event in event_reader.iter() {

        match event.state {
            Pressed => {
                match event.key_code {

                    // exit
                    Some(Escape) => {
                        match menu.visible() {
                            true => {
                                menu.hide(&mut commands);
                            },
                            false => {
                                menu.reset_mode();
                                menu.render(&mut commands, &asset_server, &cam, &windows);
                            }
                        }
                    },

                    // hide the mouse whilst the camera is panning
                    Some(Up) | Some(W) | Some(Down) | Some(S) | Some(Right) | Some(D) | Some(Left) | Some(A) => {
                        if !menu.visible() {
                            cursor.hide(&mut commands);
                        }
                    },

                    // switch biome
                    Some(Key1) => {
                        match lvl.get_biome() {
                            LevelBiome::Desert => {
                                lvl.change(LevelBiome::Grass);
                            },
                            LevelBiome::Grass => {
                                lvl.change(LevelBiome::Ice);
                            },
                            LevelBiome::Ice => {
                                lvl.change(LevelBiome::Marsh);
                            },
                            LevelBiome::Marsh => {
                                lvl.change(LevelBiome::Snow);
                            },
                            LevelBiome::Snow => {
                                lvl.change(LevelBiome::Space);
                            },
                            LevelBiome::Space => {
                                lvl.change(LevelBiome::Desert);
                            },
                        };
                        lvl.render(&mut commands, &asset_server);
                    },

                    // randomize tiles
                    Some(Key2) => {
                        match lvl.get_biome() {
                            LevelBiome::Desert => {
                                lvl.change(LevelBiome::Desert);
                            },
                            LevelBiome::Grass => {
                                lvl.change(LevelBiome::Grass);
                            },
                            LevelBiome::Ice => {
                                lvl.change(LevelBiome::Ice);
                            },
                            LevelBiome::Marsh => {
                                lvl.change(LevelBiome::Marsh);
                            },
                            LevelBiome::Snow => {
                                lvl.change(LevelBiome::Snow);
                            },
                            LevelBiome::Space => {
                                lvl.change(LevelBiome::Space);
                            },
                        };
                        lvl.render(&mut commands, &asset_server);
                    },

                    _ => (),
                }

            },
            Released => {
                match event.key_code {

                    // restore the mouse cursor once the camera stops
                    Some(Up) | Some(W) | Some(Down) | Some(S) | Some(Right) | Some(D) | Some(Left) | Some(A) => {
                        cursor.render(&mut commands, &asset_server);
                    },

                    _ => (),
                }
            },
        }
    }
}