use bevy::prelude::{
    Commands,
    Component,
    Entity,
    EventReader,
    Query,
    ResMut,
    Camera2d,
    Transform,
    Vec3,
    With,
    OrthographicProjection,
    Resource,
};
use bevy::{
    input::keyboard::KeyboardInput,
    input::ButtonState::Released,
    input::keyboard::KeyCode::KeyW,
    input::keyboard::KeyCode::KeyS,
    input::keyboard::KeyCode::KeyA,
    input::keyboard::KeyCode::KeyD,
    input::keyboard::KeyCode::ArrowUp,
    input::keyboard::KeyCode::ArrowDown,
    input::keyboard::KeyCode::ArrowRight,
    input::keyboard::KeyCode::ArrowLeft,
};

static GFX_SCALE: f32 = 2.0;
static PIXELS_TRANSLATED: f32 = 8.0;

use crate::gamestate::{Status, Gamestate};

#[derive(Component)]
pub struct CameraEntity;

#[derive(Resource)]
pub struct Camera {
    x: f32,
    y: f32,
    z: f32,
    twodee: Entity,
    screen_height: f32,
    screen_width: f32,
}

impl Camera {

    pub fn new(x: f32, y: f32, z: f32, screen_height: f32, screen_width: f32) -> Camera {
        Camera { x, y, z, twodee: Entity::from_raw(0), screen_height, screen_width }
    }

    pub fn start(&mut self, commands: &mut Commands) {
        self.twodee = commands.spawn((
            Camera2d,
            Transform::from_translation(Vec3::new(self.x, self.y, self.z)),
            OrthographicProjection {
                scale: 1.0/GFX_SCALE,
                ..OrthographicProjection::default_2d()
            },
        )).insert(CameraEntity).id();
    }

    pub fn get_x(&self) -> f32 {
        self.x
    }

    pub fn get_y(&self) -> f32 {
        self.y
    }

    pub fn screen_height(&self) -> f32 {
        self.screen_height
    }

    pub fn screen_width(&self) -> f32 {
        self.screen_width
    }
}

pub fn camera_event_handler(mut cam: ResMut<Camera>,
                            gamestate: ResMut<Gamestate>,
                            mut event_reader: EventReader<KeyboardInput>,
                            mut positions: Query<&mut Transform, With<CameraEntity>>) {

    for event in event_reader.read() {

        // ignored released events for now
        if event.state == Released {
            continue;

        // ignore keyboard events whilst the menu or dialog is visible
        } else if gamestate.get_status() != Status::Playing {
            continue
        }

        for mut transform in positions.iter_mut() {
            match event.key_code {

                // move camera
                ArrowUp | KeyW => {
                    cam.y += PIXELS_TRANSLATED;
                    transform.translation.y = cam.y;
                },
                ArrowDown | KeyS => {
                    cam.y -= PIXELS_TRANSLATED;
                    transform.translation.y = cam.y;
                },
                ArrowRight | KeyD => {
                    cam.x += PIXELS_TRANSLATED;
                    transform.translation.x = cam.x;
                },
                ArrowLeft | KeyA => {
                    cam.x -= PIXELS_TRANSLATED;
                    transform.translation.x = cam.x;
                },

                _ => (),
            }
        }
    }
}
