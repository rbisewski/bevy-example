use bevy::prelude::*;

pub struct Text {
    size: f32,
    color: Color,
    content: String,
}

impl Text {

    pub fn new(size: f32, color: Color, content: &str) -> Text {
        Text { size, color, content: content.to_string() }
    }

    pub fn render(&self, font: &str, commands: &mut Commands, asset_server: &Res<AssetServer>) {
        commands
            .spawn()
            .insert_bundle(TextBundle {
                style: Style {
                    align_self: AlignSelf::FlexEnd,
                    ..Default::default()
                },
                text: bevy::prelude::Text::with_section (
                    self.content.clone(),
                    TextStyle {
                        font: asset_server.load(font),
                        font_size: self.size,
                        color: self.color,
                    },
                    TextAlignment::default()
                ),
                ..Default::default()
            });
    }
}
