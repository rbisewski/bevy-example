use bevy::prelude::{
    AssetServer,
    Commands,
    Component,
    Entity,
    Res,
    ResMut,
    SpriteBundle,
    Transform,
};

//static MODE_DEFAULT: i8 = 0;
static MODE_CONTINUE: i8 = 1;

use crate::camera::Camera;
use crate::constants::Z_VALUE_MENU;
use crate::constants::Z_VALUE_MENU_ELEMENTS;
use crate::ui::UI;

#[derive(Component)]
pub struct MenuEntity;

pub struct Menu {
    img: String,
    initialized: bool,
    entity: Entity,
    mode: i8,
    menu_elements: Vec<UI>,
}

impl Menu {

    pub fn new(img: String) -> Menu {
        Menu { img, initialized: false, entity: Entity::from_raw(0), mode: MODE_CONTINUE, menu_elements: vec![] }
    }

    pub fn render(&mut self,
                  commands: &mut Commands,
                  asset_server: &Res<AssetServer>,
                  camera: &ResMut<Camera>) {

        // as a precaution, clear away all existing elements, if any
        self.hide(commands);

        self.entity = commands
                         .spawn()
                         .insert_bundle(SpriteBundle {
                             texture: asset_server.load(self.img.as_str()),
                             transform: Transform::from_xyz(camera.get_x(), camera.get_y(), Z_VALUE_MENU),
                             ..Default::default()
                         })
                         .insert(MenuEntity)
                         .id();

        self.initialized = true;

        match self.mode {
            0 => {
            },
            1 => {
                let mut continue_button = UI::new(String::from("Continue"),
                                                      String::from("img/ui/menu_button_continue.png"),
                                                 String::from("img/ui/menu_button_continue_hover.png"),
                                                    16.,
                                                     66.);
                continue_button.render(commands,
                                       asset_server,
                                       camera.get_x(),
                                       camera.get_y()+44.0,
                                       Z_VALUE_MENU_ELEMENTS);

                let mut save_button = UI::new(String::from("Save"),
                                                  String::from("img/ui/menu_button_save.png"),
                                             String::from("img/ui/menu_button_save_hover.png"),
                                                16.,
                                                 66.);
                save_button.render(commands,
                                   asset_server,
                                   camera.get_x(),
                                   camera.get_y()+20.0,
                                   Z_VALUE_MENU_ELEMENTS);

                let mut load_button = UI::new(String::from("Load"),
                                                  String::from("img/ui/menu_button_load.png"),
                                             String::from("img/ui/menu_button_load_hover.png"),
                                              16.,
                                               66.);
                load_button.render(commands,
                                   asset_server,
                                   camera.get_x(),
                                   camera.get_y()-4.0,
                                   Z_VALUE_MENU_ELEMENTS);

                let mut options_button = UI::new(String::from("Options"),
                                                     String::from("img/ui/menu_button_options.png"),
                                                String::from("img/ui/menu_button_options_hover.png"),
                                                   16.,
                                                    66.);
                options_button.render(commands,
                                      asset_server,
                                      camera.get_x(),
                                      camera.get_y()-28.0,
                                      Z_VALUE_MENU_ELEMENTS);

                let mut quit_button = UI::new(String::from("Quit"),
                                                     String::from("img/ui/menu_button_quit.png"),
                                                String::from("img/ui/menu_button_quit_hover.png"),
                                                   16.,
                                                    66.);
                quit_button.render(commands,
                                   asset_server,
                                   camera.get_x(),
                                   camera.get_y()-52.0,
                                   Z_VALUE_MENU_ELEMENTS);

                self.menu_elements = vec![continue_button, save_button, load_button, options_button, quit_button];
            },
            _ => {
            }
        }
    }

    pub fn hide(&mut self, commands: &mut Commands) {
        if !self.initialized {
            return
        }

        commands.entity(self.entity).despawn();
        self.initialized = false;

        for element in self.menu_elements.iter_mut() {
            element.free(commands);
        }

        self.menu_elements.clear();
    }

    pub fn visible(&mut self) -> bool {
        self.initialized
    }

    pub fn hover_events(&mut self, commands: &mut Commands, asset_server: &Res<AssetServer>, mouse_x: f32, mouse_y: f32) {
        if !self.visible() {
            return
        }

        for element in self.menu_elements.iter_mut() {
            let x = element.get_x();
            let y = element.get_y();
            let z = element.get_z();
            if element.mouse_is_hovering(mouse_x, mouse_y) {
                element.render_hover(commands, asset_server, x, y, z);

            } else if !element.mouse_is_hovering(mouse_x, mouse_y) {
                element.render(commands, asset_server, x, y, z);
            }
        }
    }

    pub fn click_events(&mut self, commands: &mut Commands, mouse_x: f32, mouse_y: f32) {
        if !self.visible() {
            return
        }

        let mut name = String::from("");

        for element in self.menu_elements.iter_mut() {
            if !element.mouse_is_hovering(mouse_x, mouse_y) {
                continue
            }
            name = String::from(element.get_name());
            break;
        }

        match name.as_str() {
            "Continue" => {
                self.hide(commands);
            },
            "Quit" => {
                std::process::exit(0);
            },
            _ => {

            }
        }
    }
}
