use runa_render_api::queue::RenderQueue;

use crate::{
    components::{sprite_renderer::SpriteRenderer, transform::Transform},
    ocs::{object::Object, script::Script},
};

#[derive(Default)]
pub struct World {
    pub objects: Vec<Object>,
}

impl World {
    pub fn default() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn spawn(&mut self, script: Box<dyn Script>) -> &Object {
        let mut object = Object::new();
        object.set_script(script);

        self.objects.push(object);
        self.objects.get(self.objects.len() - 1).unwrap()
    }

    pub fn construct(&mut self) {
        for object in &mut self.objects {
            if let Some(script) = object.script.take() {
                script.construct(object);
                object.script = Some(script);
            }
        }
    }

    pub fn start(&mut self) {
        for object in &mut self.objects {
            if let Some(mut script) = object.script.take() {
                script.start(object);
                object.script = Some(script);
            }
        }
    }

    pub fn update(&mut self, dt: f32) {
        for object in &mut self.objects {
            if let Some(mut script) = object.script.take() {
                script.update(object, dt);
                object.script = Some(script);
            }
        }
    }

    pub fn render(&self, renderer_queue: &mut RenderQueue) {
        for object in &self.objects {
            if let Some(sprite) = object.get_component::<SpriteRenderer>() {
                if let Some(transform) = object.get_component::<Transform>() {
                    // Получаем текстуру (временно — заглушка)
                    renderer_queue.draw_sprite(
                        sprite.get_texture_handle(),
                        transform.position,
                        transform.rotation,
                        transform.scale,
                    );
                }
            }
        }
    }
}
