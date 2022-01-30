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
};

use crate::camera::Camera;

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

    pub fn new(img: String, initialized: bool, entity: Entity) -> Cursor {
        Cursor { img, initialized, entity, x: 0.0, y: 0.0 }
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
                             transform: Transform::from_xyz(self.x, self.y, 0.0),
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

pub fn mouse_event_handler(mut event_reader: EventReader<CursorMoved>,
                           mut cursor: ResMut<Cursor>,
                           cam: Res<Camera>,
                           mut positions: Query<&mut Transform, With<CursorEntity>>) {

    for event in event_reader.iter() {
        for mut transform in positions.iter_mut() {

            // record the cursor's position on the screen
            cursor.x = event.position.x + cam.get_x() - (cam.screen_width()/2.0);
            cursor.y = event.position.y + cam.get_y() - (cam.screen_height()/2.0);

            // move the mouse graphic to the desired location
            transform.translation.x = event.position.x + cam.get_x() - (cam.screen_width()/2.0);
            transform.translation.y = event.position.y + cam.get_y() - (cam.screen_height()/2.0);
        }
    }
}
