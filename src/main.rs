mod camera;
use camera::{Camera, camera_event_handler};

mod constants;
use constants::{CAMERA_HIGHEST_LEVEL, SCREEN_HEIGHT, SCREEN_WIDTH};

mod creature;

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

mod utils;

use bevy::{prelude::{
    App,
    AssetServer,
    Commands,
    DefaultPlugins,
    ImagePlugin,
    PluginGroup,
    Res,
    ResMut, Startup, Update,
}, window::{
    CursorOptions as BevyCursor,
    MonitorSelection,
    PresentMode,
    Window,
    WindowMode,
    WindowPlugin,
    WindowResolution
}};

fn main() {

    let current_options = get_options();

    let mode: WindowMode = if current_options.fullscreen && current_options.borderless { WindowMode::BorderlessFullscreen(MonitorSelection::Primary) }
                           else if current_options.fullscreen { WindowMode::Fullscreen(MonitorSelection::Primary) }
                           else { WindowMode::Windowed };

    let scale_factor_override = match current_options.four_k_mode {
        true => 2.,
        false => 1.,
    };

    // Present Mode is what wgpu calls "V-Sync"
    let present_mode = match current_options.vsync {
        true => PresentMode::AutoVsync,
        false => PresentMode::Immediate
    };

    let bevy_cursor = BevyCursor {
        visible: false,
        ..Default::default()
    };

    App::new()

        .add_plugins(DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    cursor_options: bevy_cursor,
                    fit_canvas_to_parent: true,
                    title: "Bevy engine example using tiles, camera, and keyboard plus mouse input".to_string(),
                    resolution: WindowResolution::new(SCREEN_WIDTH,SCREEN_HEIGHT).with_scale_factor_override(scale_factor_override),
                    resizable: false,
                    mode,
                    present_mode,
                    ..Default::default()
                }),
                ..Default::default()})
        )

        .insert_resource(Camera::new(320.0, 320.0, CAMERA_HIGHEST_LEVEL, SCREEN_HEIGHT, SCREEN_WIDTH))
        .insert_resource(Cursor::new("img/ui/mouse_gfx.png".to_string()))
        .insert_resource(Dialog::new())
        .insert_resource(Gamestate::new())
        .insert_resource(Menu::new("img/ui/menu_main.png".to_string()))
        .insert_resource(Level::new(LevelBiome::Marsh))

        .add_systems(Startup, setup)

        .add_systems(Update, gamestate_handler)
        .add_systems(Update, camera_event_handler)
        .add_systems(Update, keyboard_event_handler)
        .add_systems(Update, mouse_event_handler)

        .run();
}

fn setup(mut commands: Commands,
         mut asset_server: Res<AssetServer>,
         mut cam: ResMut<Camera>,
         mut cursor: ResMut<Cursor>,
         mut dialog: ResMut<Dialog>,
         mut menu: ResMut<Menu>,
         mut lvl: ResMut<Level>) {

    cam.start(&mut commands);
    dialog.load_dialog(&mut commands, 1);
    menu.render(&mut commands, &asset_server, &cam);
    cursor.render(&mut commands, &mut asset_server);
    lvl.render(&mut commands, &asset_server);
}
