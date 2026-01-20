use glam::{Quat, Vec2};
use runa_core::components::sprite_renderer::SpriteRenderer;
use runa_core::components::transform::Transform;
use runa_core::ocs::object::Object;
use runa_core::ocs::script::Script;

pub struct RotatingSprite2 {
    pub speed: f32,
}

impl RotatingSprite2 {
    pub fn new(degrees_per_second: f32) -> Self {
        Self {
            speed: degrees_per_second,
        }
    }

    pub fn booster(&mut self) {
        self.speed *= 2.0;
    }
}

impl Script for RotatingSprite2 {
    fn construct(&self, _object: &mut runa_core::ocs::object::Object) {
        _object
            .add_component(Transform::default())
            .add_component(SpriteRenderer {
                texture: Some(runa_asset::loader::load_image("assets/Tester2.png")),
            });
    }

    fn start(&mut self, _object: &mut Object) {
        if let Some(transform) = _object.get_component_mut::<Transform>() {
            transform.position = Vec2 { x: 2.0, y: 5.0 };
            transform.scale = Vec2 { x: 1.0, y: 1.0 };
        }
    }

    fn update(&mut self, _object: &mut Object, dt: f32) {
        if let Some(transform) = _object.get_component_mut::<Transform>() {
            // // Применяем поворот
            // transform.rotation *= Quat::from_rotation_z(self.speed * dt);

            // // ⚠️ ОБЯЗАТЕЛЬНО нормализуем!
            // transform.rotation = transform.rotation.normalize();

            // if transform.rotation.y >= 90.0 {
            //     self.booster();
            // }
        }
    }
}
