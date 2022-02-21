mod camera;
use camera::{Camera, camera_event_handler};

mod constants;

mod cursor;
use cursor::{Cursor, mouse_event_handler};

mod decal;

mod menu;
use menu::Menu;

mod keyboard;
use keyboard::keyboard_event_handler;

mod tile;

mod level;
use level::{Level, LevelBiome};

mod text;
use text::Text;

mod ui;

use bevy::prelude::{
    App,
    AssetServer,
    Color,
    Commands,
    DefaultPlugins,
    Res,
    ResMut,
    WindowDescriptor,
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
        " Press {ESC} to open and close the menu.",
    ].concat();

    App::new()
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
        .insert_resource(Cursor::new("img/ui/mouse_gfx.png".to_string()))
        .insert_resource(Menu::new("img/ui/menu_main.png".to_string()))
        .insert_resource(Level::new(LevelBiome::Marsh))
        .insert_resource(Text::new(32.0, Color::WHITE, &text_content))

        .add_startup_system(setup)

        .add_system(camera_event_handler)
        .add_system(keyboard_event_handler)
        .add_system(mouse_event_handler)

        .run();
}

fn setup(mut commands: Commands,
         asset_server: Res<AssetServer>,
         mut cam: ResMut<Camera>,
         mut cursor: ResMut<Cursor>,
         mut menu: ResMut<Menu>,
         mut lvl: ResMut<Level>,
         txt: ResMut<Text>) {

    cam.start(&mut commands);
    menu.render(&mut commands, &asset_server, &cam);
    cursor.render(&mut commands, &asset_server);
    lvl.render(&mut commands, &asset_server);
    txt.render("fonts/ultra_thin.ttf", &mut commands, &asset_server);
}
