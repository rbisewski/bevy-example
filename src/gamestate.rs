use bevy::prelude::{
    AssetServer,
    Commands,
    Res, 
    ResMut, 
};

use crate::camera::Camera;
use crate::dialog::Dialog;
use crate::menu::Menu;

#[derive(Copy, Clone, PartialEq)]
pub enum Status {
    Playing,
    MenuOpen,
    DialogOpen,
}

pub struct Gamestate {
    status: Status,
    changed: bool,
}

impl Gamestate {

    pub fn new() -> Gamestate {
        Gamestate {
            status: Status::MenuOpen,
            changed: false,
        }
    }

    pub fn get_status(&self) -> Status {
        self.status
    }

    pub fn set_status(&mut self, status: Status) {
        self.status = status;
        self.changed = true;
    }
}

pub fn gamestate_handler(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    cam: ResMut<Camera>,
    mut gamestate: ResMut<Gamestate>,
    mut menu: ResMut<Menu>,
    mut dialog: ResMut<Dialog>
) {
        if !gamestate.changed {
            return;
        }

        match gamestate.get_status() {
            Status::MenuOpen => {
                dialog.free(&mut commands);
                menu.reset_mode();
                menu.render(&mut commands, &asset_server, &cam);
                gamestate.changed = false;
            },
            Status::DialogOpen => {
                dialog.render(&mut commands, &asset_server, &cam);
                menu.hide(&mut commands);
                gamestate.changed = false;
            },
            Status::Playing => {
                dialog.free(&mut commands);
                gamestate.changed = false;
            },
        }
}