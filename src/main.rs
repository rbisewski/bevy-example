mod camera;
use camera::{Camera, camera_event_handler};

mod constants;
use constants::{CAMERA_HIGHEST_LEVEL, SCREEN_HEIGHT, SCREEN_WIDTH};

mod cursor;
use cursor::{Cursor, mouse_event_handler};

mod decal;

mod dialog;
use dialog::Dialog;

mod gamestate;
use gamestate::{Gamestate, gamestate_handler};

mod menu;
use menu::Menu;

mod keyboard;
use keyboard::keyboard_event_handler;

mod tile;

mod level;
use level::{Level, LevelBiome};

mod options;
use options::get_options;

mod text;

mod ui;

use bevy::{prelude::{
    App,
    AssetServer,
    Commands,
    DefaultPlugins,
    Res,
    ResMut,
    WindowDescriptor,
}, window::{
    PresentMode,
    WindowMode
}};

fn main() {

    let current_options = get_options();

    let mode: WindowMode = if current_options.fullscreen && current_options.borderless { WindowMode::BorderlessFullscreen }
                           else if current_options.fullscreen { WindowMode::Fullscreen }
                           else { WindowMode::Windowed };

    let scale_factor_override = match current_options.four_k_mode {
        true => Some(2.0),
        false => Some(1.0)
    };

    // Present Mode is what wgpu calls "V-Sync"
    let present_mode = match current_options.vsync {
        true => PresentMode::Mailbox,
        false => PresentMode::Immediate
    };

    App::new()
        .insert_resource(WindowDescriptor {
            title: "Bevy engine example using tiles, camera, and keyboard plus mouse input".to_string(),
            scale_factor_override,
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            resizable: false,
            cursor_visible: false,
            mode,
            present_mode,
            ..Default::default()
        })

        .add_plugins(DefaultPlugins)

        .insert_resource(Camera::new(320.0, 320.0, CAMERA_HIGHEST_LEVEL, SCREEN_HEIGHT, SCREEN_WIDTH))
        .insert_resource(Cursor::new("img/ui/mouse_gfx.png".to_string()))
        .insert_resource(Dialog::new())
        .insert_resource(Gamestate::new())
        .insert_resource(Menu::new("img/ui/menu_main.png".to_string()))
        .insert_resource(Level::new(LevelBiome::Marsh))

        .add_startup_system(setup)

        .add_system(gamestate_handler)
        .add_system(camera_event_handler)
        .add_system(keyboard_event_handler)
        .add_system(mouse_event_handler)

        .run();
}

fn setup(mut commands: Commands,
         asset_server: Res<AssetServer>,
         mut cam: ResMut<Camera>,
         mut cursor: ResMut<Cursor>,
         mut dialog: ResMut<Dialog>,
         mut menu: ResMut<Menu>,
         mut lvl: ResMut<Level>) {

    cam.start(&mut commands);
    dialog.load_dialog(&mut commands, "dialog/generic.json".to_string(), 1);
    menu.render(&mut commands, &asset_server, &cam);
    cursor.render(&mut commands, &asset_server);
    lvl.render(&mut commands, &asset_server);
}
