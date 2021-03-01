use bevy::prelude::*;
use bevy::{
    input::keyboard::KeyboardInput,
    input::ElementState::Released,
    input::keyboard::KeyCode::W,
    input::keyboard::KeyCode::S,
    input::keyboard::KeyCode::A,
    input::keyboard::KeyCode::D,
    input::keyboard::KeyCode::Up,
    input::keyboard::KeyCode::Down,
    input::keyboard::KeyCode::Right,
    input::keyboard::KeyCode::Left,
};

static GFX_SCALE: f32 = 2.0;
static PIXELS_TRANSLATED: f32 = 8.0;

pub struct CameraEntity;

pub struct Camera {
    x: f32,
    y: f32,
    z: f32,
    ui: Entity,
    twodee: Entity,
}

#[derive(Default)]
pub struct CameraState {
    event_reader: EventReader<KeyboardInput>,
}

impl Camera {

    pub fn new(x: f32, y: f32, z: f32) -> Camera {
        Camera { x: x, y: y, z: z, ui: Entity::new(0), twodee: Entity::new(0) }
    }

    pub fn start(&mut self, commands: &mut Commands) {
        commands.spawn(CameraUiBundle::default());
        self.ui = commands.current_entity().unwrap();

        commands
            .spawn(Camera2dBundle {
                transform: Transform {
                    translation: Vec3::new(self.x, self.y, self.z),
                    scale: Vec3::splat(1.0/GFX_SCALE),
                    ..Default::default()
            },
            ..Default::default()
        })
        .with(CameraEntity);
        self.twodee = commands.current_entity().unwrap();
    }
}

pub fn camera_event_handler(mut state: Local<CameraState>,
                            mut cam: ResMut<Camera>,
                            mut positions: Query<&mut Transform, With<CameraEntity>>,
                            keyboard_input_events: Res<Events<KeyboardInput>>) {

    for event in state.event_reader.iter(&keyboard_input_events) {

        // ignored released events for now
        if event.state == Released {
            continue;
        }

        for mut transform in positions.iter_mut() {
            match event.key_code {

                // move camera
                Some(Up) | Some(W) => {
                    cam.y = cam.y + PIXELS_TRANSLATED;
                    transform.translation.y = cam.y;
                },
                Some(Down) | Some(S) => {
                    cam.y = cam.y - PIXELS_TRANSLATED;
                    transform.translation.y = cam.y;
                },
                Some(Right) | Some(D) => {
                    cam.x = cam.x + PIXELS_TRANSLATED;
                    transform.translation.x = cam.x;
                },
                Some(Left) | Some(A) => {
                    cam.x = cam.x - PIXELS_TRANSLATED;
                    transform.translation.x = cam.x;
                },

                _ => (),
            }
        }
    }
}
