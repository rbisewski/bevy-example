use bevy::prelude::{
    AssetServer,
    Commands,
    EventReader,
    Res,
    ResMut,
};

use bevy::{
    input::keyboard::KeyboardInput,
    input::ButtonState::Pressed,
    input::ButtonState::Released,
    input::keyboard::KeyCode::Digit1,
    input::keyboard::KeyCode::Digit2,
    input::keyboard::KeyCode::KeyW,
    input::keyboard::KeyCode::KeyS,
    input::keyboard::KeyCode::KeyA,
    input::keyboard::KeyCode::KeyD,
    input::keyboard::KeyCode::ArrowUp,
    input::keyboard::KeyCode::ArrowDown,
    input::keyboard::KeyCode::ArrowRight,
    input::keyboard::KeyCode::ArrowLeft,
    input::keyboard::KeyCode::Escape,
};

use crate::cursor::Cursor;
use crate::gamestate::{Gamestate, Status};
use crate::menu::Menu;
use crate::level::{Level, LevelBiome};

pub fn keyboard_event_handler(mut commands: Commands,
                          mut asset_server: Res<AssetServer>,
                          mut event_reader: EventReader<KeyboardInput>,
                          mut cursor: ResMut<Cursor>,
                          mut gamestate: ResMut<Gamestate>,
                          menu: ResMut<Menu>,
                          mut lvl: ResMut<Level>) {

    for event in event_reader.read() {

        match event.state {
            Pressed => {
                match event.key_code {

                    // exit
                    Escape => {
                        match menu.visible() {
                            true => {
                                gamestate.set_status(Status::DialogOpen);
                            },
                            false => {
                                gamestate.set_status(Status::MenuOpen);
                            }
                        }
                    },

                    // hide the mouse whilst the camera is panning
                    ArrowUp | KeyW | ArrowDown | KeyS | ArrowRight | KeyD | ArrowLeft | KeyA => {
                        if gamestate.get_status() == Status::Playing {
                            cursor.hide(&mut commands);
                        }
                    },

                    // switch biome
                    Digit1 => {
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
                    Digit2 => {
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
                    ArrowUp | KeyW | ArrowDown | KeyS | ArrowRight | KeyD | ArrowLeft | KeyA => {
                        cursor.render(&mut commands, &mut asset_server);
                    },

                    _ => (),
                }
            },
        }
    }
}