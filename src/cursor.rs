use bevy::prelude::*;

pub struct CursorEntity;

pub struct Cursor {
    img: String,
    initialized: bool,
    entity: Entity,
}

impl Cursor {

    pub fn new(img: String, initialized: bool, entity: Entity) -> Cursor {
        Cursor { img: img, initialized: initialized, entity: entity }
    }

    pub fn render(&mut self, 
                  commands: &mut Commands, 
                  asset_server: &Res<AssetServer>, 
                  materials: &mut ResMut<Assets<ColorMaterial>>) {

        let texture_handle;

        if self.initialized {
            commands.entity(self.entity).despawn();
            self.initialized = false;
        } 

        texture_handle = asset_server.load(self.img.as_str());

        self.entity = commands
                         .spawn()
                         .insert_bundle(SpriteBundle {
                             material: materials.add(texture_handle.into()),
                             ..Default::default()
                         })
                         .insert(CursorEntity)
                         .id();

        self.initialized = true;
    }
}

pub fn mouse_event_handler(mut event_reader: EventReader<CursorMoved>,
                           mut positions: Query<&mut Transform, With<CursorEntity>>) {

    for event in event_reader.iter() {
        for mut transform in positions.iter_mut() {
            transform.translation.x = event.position.x;
            transform.translation.y = event.position.y;
        }
    }
}
