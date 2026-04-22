use std::sync::Arc;

use super::command::WorldCommand;
use glam::{Vec2, Vec3};
use runa_render_api::RenderQueue;

use crate::{
    audio::{AudioEngine, SoundId},
    components::{
        ActiveCamera, AudioListener, AudioSource, Camera, Canvas, Collider2D, MeshRenderer,
        SpriteRenderer, Tilemap, Transform,
    },
    debug_renderer::DebugRenderer,
    ocs::{Object, ObjectId, Script},
    registry::{ArchetypeKey, RunaArchetype, RuntimeRegistry},
};

pub struct World {
    objects: Vec<Object>,
    debug_renderer: DebugRenderer,
    pub audio_engine: AudioEngine,
    next_object_id: u64,
    command_queue: Vec<WorldCommand>,
    processing_lifecycle: bool,
    started: bool,
    runtime_registry: Option<Arc<RuntimeRegistry>>,
}

impl World {
    /// Play a sound through the audio engine
    pub fn play_sound(&mut self, audio_source: &AudioSource) -> Option<SoundId> {
        self.audio_engine.play(audio_source)
    }

    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            debug_renderer: DebugRenderer::new(),
            audio_engine: AudioEngine::default(),
            next_object_id: 1,
            command_queue: Vec::new(),
            processing_lifecycle: false,
            started: false,
            runtime_registry: None,
        }
    }

    pub fn spawn(&mut self, object: Object) -> ObjectId {
        self.insert_object(object)
    }

    pub fn spawn_script<S: Script>(&mut self, script: S) -> ObjectId {
        self.insert_object(Object::empty().with(script))
    }

    pub fn set_runtime_registry(&mut self, runtime_registry: Arc<RuntimeRegistry>) {
        self.runtime_registry = Some(runtime_registry);
    }

    pub fn runtime_registry(&self) -> Option<&RuntimeRegistry> {
        self.runtime_registry.as_deref()
    }

    pub fn spawn_archetype<T: RunaArchetype>(&mut self) -> ObjectId {
        T::create(self)
    }

    pub fn spawn_archetype_by_key(&mut self, key: &ArchetypeKey) -> Option<ObjectId> {
        let registry = self.runtime_registry.clone()?;
        registry.spawn_archetype_by_key(self, key)
    }

    pub fn spawn_archetype_by_name(&mut self, name: &str) -> Option<ObjectId> {
        let registry = self.runtime_registry.clone()?;
        registry.spawn_archetype_by_name(self, name)
    }

    fn insert_object(&mut self, mut object: Object) -> ObjectId {
        let id = self.next_object_id;
        self.next_object_id += 1;
        object.set_id(id);

        self.objects.push(object);

        let world_ptr = self as *mut World;
        let object = self.objects.last_mut().unwrap();
        object.set_world(unsafe { &mut *world_ptr });
        if self.started && !self.processing_lifecycle {
            Self::start_object_lifecycle(object);
        }
        id
    }

    pub fn construct(&mut self) {
        self.audio_engine
            .initialize()
            .expect("Failed to initialize audio engine");

        let world_ptr = self as *mut World;
        for object in &mut self.objects {
            object.set_world(unsafe { &mut *world_ptr });
        }
    }

    pub fn start(&mut self) {
        self.processing_lifecycle = true;
        let world_ptr = self as *mut World;
        for object in &mut self.objects {
            object.set_world(unsafe { &mut *world_ptr });
            Self::start_object_lifecycle(object);
        }
        self.processing_lifecycle = false;
        self.started = true;
        self.apply_commands();
    }

    pub fn update(&mut self, dt: f32) {
        for object in &mut self.objects {
            if let Some(transform) = object.get_component_mut::<Transform>() {
                transform.prepare_for_update();
            }
        }

        // Update scripts
        self.processing_lifecycle = true;
        for object in &mut self.objects {
            object.run_update(dt);
        }
        for object in &mut self.objects {
            object.run_late_update(dt);
        }
        self.processing_lifecycle = false;
        self.apply_commands();

        // Find active AudioListener and update listener position
        let mut listener_found = false;
        for object in &self.objects {
            if let (Some(listener), Some(transform)) = (
                object.get_component::<AudioListener>(),
                object.get_component::<Transform>(),
            ) {
                if listener.active {
                    self.audio_engine.set_listener(
                        transform.position,
                        transform.rotation,
                        listener.volume,
                    );
                    self.audio_engine
                        .set_stereo_separation(listener.stereo_separation);
                    listener_found = true;
                    break;
                }
            }
        }

        // If no active listener found, use default position
        if !listener_found {
            self.audio_engine
                .set_listener(Vec3::ZERO, glam::Quat::IDENTITY, 1.0);
        }

        // Update spatial sound volumes based on listener position
        self.audio_engine.update_spatial_volumes();

        // Process audio requests (play/stop) from AudioSource components
        for object in &mut self.objects {
            // Get sound position for 3D audio first (to avoid borrow conflicts)
            let sound_position = object.get_component::<Transform>().map(|t| t.position);

            if let Some(audio) = object.get_component_mut::<AudioSource>() {
                // Handle stop requests first
                if audio.stop_requested {
                    if let Some(sound_id) = audio.sound_id.take() {
                        self.audio_engine.stop(sound_id);
                    }
                    audio.playing = false;
                    audio.stop_requested = false;
                }

                // Handle play requests
                if audio.play_requested && audio.audio_asset.is_some() {
                    // Stop previous sound if still playing
                    if let Some(sound_id) = audio.sound_id.take() {
                        self.audio_engine.stop(sound_id);
                    }

                    // Play new sound with spatial positioning if needed
                    let sound_id = self.audio_engine.play_spatial(audio, sound_position);

                    if let Some(id) = sound_id {
                        audio.sound_id = Some(id);
                        audio.playing = true;
                    }
                    audio.play_requested = false;
                }
            }
        }

        // Cleanup finished sounds
        self.audio_engine.cleanup();

        // UI
        for object in &mut self.objects {
            let viewport_size = if let (Some(camera), Some(_active)) = (
                object.get_component::<Camera>(),
                object.get_component::<ActiveCamera>(),
            ) {
                Some(glam::Vec2::new(
                    camera.viewport_size.0 as f32,
                    camera.viewport_size.1 as f32,
                ))
            } else {
                None
            };

            if let (Some(canvas), Some(viewport_size)) =
                (object.get_component_mut::<Canvas>(), viewport_size)
            {
                if canvas.dirty_layout {
                    canvas.layout(viewport_size);
                }
            }
        }
    }

    pub fn render(&self, render_queue: &mut RenderQueue, interpolation_factor: f32) {
        for object in &self.objects {
            // 3D Mesh rendering
            if let (Some(transform), Some(mesh_renderer)) = (
                object.get_component::<Transform>(),
                object.get_component::<MeshRenderer>(),
            ) {
                // Convert Mesh vertices to render_api Vertex3D
                let vertices: Vec<runa_render_api::command::Vertex3D> = mesh_renderer
                    .mesh
                    .vertices
                    .iter()
                    .map(|v| runa_render_api::command::Vertex3D {
                        position: v.position,
                        normal: v.normal,
                        uv: v.uv,
                    })
                    .collect();

                // Create model matrix
                let model_matrix = glam::Mat4::from_scale_rotation_translation(
                    transform.scale,
                    transform.interpolated_rotation(interpolation_factor),
                    transform.interpolated_position(interpolation_factor),
                );

                render_queue.draw_mesh_3d(
                    vertices,
                    mesh_renderer.mesh.indices.clone(),
                    model_matrix,
                    mesh_renderer.color,
                );
            }

            // 2D Sprite rendering
            if let (Some(transform), Some(sprite)) = (
                object.get_component::<Transform>(),
                object.get_component::<SpriteRenderer>(),
            ) {
                let Some(texture) = sprite.texture.clone() else {
                    continue;
                };

                render_queue.draw_sprite(
                    Arc::from(texture),
                    transform.interpolated_position(interpolation_factor),
                    transform.interpolated_rotation(interpolation_factor),
                    transform.scale,
                );
            }

            if let (Some(tilemap), Some(transform), Some(_renderer)) = (
                object.get_component::<Tilemap>(),
                object.get_component::<Transform>(),
                object.get_component::<crate::components::TilemapRenderer>(),
            ) {
                for layer in &tilemap.layers {
                    if !layer.visible {
                        continue;
                    }

                    for y in tilemap.offset.y..(tilemap.offset.y + tilemap.height as i32) {
                        for x in tilemap.offset.x..(tilemap.offset.x + tilemap.width as i32) {
                            let array_x = (x - tilemap.offset.x) as u32;
                            let array_y = (y - tilemap.offset.y) as u32;

                            if let Some(tile) = layer.get_tile(array_x as u32, array_y as u32) {
                                if tile.texture.is_none() {
                                    continue;
                                }

                                // Tile world position relative to the object
                                let world_pos = tilemap.tile_to_world(x, y);
                                let final_pos =
                                    transform.interpolated_position(interpolation_factor) + world_pos;

                                render_queue.draw_tile(
                                    tile.texture.clone().unwrap(),
                                    final_pos,
                                    tilemap.tile_size,
                                    [
                                        tile.uv_rect.x,
                                        tile.uv_rect.y,
                                        tile.uv_rect.width,
                                        tile.uv_rect.height,
                                    ],
                                    tile.flip_x,
                                    tile.flip_y,
                                    [1.0, 1.0, 1.0, 1.0],
                                );
                            }
                        }
                    }
                }
            }

            if let (Some(canvas), Some(_camera), Some(_ac)) = (
                &mut object.get_component::<Canvas>(),
                object.get_component::<Camera>(),
                object.get_component::<ActiveCamera>(),
            ) {
                canvas.build_render_commands(render_queue);
            }
        }

        // Debug rendering
        self.debug_renderer.render_debug(self, render_queue);
    }

    pub fn set_debug_draw_collisions(&mut self, enabled: bool) {
        self.debug_renderer.set_debug_draw_collisions(enabled);
    }

    pub fn is_debug_draw_collisions_enabled(&self) -> bool {
        self.debug_renderer.is_debug_draw_collisions_enabled()
    }

    /// Remove an object from the world.
    ///
    /// If a lifecycle pass is active, removal is deferred until `apply_commands()`.
    pub fn despawn(&mut self, id: ObjectId) -> bool {
        if self.processing_lifecycle {
            self.queue_command(WorldCommand::Despawn(id));
            return true;
        }

        self.despawn_immediate(id).is_some()
    }

    pub fn get(&self, id: ObjectId) -> Option<&Object> {
        self.objects.iter().find(|object| object.id() == Some(id))
    }

    pub fn get_mut(&mut self, id: ObjectId) -> Option<&mut Object> {
        self.objects
            .iter_mut()
            .find(|object| object.id() == Some(id))
    }

    pub fn find_first_with<T: 'static>(&self) -> Option<ObjectId> {
        self.objects
            .iter()
            .find(|object| object.get_component::<T>().is_some())
            .and_then(|object| object.id())
    }

    /// Return all matching object ids.
    ///
    /// Order is intentionally not part of the public contract.
    pub fn find_all_with<T: 'static>(&self) -> Vec<ObjectId> {
        self.objects
            .iter()
            .filter(|object| object.get_component::<T>().is_some())
            .filter_map(|object| object.id())
            .collect()
    }

    /// Query object ids by component type.
    ///
    /// Order is intentionally not part of the public contract.
    pub fn query<T: 'static>(&self) -> Vec<ObjectId> {
        self.find_all_with::<T>()
    }

    pub(crate) fn queue_command(&mut self, command: WorldCommand) {
        self.command_queue.push(command);
    }

    pub fn apply_commands(&mut self) {
        while !self.command_queue.is_empty() {
            let commands = std::mem::take(&mut self.command_queue);
            for command in commands {
                match command {
                    WorldCommand::Despawn(object_id) => {
                        self.despawn_immediate(object_id);
                    }
                    WorldCommand::Spawn(object) => {
                        self.insert_object(object);
                    }
                }
            }
        }
    }

    pub fn overlaps_collider_2d(
        &self,
        center: Vec2,
        collider: &Collider2D,
        ignore: Option<*const Object>,
    ) -> bool {
        self.objects.iter().any(|object| {
            if ignore.is_some_and(|ignored| std::ptr::eq(object as *const Object, ignored)) {
                return false;
            }

            let Some(other_transform) = object.get_component::<Transform>() else {
                return false;
            };
            let Some(other_collider) = object.get_component::<Collider2D>() else {
                return false;
            };

            collider.intersects(center, other_collider, other_transform.position.truncate())
        })
    }

    fn despawn_immediate(&mut self, id: ObjectId) -> Option<Object> {
        let index = self
            .objects
            .iter()
            .position(|object| object.id() == Some(id))?;
        Some(self.objects.remove(index))
    }

    fn start_object_lifecycle(object: &mut Object) {
        object.run_start();

        if let Some(audio) = object.get_component_mut::<AudioSource>() {
            if audio.play_on_awake && audio.audio_asset.is_some() {
                audio.play_requested = true;
            }
        }
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::World;
    use crate::{
        components::{SerializedFieldAccess, Transform},
        ocs::{Object, Script, ScriptContext},
    };

    struct DespawnSelf;
    impl SerializedFieldAccess for DespawnSelf {}

    impl Script for DespawnSelf {
        fn update(&mut self, ctx: &mut ScriptContext, _dt: f32) {
            if let Some(id) = ctx.id() {
                ctx.commands().despawn(id);
            }
        }
    }

    #[test]
    fn deferred_despawn_applies_after_update_phase() {
        let mut world = World::default();
        world.spawn(Object::new("Transient").with(DespawnSelf));
        world.start();

        assert_eq!(world.query::<Transform>().len(), 1);
        world.update(1.0 / 60.0);
        assert!(world.query::<Transform>().is_empty());
    }
}
