mod camera;
use camera::{Camera, camera_event_handler};

mod cursor;
use cursor::{Cursor, mouse_event_handler};

mod decal;

mod tile;

mod level;
use level::{Level, LevelBiome};

mod text;
use text::Text;

use bevy::prelude::{
    App,
    Assets,
    AssetServer,
    Color,
    ColorMaterial,
    Commands,
    DefaultPlugins,
    Entity,
    EventReader,
    Res,
    ResMut,
    IntoSystem,
    Windows,
    WindowDescriptor,
};

use bevy::{
    input::keyboard::KeyboardInput,
    input::ElementState::Pressed,
    input::ElementState::Released,
    input::keyboard::KeyCode::Key1,
    input::keyboard::KeyCode::Key2,
    input::keyboard::KeyCode::Key3,
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

const CAMERA_HIGHEST_LEVEL: f32 = 1.0;

const SCREEN_HEIGHT: f32 = 720.0;
const SCREEN_WIDTH: f32 = 1280.0;

fn main() {

    let text_content = [
        " Press {1} to change the biome.\n",
        " Press {2} to randomize the tiles.\n",
        " Press {3} to enable or disable 4K resolution.\n",
        " Press {W,A,S,D} or the arrow keys to navigate.\n",
        " Press {ESC} to exit the program.",
    ].concat();

    App::build()
        .insert_resource(WindowDescriptor {
            title: "Bevy engine example using tiles, camera, and keyboard plus mouse input".to_string(),
            scale_factor_override: Some(1.0),
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            resizable: false,
            cursor_visible: false,
            vsync: true,
            ..Default::default()
        })

        .add_plugins(DefaultPlugins)

        .insert_resource(Camera::new(320.0, 320.0, CAMERA_HIGHEST_LEVEL, SCREEN_HEIGHT, SCREEN_WIDTH))
        .insert_resource(Cursor::new("img/ui/mouse_gfx.png".to_string(), false, Entity::new(0)))
        .insert_resource(Level::new(LevelBiome::Marsh))
        .insert_resource(Text::new(32.0, Color::WHITE, &text_content))

        .add_startup_system(setup.system())

        .add_system(camera_event_handler.system())
        .add_system(keyboard_event_handler.system())
        .add_system(mouse_event_handler.system())

        .run();
}

fn setup(mut commands: Commands,
         asset_server: Res<AssetServer>,
         mut cam: ResMut<Camera>,
         mut cursor: ResMut<Cursor>,
         mut lvl: ResMut<Level>,
         txt: ResMut<Text>,
         mut materials: ResMut<Assets<ColorMaterial>>) {

    cam.start(&mut commands);
    cursor.render(&mut commands, &asset_server, &mut materials);
    lvl.render(&mut commands, &asset_server, &mut materials);
    txt.render("fonts/ultra_thin.ttf", &mut commands, &asset_server);
}

fn keyboard_event_handler(mut commands: Commands,
                          asset_server: Res<AssetServer>,
                          mut event_reader: EventReader<KeyboardInput>,
                          mut cursor: ResMut<Cursor>,
                          mut lvl: ResMut<Level>,
                          mut materials: ResMut<Assets<ColorMaterial>>,
                          mut windows: ResMut<Windows>) {

    for event in event_reader.iter() {

        match event.state {
            Pressed => {
                match event.key_code {

                    // exit
                    Some(Escape) => {
                        std::process::exit(0);
                    },

                    // hide the mouse whilst the camera is panning
                    Some(Up) | Some(W) | Some(Down) | Some(S) | Some(Right) | Some(D) | Some(Left) | Some(A) => {
                        cursor.hide(&mut commands);
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
                                lvl.change(LevelBiome::Desert);
                            },
                        };
                        lvl.render(&mut commands, &asset_server, &mut materials);
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
                        };
                        lvl.render(&mut commands, &asset_server, &mut materials);
                    },

                    Some(Key3) => {
                        let window = match windows.get_primary_mut() {
                            Some(w) => w,
                            _ => break
                        };
                        window.set_scale_factor_override(
                            window
                                .scale_factor_override()
                                .map(|n| ((n % 2.) + 1.))
                        );
                    },

                    _ => (),
                }

            },
            Released => {
                match event.key_code {

                    // restore the mouse cursor once the camera stops
                    Some(Up) | Some(W) | Some(Down) | Some(S) | Some(Right) | Some(D) | Some(Left) | Some(A) => {
                        cursor.render(&mut commands, &asset_server, &mut materials);
                    },

                    _ => (),
                }
            },
        }
    }
}
