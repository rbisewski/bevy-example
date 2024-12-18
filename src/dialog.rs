use bevy::prelude::{
    AssetServer,
    Commands,
    Res,
    ResMut, Resource,
};

use crate::camera::Camera;
use crate::constants::{Z_VALUE_MENU, Z_VALUE_MENU_ELEMENTS, DIALOG_MAIN_TEXT_COLOR, DIALOG_CHOICE_COLOR, DIALOG_CHOICE_HEIGHT, DIALOG_FONT_SIZE};
use crate::text::Text;
use crate::ui::UI;

struct DialogChoice {
    text: Text,
    next: i64,
}

#[derive(Resource)]
pub struct Dialog {
    initialized: bool,
    ui: UI,
    text: Text,
    dialog_choices: Vec<DialogChoice>,
}

impl Dialog {

    pub fn new() -> Dialog {

        let ui = UI::new( 
            "Dialog Box".to_string(),
            "img/ui/menu_dialog.png".to_string(),
            "".to_string(),
            140.,
            552.,
        );

        let text = Text::new(DIALOG_FONT_SIZE, DIALOG_MAIN_TEXT_COLOR, "", false);

        Dialog {initialized: false, ui, text, dialog_choices: vec![]}
    }

    pub fn load_dialog(&mut self, commands: &mut Commands, number: i8) {

        // the zeroth dialog option is reserved for null
        if number == 0 {
            return;
        }

        let number_as_string: String = number.to_string();

        let contents = include_str!("../dialog/generic.json");

        let parsed: serde_json::Value = serde_json::from_str(contents).expect("Unable to open the dialog file.");

        let dialog_entry = &parsed[number_as_string];

        // free memory used from existing dialog main content
        self.text.free(commands);

        // load text content
        let content = match dialog_entry["content"].as_str() {
            Some(s) => s.to_string(),
            _ => return,
        };
        self.text.set_content(content);

        // free memory used from existing dialog choices
        for d in self.dialog_choices.iter_mut() {
            d.text.free(commands);
            d.next = 0;
        }
        self.dialog_choices.clear();

        // load new dialog choices into memory
        let mut count = 1;
        loop {
            let choice_entry = [
                number.to_string(),
                ".".to_string(),
                count.to_string(),
            ].concat();

            let choice_text = match dialog_entry["choices"][&choice_entry]["text"].as_str() {
                Some(s) => s.to_string(),
                _ => break,
            };

            let choice_next = dialog_entry["choices"][&choice_entry]["next"].as_i64().unwrap_or_default();

            self.dialog_choices.push(
                DialogChoice {
                    text: Text::new(DIALOG_FONT_SIZE, DIALOG_CHOICE_COLOR, &choice_text, true),
                    next: choice_next,
                }
            );

            count += 1;
        }
    }

    pub fn render(&mut self, commands: &mut Commands, asset_server: &Res<AssetServer>, camera: &ResMut<Camera>) {

        if self.initialized {
            self.free(commands);
        }

        let x = camera.get_x();
        let y = camera.get_y()-76.;

        self.ui.render(commands, asset_server, x, y, Z_VALUE_MENU);

        let mut text_x = x-72.;
        let mut text_y = y+16.;

        self.text.render("fonts/eight_bit.ttf", commands, asset_server, text_x, text_y, Z_VALUE_MENU_ELEMENTS);

        text_x -= 46.;

        // each line of text is 20 / 2 = 10px of space
        text_y -= (self.text.lines() as f32) * 10.;

        for d in self.dialog_choices.iter_mut() {
            d.text.render("fonts/eight_bit.ttf", commands, asset_server, text_x, text_y, Z_VALUE_MENU_ELEMENTS);
            text_x -= DIALOG_CHOICE_HEIGHT;
            text_y -= 14.;
        }

        self.initialized = true;
    }

    pub fn free(&mut self, commands: &mut Commands) {

        if !self.initialized {
            return
        }

        self.ui.free(commands);
        self.text.free(commands);

        for d in self.dialog_choices.iter_mut() {
            d.text.free(commands);
            d.next = 0;
        }

        self.initialized = false;
    }

    pub fn visible(&self) -> bool {
        self.initialized
    }

    pub fn hover_events(&mut self, commands: &mut Commands, asset_server: &Res<AssetServer>, mouse_x: f32, mouse_y: f32) {
        if !self.visible() {
            return;
        }

        for d in self.dialog_choices.iter_mut() {
            let was_hovering = d.text.get_hover();
            let is_hovering = d.text.mouse_is_hovering(mouse_x, mouse_y);

            if was_hovering && !is_hovering {
                d.text.set_hover(false);
                d.text.render("fonts/eight_bit.ttf", commands, asset_server, d.text.get_x(), d.text.get_y(), Z_VALUE_MENU_ELEMENTS);

            } else if !was_hovering && is_hovering {
                d.text.set_hover(true);
                d.text.render("fonts/eight_bit.ttf", commands, asset_server, d.text.get_x(), d.text.get_y(), Z_VALUE_MENU_ELEMENTS);
            }
        }
    }

    pub fn click_events(&mut self,
                        mouse_x: f32,
                        mouse_y: f32) -> i64 {

        if !self.visible() {
            return 0;
        }

        for d in self.dialog_choices.iter_mut() {
            if d.text.mouse_is_hovering(mouse_x, mouse_y) {
                return d.next;
            }
        }

        0
    }
}
