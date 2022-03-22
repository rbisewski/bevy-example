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
static MODE_OPTIONS: i8 = 2;
static MODE_QUIT: i8 = 3;

use crate::camera::Camera;
use crate::constants::Z_VALUE_MENU;
use crate::constants::Z_VALUE_MENU_ELEMENTS;
use crate::options::get_options;
use crate::ui::UI;

#[derive(Component)]
pub struct MenuEntity;

pub struct Menu {
    img: String,
    initialized: bool,
    entity: Entity,
    mode: i8,
    borderless: bool,
    menu_elements: Vec<UI>,
}

impl Menu {

    pub fn new(img: String) -> Menu {
        Menu { img, 
               initialized: false,
               entity: Entity::from_raw(0),
               mode: MODE_CONTINUE,
               borderless: false,
               menu_elements: vec![] }
    }

    pub fn render(&mut self,
                  commands: &mut Commands,
                  asset_server: &Res<AssetServer>,
                  camera: &ResMut<Camera>) {

        // as a precaution, clear away all existing elements, if any
        self.hide(commands);

        match self.mode {
            //
            // MODE_DEFAULT
            //
            0 => {
            },
            //
            // MODE_CONTINUE
            //
            1 => {
                self.img = String::from("img/ui/menu_main.png");

                let mut continue_button = UI::new(String::from("Continue"),
                                                      String::from("img/ui/menu_button_continue.png"),
                                                 String::from("img/ui/menu_button_continue_hover.png"),
                                                    16.,
                                                     66.);
                continue_button.render(commands,
                                       asset_server,
                                       camera.get_x(),
                                       camera.get_y()+44.,
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
                                   camera.get_y()-52.,
                                   Z_VALUE_MENU_ELEMENTS);

                self.menu_elements = vec![continue_button, save_button, load_button, options_button, quit_button];
            },
            //
            // MODE_OPTIONS
            //
            2 => {
                self.img = String::from("img/ui/menu_options.png");

                let unchecked_box = "img/ui/menu_checkbox_false.png";
                let unchecked_box_hover = "img/ui/menu_checkbox_false_hover.png";
                let checked_box = "img/ui/menu_checkbox_true.png";
                let checked_box_hover = "img/ui/menu_checkbox_true_hover.png";

                let options = get_options();

                let mut back_button = UI::new(String::from("Back"),
                                                  String::from("img/ui/menu_button_back.png"),
                                             String::from("img/ui/menu_button_back_hover.png"),
                                                16.,
                                                 66.);
                back_button.render(commands,
                                   asset_server,
                                   camera.get_x(),
                                   camera.get_y()+44.,
                                   Z_VALUE_MENU_ELEMENTS);

                let mut button_gfx = if options.four_k_mode { checked_box } else { unchecked_box };
                let mut button_hover_gfx = if options.four_k_mode { checked_box_hover } else { unchecked_box_hover };
                let mut four_k_mode_button = UI::new(String::from("4K Mode"),
                                                         button_gfx.to_string(),
                                                         button_hover_gfx.to_string(),
                                                       16.,
                                                        17.);
                four_k_mode_button.set_xoffset(-24.);

                four_k_mode_button.render(commands,
                                   asset_server,
                                   camera.get_x()+67.5,
                                   camera.get_y()+17.5,
                                   Z_VALUE_MENU_ELEMENTS);

                button_gfx = if options.borderless { checked_box } else { unchecked_box };
                button_hover_gfx = if options.borderless { checked_box_hover } else { unchecked_box_hover };
                let mut borderless_button = UI::new(String::from("Borderless"),
                                                        button_gfx.to_string(),
                                                   button_hover_gfx.to_string(),
                                                      16.,
                                                       17.);
                borderless_button.set_xoffset(-24.);

                borderless_button.render(commands,
                                   asset_server,
                                   camera.get_x()+67.5,
                                   camera.get_y()-2.5,
                                   Z_VALUE_MENU_ELEMENTS);

                button_gfx = if options.vsync { checked_box } else { unchecked_box };
                button_hover_gfx = if options.vsync { checked_box_hover } else { unchecked_box_hover };
                let mut vsync_button = UI::new(String::from("V-sync"),
                                                        button_gfx.to_string(),
                                                   button_hover_gfx.to_string(),
                                                       16.,
                                                        17.);
                vsync_button.set_xoffset(-24.);

                vsync_button.render(commands,
                                   asset_server,
                                   camera.get_x()+67.5,
                                   camera.get_y()-22.5,
                                   Z_VALUE_MENU_ELEMENTS);

                button_gfx = if options.fullscreen { checked_box } else { unchecked_box };
                button_hover_gfx = if options.fullscreen { checked_box_hover } else { unchecked_box_hover };
                let mut fullscreen_button = UI::new(String::from("Fullscreen"),
                                                        button_gfx.to_string(),
                                                   button_hover_gfx.to_string(),
                                                       16.,
                                                        17.);
                fullscreen_button.set_xoffset(-24.);

                fullscreen_button.render(commands,
                                   asset_server,
                                   camera.get_x()+67.5,
                                   camera.get_y()-42.5,
                                   Z_VALUE_MENU_ELEMENTS);

                self.menu_elements = vec![back_button, four_k_mode_button, borderless_button, vsync_button, fullscreen_button];
            },
            //
            // MODE_QUIT
            //
            3 => {
                self.img = String::from("img/ui/menu_quit.png");

                let mut yes_quit_button = UI::new(String::from("Yes, quit"),
                                                      String::from("img/ui/menu_button_yes_quit.png"),
                                                 String::from("img/ui/menu_button_yes_quit_hover.png"),
                                                    16.,
                                                     66.);
                yes_quit_button.render(commands,
                                      asset_server,
                                      camera.get_x(),
                                      camera.get_y()-12.,
                                      Z_VALUE_MENU_ELEMENTS);

                let mut no_stay_button = UI::new(String::from("No, stay"),
                                                     String::from("img/ui/menu_button_no_stay.png"),
                                                String::from("img/ui/menu_button_no_stay_hover.png"),
                                                   16.,
                                                    66.);
                no_stay_button.render(commands,
                                   asset_server,
                                   camera.get_x(),
                                   camera.get_y()-36.,
                                   Z_VALUE_MENU_ELEMENTS);

                self.menu_elements = vec![yes_quit_button, no_stay_button];
            },
            _ => {
            }
        }

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

    pub fn is_borderless(&self) -> bool {
        self.borderless
    }

    pub fn set_mode(&mut self, mode: i8) {
        self.mode = mode;
    }
    pub fn reset_mode(&mut self) {
        self.mode = MODE_CONTINUE;
    }

    pub fn visible(&self) -> bool {
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

    pub fn click_events(&mut self,
                        commands: &mut Commands,
                        asset_server: &Res<AssetServer>,
                        cam: &ResMut<Camera>,
                        mouse_x: f32,
                        mouse_y: f32) -> String {

        if !self.visible() {
            return String::from("")
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
            "Options" => {
                self.set_mode(MODE_OPTIONS);
                self.render(commands, asset_server, cam);
            },
            "Quit" => {
                self.set_mode(MODE_QUIT);
                self.render(commands, asset_server, cam);
            },
            "Yes, quit" => {
                std::process::exit(0);
            },
            "Back" | "No, stay" => {
                self.reset_mode();
                self.render(commands, asset_server, cam);
            },
            "4K Mode" => {
                return String::from("4k_mode");
            },
            "Borderless" => {
                return String::from("borderless");
            },
            "V-sync" => {
                return String::from("vsync");
            },
            "Fullscreen" => {
                return String::from("fullscreen");
            },
            _ => {
            }
        }

        String::from("")
    }
}
