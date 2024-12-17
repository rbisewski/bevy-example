use bevy::{math::{Quat, Vec3}, prelude::{
    AssetServer, Color, Commands, Entity, Res, Text2d, TextFont, Transform
}, text::TextColor};

use crate::constants::{
    DIALOG_HOVER_COLOR,
    DIALOG_CHOICE_COLOR,
    DIALOG_CHOICE_HEIGHT,
    DIALOG_CHOICE_WIDTH,
    MOUSE_GFX_HEIGHT,
    MOUSE_GFX_WIDTH,
    TEXT_DIALOG_SCALE,
};

pub struct Text {
    initialized: bool,
    entity: Entity,
    size: f32,
    color: Color,
    content: String,
    hoverable: bool,
    hovered: bool,
    x: f32,
    y: f32,
    z: f32,
}

impl Text {

    pub fn new(size: f32, color: Color, content: &str, hoverable: bool) -> Text {
        Text {
            initialized: false,
            entity: Entity::from_raw(0), 
            size,
            color, 
            content: content.to_string(),
            hoverable,
            hovered: false,
            x: 0.,
            y: 0.,
            z: 0.,
        }
    }

    pub fn render(&mut self, font: &str, commands: &mut Commands, asset_server: &Res<AssetServer>, x: f32, y: f32, z: f32) {

        if self.initialized {
            self.free(commands);
        }

        self.entity = commands.spawn((
            Text2d::from(self.content.clone()),
            TextFont {
                font: asset_server.load(font.to_owned()),
                font_size: self.size,
                ..Default::default()
            },
            TextColor(self.color),
            Transform {
                rotation: Quat::from_rotation_z(0.),
                scale: Vec3::new(TEXT_DIALOG_SCALE,TEXT_DIALOG_SCALE,TEXT_DIALOG_SCALE),
                translation: Vec3::new(x, y, z),
            }
        )).id();

        self.initialized = true;
        self.x = x;
        self.y = y;
        self.z = z;
    }

    pub fn set_content(&mut self, content: String) {
        self.content = content;
    }

    pub fn lines(&self) -> i8 {
        (self.content.matches('\n').count() as i8) + 1
    }

    pub fn free(&mut self, commands: &mut Commands) {
        if !self.initialized {
            return
        }

        commands.entity(self.entity).despawn();

        self.initialized = false;
    }

    pub fn get_hover(&self) -> bool {
        self.hovered
    }

    pub fn set_hover(&mut self, hovered: bool) {
        self.hovered = hovered;

        if self.hovered {
            self.color = DIALOG_HOVER_COLOR;
        } else {
            self.color = DIALOG_CHOICE_COLOR;
        }
    }

    pub fn get_x(&self) -> f32 {
        self.x
    }

    pub fn get_y(&self) -> f32 {
        self.y
    }

    pub fn mouse_is_hovering(&self, x: f32, y: f32) -> bool {
        if !self.hoverable || !self.initialized {
            return false;
        }

        let x_offset = 24.;
        let y_offset = 0.;

        if (x + x_offset + MOUSE_GFX_WIDTH >= self.x)
        && (x + x_offset + MOUSE_GFX_WIDTH <= self.x + DIALOG_CHOICE_WIDTH)
        && (y + y_offset + MOUSE_GFX_HEIGHT >= self.y)
        && (y + y_offset + MOUSE_GFX_HEIGHT <= self.y + DIALOG_CHOICE_HEIGHT) {
            return true;
        }

        false
    }
}
