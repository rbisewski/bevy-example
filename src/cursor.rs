use bevy::input::ButtonState;
use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::{
    AssetServer,
    Commands,
    Component,
    CursorMoved,
    Entity,
    EventReader,
    Query,
    Res,
    ResMut,
    SpriteBundle,
    Transform,
    With,
    MouseButton,
};

use crate::camera::Camera;
use crate::dialog::Dialog;
use crate::gamestate::{Gamestate, Status};
use crate::menu::Menu;
use crate::constants::{Z_VALUE_CURSOR};
use crate::options::{toggle_option};

#[derive(Component)]
pub struct CursorEntity;

pub struct Cursor {
    img: String,
    initialized: bool,
    entity: Entity,
    x: f32,
    y: f32,
}

impl Cursor {

    pub fn new(img: String) -> Cursor {
        Cursor { img, initialized: false, entity: Entity::from_raw(0), x: 0., y: 0. }
    }

    pub fn render(&mut self,
                  commands: &mut Commands,
                  asset_server: &Res<AssetServer>) {

        if self.initialized {
            commands.entity(self.entity).despawn();
            self.initialized = false;
        }

        self.entity = commands
                         .spawn()
                         .insert_bundle(SpriteBundle {
                             texture: asset_server.load(self.img.as_str()),
                             transform: Transform::from_xyz(self.x, self.y, Z_VALUE_CURSOR),
                             ..Default::default()
                         })
                         .insert(CursorEntity)
                         .id();

        self.initialized = true;
    }

    pub fn hide(&mut self, commands: &mut Commands) {
        if self.initialized {
            commands.entity(self.entity).despawn();
            self.initialized = false;
        }
    }
}

pub fn mouse_event_handler(mut cursor_moved: EventReader<CursorMoved>,
                           mut cursor_clicked: EventReader<MouseButtonInput>,
                           mut commands: Commands,
                           asset_server: Res<AssetServer>,
                           mut cursor: ResMut<Cursor>,
                           cam: ResMut<Camera>,
                           mut gamestate: ResMut<Gamestate>,
                           mut menu: ResMut<Menu>,
                           mut dialog: ResMut<Dialog>,
                           mut positions: Query<&mut Transform, With<CursorEntity>>) {

    for event in cursor_moved.iter() {
        for mut transform in positions.iter_mut() {

            // record the cursor's position on the screen
            cursor.x = event.position.x + cam.get_x() - (cam.screen_width()/2.);
            cursor.y = event.position.y + cam.get_y() - (cam.screen_height()/2.);

            // move the mouse graphic to the desired location
            transform.translation.x = cursor.x;
            transform.translation.y = cursor.y;

            match gamestate.get_status() {

                Status::MenuOpen => {
                    menu.hover_events(&mut commands, &asset_server, cursor.x, cursor.y);
                },
                Status::DialogOpen => {
                    dialog.hover_events(&mut commands, &asset_server, cursor.x, cursor.y);
                },
                Status::Playing => {
                },
            }
        }
    }

    for event in cursor_clicked.iter() {
        if event.state == ButtonState::Pressed && event.button == MouseButton::Left {

            match gamestate.get_status() {

                Status::MenuOpen => {
                    let response = menu.click_events(&mut commands,
                                                            &asset_server,
                                                            &cam,
                                                            &mut gamestate,
                                                            cursor.x,
                                                            cursor.y);

                    match response.as_str() {
                        "4k_mode" => {
                            toggle_option("4K Mode".to_string());
                        },
                        "borderless" => {
                            toggle_option("Borderless".to_string());
                        },
                        "vsync" => {
                            toggle_option("V-sync".to_string());
                        },
                        "fullscreen" => {
                            toggle_option("Fullscreen".to_string());
                        },
                        _ => {
                            continue;
                        }
                    };

                    menu.set_options_modified_flag();
                    menu.render(&mut commands, &asset_server, &cam);
                },

                Status::DialogOpen => {
                    let choice = dialog.click_events(cursor.x, cursor.y);
                    if choice == 0 {
                        gamestate.set_status(Status::Playing);
                    } else {
                        // TODO: implement this to load the next element in the dialog tree
                    }
                },

                Status::Playing => {
                },
            }
        }
    }
}
