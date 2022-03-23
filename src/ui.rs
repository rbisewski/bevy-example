use bevy::prelude::{
    AssetServer,
    Commands,
    Component,
    Entity,
    Res,
    SpriteBundle,
    Transform,
};

#[derive(Component)]
pub struct UIEntity;

pub struct UI {
    name: String,
    img: String,
    img_hover: String,
    initialized: bool,
    hovered: bool,
    entity: Entity,
    entity_hover: Entity,
    height: f32,
    width: f32,
    x: f32,
    y: f32,
    z: f32,
    xoffset: f32,
}

impl UI {

    pub fn new(name: String, img: String, img_hover: String, height: f32, width: f32) -> UI {
        UI { name, img, img_hover, initialized: false, hovered: false, entity: Entity::from_raw(0), entity_hover: Entity::from_raw(0), height, width, x: 0., y: 0., z: 0., xoffset: 0.}
    }

    pub fn render(&mut self, commands: &mut Commands, asset_server: &Res<AssetServer>, x: f32, y: f32, z: f32) {

        if !self.initialized {

            self.entity = commands.spawn()
                                  .insert_bundle(SpriteBundle {
                                      texture: asset_server.load(self.img.as_str()),
                                      transform: Transform::from_xyz(x+self.xoffset, y, z),
                                      ..Default::default()
                                  })
                                  .insert(UIEntity)
                                  .id();

            self.entity_hover = commands.spawn()
                                        .insert(UIEntity)
                                        .id();
            self.initialized = true;

        }

        if self.hovered {
            commands.entity(self.entity_hover).remove_bundle::<SpriteBundle>();
        }

        self.hovered = false;
        self.x = x;
        self.y = y;
        self.z = z;
    }

    pub fn render_hover(&mut self, commands: &mut Commands, asset_server: &Res<AssetServer>, x: f32, y: f32, z: f32) {

        if self.img_hover.is_empty() {
            return;
        }

        commands.entity(self.entity_hover)
                .insert_bundle(SpriteBundle {
                    texture: asset_server.load(self.img_hover.as_str()),
                    transform: Transform::from_xyz(x+self.xoffset, y, z+0.01),
                    ..Default::default()
                });
        self.hovered = true;
        self.x = x;
        self.y = y;
        self.z = z;
    }

    pub fn free(&mut self, commands: &mut Commands) {
        if !self.initialized {
            return
        }

        commands.entity(self.entity).despawn();
        commands.entity(self.entity_hover).despawn();
        self.initialized = false;
        self.hovered = false;
        self.x = 0.;
        self.y = 0.;
        self.z = 0.;
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn get_x(&self) -> f32 {
        self.x
    }

    pub fn get_y(&self) -> f32 {
        self.y
    }

    pub fn get_z(&self) -> f32 {
        self.z
    }

    pub fn set_xoffset(&mut self, x: f32) {
        self.xoffset = x;
    }

    pub fn mouse_is_hovering(&self, x: f32, y: f32) -> bool {
        let mouse_gfx_height: f32 = 16.;
        let mouse_gfx_width: f32 = 28.;

        if (x+mouse_gfx_width >= self.x)
        && (x+mouse_gfx_width <= self.x + self.width)
        && (y+mouse_gfx_height >= self.y)
        && (y+mouse_gfx_height <= self.y + self.height) {
            return true;
        }

        false
    }
}