use std::fs;
use json::parse;

use bevy::prelude::{
    AssetServer,
    Commands,
    Res,
    ResMut,
};

use crate::camera::Camera;
use crate::constants::{Z_VALUE_MENU, Z_VALUE_MENU_ELEMENTS, DIALOG_MAIN_TEXT_COLOR, DIALOG_CHOICE_COLOR};
use crate::text::Text;
use crate::ui::UI;

pub struct Dialog {
    initialized: bool,
    ui: UI,
    text: Text,
    dialog_choices: Vec<Text>,
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

        let text = Text::new(24., DIALOG_MAIN_TEXT_COLOR, "", false);

        Dialog {initialized: false, ui, text, dialog_choices: vec![]}
    }

    pub fn load_dialog(&mut self, commands: &mut Commands, filename: String, number: i8) {

        // the zeroth dialog option is reserved for null
        if number == 0 {
            return;
        }

        let number_as_string: String = number.to_string();

        let contents = match fs::read_to_string(filename) {
            Ok(s) => s,
            _ => return,
        };

        let parsed = match parse(contents.as_str()) {
            Ok(j) => j,
            _ => return,
        };

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
            d.free(commands);
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

            let choice_text = match dialog_entry["choices"][choice_entry]["text"].as_str() {
                Some(s) => s.to_string(),
                _ => break,
            };

            self.dialog_choices.push(Text::new(24., DIALOG_CHOICE_COLOR, &choice_text, true));

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

        let text_x = x-166.;
        let mut text_y = y+40.;

        self.text.render("fonts/eight_bit.ttf", commands, asset_server, text_x, text_y, Z_VALUE_MENU_ELEMENTS);

        // add a space of 6px between the dialog text and choices
        text_y -= 6.;

        // each line of text is 12px plus 2px of space
        text_y -= (self.text.lines() as f32) * 14.;
        for d in self.dialog_choices.iter_mut() {
            d.render("fonts/eight_bit.ttf", commands, asset_server, text_x, text_y, Z_VALUE_MENU_ELEMENTS);
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
            d.free(commands);
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
            let was_hovering = d.get_hover();
            let is_hovering = d.mouse_is_hovering(mouse_x, mouse_y);

            if was_hovering && !is_hovering {
                d.set_hover(false);
                d.render("fonts/eight_bit.ttf", commands, asset_server, d.get_x(), d.get_y(), Z_VALUE_MENU_ELEMENTS);

            } else if !was_hovering && is_hovering {
                d.set_hover(true);
                d.render("fonts/eight_bit.ttf", commands, asset_server, d.get_x(), d.get_y(), Z_VALUE_MENU_ELEMENTS);
            }
        }
    }

    pub fn click_events(&mut self,
                        mouse_x: f32,
                        mouse_y: f32) -> i8 {

        if !self.visible() {
            return 0;
        }

        // TODO: make this better
        let mut count = 1;
        for d in self.dialog_choices.iter_mut() {
            if d.mouse_is_hovering(mouse_x, mouse_y) {
                return count;
            }
            count += 1;
        }

        0
    }
}
