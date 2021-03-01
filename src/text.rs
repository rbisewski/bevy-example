use bevy::prelude::*;

pub struct Text {
    size: f32,
    color: Color,
    content: String,
}

impl Text {

    pub fn new(size: f32, color: Color, content: &str) -> Text {
        Text { size: size, color: color, content: content.to_string() }
    }

    pub fn render(&self, font: &str, commands: &mut Commands, asset_server: &Res<AssetServer>) {
        commands.spawn(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            text: bevy::prelude::Text {
                value: self.content.clone(),
                font: asset_server.load(font),
                style: TextStyle {
                    font_size: self.size,
                    color: self.color,
                    ..Default::default()
                },
            },
            ..Default::default()
        });
    }
}
