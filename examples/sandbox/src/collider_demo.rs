use runa_core::{
    components::{Collider2D, SpriteRenderer, Transform},
    glam::Vec3,
    ocs::{Object, World},
};
use runa_engine::RunaArchetype;

pub fn create_collider_demo_box() -> Object {
    let mut transform = Transform::default();
    transform.position = Vec3::new(32.0, 0.0, 0.0);

    Object::new("Collider Demo Box")
        .with(transform)
        .with(SpriteRenderer {
            texture: Some(runa_asset::load_image!("assets/art/Tester2.png")),
            texture_path: Some("assets/art/Tester2.png".to_string()),
            pixels_per_unit: 16.0,
        })
        .with(Collider2D::new(16.0, 16.0))
}

#[derive(RunaArchetype)]
#[runa(name = "collider_demo_box")]
pub struct ColliderDemoBoxArchetype;

impl ColliderDemoBoxArchetype {
    pub fn create(world: &mut World) -> u64 {
        world.spawn(create_collider_demo_box())
    }
}
