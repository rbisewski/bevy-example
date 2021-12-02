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

impl Camera {

    pub fn new(x: f32, y: f32, z: f32) -> Camera {
        Camera { x, y, z, ui: Entity::new(0), twodee: Entity::new(0) }
    }

    pub fn start(&mut self, commands: &mut Commands) {

        self.ui = commands
                     .spawn()
                     .insert_bundle(UiCameraBundle::default())
                     .id();

        let mut twodee_cam = OrthographicCameraBundle::new_2d();
        let mut transform = Transform::from_translation(Vec3::new(self.x, self.y, self.z));
        transform.scale = Vec3::splat(1.0/GFX_SCALE);
        twodee_cam.transform = transform;

        self.twodee = commands
                         .spawn()
                         .insert_bundle(twodee_cam)
                         .insert(CameraEntity)
                         .id();
    }
}

pub fn camera_event_handler(mut cam: ResMut<Camera>,
                            mut event_reader: EventReader<KeyboardInput>,
                            mut positions: Query<&mut Transform, With<CameraEntity>>) {

    for event in event_reader.iter() {

        // ignored released events for now
        if event.state == Released {
            continue;
        }

        for mut transform in positions.iter_mut() {
            match event.key_code {

                // move camera
                Some(Up) | Some(W) => {
                    cam.y += PIXELS_TRANSLATED;
                    transform.translation.y = cam.y;
                },
                Some(Down) | Some(S) => {
                    cam.y -= PIXELS_TRANSLATED;
                    transform.translation.y = cam.y;
                },
                Some(Right) | Some(D) => {
                    cam.x += PIXELS_TRANSLATED;
                    transform.translation.x = cam.x;
                },
                Some(Left) | Some(A) => {
                    cam.x -= PIXELS_TRANSLATED;
                    transform.translation.x = cam.x;
                },

                _ => (),
            }
        }
    }
}
