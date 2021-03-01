use bevy::prelude::*;

pub struct CursorEntity;

pub struct Cursor {
    img: String,
    initialized: bool,
    entity: Entity,
}

#[derive(Default)]
pub struct MouseState {
    event_reader: EventReader<CursorMoved>,
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
            commands.despawn(self.entity);
            self.initialized = false;
        } 

        texture_handle = asset_server.load(self.img.as_str());

        commands
            .spawn(SpriteBundle {
                material: materials.add(texture_handle.into()),
                ..Default::default()
            })
            .with(CursorEntity);

        self.entity = commands.current_entity().unwrap();
        self.initialized = true;
    }
}

pub fn mouse_event_handler(mut state: Local<MouseState>,
                           mut positions: Query<&mut Transform, With<CursorEntity>>,
                           mouse_input_events: Res<Events<CursorMoved>>) {

    for event in state.event_reader.iter(&mouse_input_events) {
        for mut transform in positions.iter_mut() {
            transform.translation.x = event.position.x;
            transform.translation.y = event.position.y;
        }
    }
}
