use glam::Vec2;
use runa_core::components::sprite_renderer::SpriteRenderer;
use runa_core::components::transform::Transform;
use runa_core::ocs::object::Object;
use runa_core::ocs::script::Script;

pub struct RotatingSprite {
    pub speed: f32,
}

impl RotatingSprite {
    pub fn new(degrees_per_second: f32) -> Self {
        Self {
            speed: degrees_per_second,
        }
    }
}

impl Script for RotatingSprite {
    fn construct(&self, _object: &mut runa_core::ocs::object::Object) {
        _object
            .add_component(Transform::default())
            .add_component(SpriteRenderer {
                texture: Some(runa_asset::loader::load_image("assets/player.png")),
            });
    }

    fn start(&mut self, _object: &mut Object) {
        if let Some(transform) = _object.get_component_mut::<Transform>() {
            transform.position = Vec2 { x: 0.0, y: 0.0 };
            transform.rotation = 0.0;
        }
    }

    fn update(&mut self, _object: &mut Object, dt: f32) {
        if let Some(transform) = _object.get_component_mut::<Transform>() {
            transform.rotation += self.speed * dt;
        }
    }
}
