mod camera;
use camera::{Camera, camera_event_handler};

mod cursor;
use cursor::{Cursor, mouse_event_handler};

mod level;
use level::{Level, LevelBiome};

mod text;
use text::Text;

use bevy::prelude::*;

use bevy::{
    input::keyboard::KeyboardInput,
    input::ElementState::Released,
    input::keyboard::KeyCode::Key1,
    input::keyboard::KeyCode::Key2,
    input::keyboard::KeyCode::Escape,
};

fn main() {

    let text_content = [
        " Press {1} to change the biome.\n",
        " Press {2} to randomize the tiles.\n",
        " Press {W,A,S,D} or the arrow keys to navigate.\n",
        " Press {ESC} to exit the program.",
    ].concat();

    App::build()
        .insert_resource(WindowDescriptor {
            title: "Bevy engine example using tiles, camera, and keyboard plus mouse input".to_string(),
            width: 1280.0,
            height: 720.0,
            resizable: false,
            cursor_visible: false,
            vsync: true,
            ..Default::default()
        })

        .add_plugins(DefaultPlugins)

        .insert_resource(Camera::new(320.0, 320.0, 0.0))
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
                          mut lvl: ResMut<Level>,
                          mut materials: ResMut<Assets<ColorMaterial>>) {

    for event in event_reader.iter() {

        // ignored released events for now
        if event.state == Released {
            continue;
        }

        match event.key_code {

            // exit
            Some(Escape) => {
                std::process::exit(0);
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

            _ => (),
        }
    }
}
