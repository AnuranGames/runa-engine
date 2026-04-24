use crate::components::{Collider2D, Component, ComponentRuntimeKind, SerializedFieldAccess};
use crate::ocs::{Object, ObjectHandle, ObjectId, ScriptCommands, World};
use glam::Vec2;

pub struct ScriptContext<'a> {
    object: &'a mut Object,
}

impl<'a> ScriptContext<'a> {
    pub(crate) fn new(object: &'a mut Object) -> Self {
        Self { object }
    }

    pub fn id(&self) -> Option<ObjectId> {
        self.object.id()
    }

    pub fn name(&self) -> &str {
        &self.object.name
    }

    pub fn set_name(&mut self, name: impl Into<String>) {
        self.object.name = name.into();
    }

    pub fn get_component<T: 'static>(&self) -> Option<&T> {
        self.object.get_component::<T>()
    }

    pub fn get_component_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.object.get_component_mut::<T>()
    }

    pub fn add_component<T: Component>(&mut self, component: T) -> &mut Self {
        self.object.add_component(component);
        self
    }

    pub fn handle(&self) -> Option<ObjectHandle> {
        self.object.handle()
    }

    pub fn object(&self) -> &Object {
        self.object
    }

    pub fn object_mut(&mut self) -> &mut Object {
        self.object
    }

    pub fn world(&self) -> &World {
        self.object
            .get_world()
            .expect("ScriptContext world access is only valid for world-owned objects")
    }

    pub fn commands(&mut self) -> ScriptCommands {
        ScriptCommands::new(self.object.get_world_ptr())
    }

    pub fn get_object(&self, id: ObjectId) -> Option<&Object> {
        self.world().get(id)
    }

    pub fn find_first_with<T: 'static>(&self) -> Option<ObjectId> {
        self.world().find_first_with::<T>()
    }

    pub fn find_all_with<T: 'static>(&self) -> Vec<ObjectId> {
        self.world().find_all_with::<T>()
    }

    pub fn is_colliding_2d(&mut self) -> bool {
        self.object.is_colliding_2d()
    }

    pub fn would_collide_2d_at(&mut self, center: Vec2) -> bool {
        self.object.would_collide_2d_at(center)
    }

    pub fn overlaps_collider_2d(&self, center: Vec2, collider: &Collider2D) -> bool {
        let self_ptr = self.object as *const Object;
        self.world()
            .overlaps_collider_2d(center, collider, Some(self_ptr))
    }
}

/// Script component that adds custom behavior to an object.
///
/// Scripts are attachable behavior components.
/// They follow a deterministic lifecycle and operate on an already-composed object.
///
/// # Lifecycle
/// 1. `start()` - Called once after the object enters the world
/// 2. `update()` - Called every tick while object exists in the world
/// 3. `late_update()` - Called after all regular updates for the tick
///
/// # Example
/// ```rust,ignore
/// struct Player {
///     speed: f32,
/// }
///
/// impl Script for Player {
///     fn start(&mut self, ctx: &mut ScriptContext) {
///         // Access components after object is in world
///         println!("Player spawned at {:?}", ctx.get_component::<Transform>().unwrap().position);
///     }
///
///     fn update(&mut self, ctx: &mut ScriptContext, dt: f32) {
///         // Game logic runs every tick
///         if Input::is_key_pressed(KeyCode::W) {
///             let transform = ctx.get_component_mut::<Transform>().unwrap();
///             transform.position.y -= self.speed * dt;
///         }
///
///         // Audio playback via AudioSource::play()
///         if let Some(audio) = ctx.get_component_mut::<AudioSource>() {
///             audio.play();
///         }
///     }
/// }
///
/// // Queue world mutations instead of mutating the world directly.
/// // if let Some(id) = ctx.id() {
/// //     ctx.commands().despawn(id);
/// // }
/// ```
pub trait Script: SerializedFieldAccess + 'static {
    /// Called once on the first tick after the object is added to the world.
    ///
    /// Use this method to:
    /// - Access other objects in the world
    /// - Query world state (e.g., find nearest enemy)
    /// - Start coroutines or timed events
    /// - Initialize physics/collision state
    ///
    /// This is the earliest point where the object is fully integrated into the simulation.
    fn start(&mut self, _ctx: &mut ScriptContext) {}

    /// Called every tick while the object exists in the world.
    ///
    /// Use this method for:
    /// - Input handling (`Input::is_key_pressed()`)
    /// - Movement and animation
    /// - AI behavior and decision making
    /// - Physics updates (use fixed timestep for determinism)
    /// - Audio playback via `AudioSource::play()`
    ///
    /// Parameters:
    /// - `dt`: Delta time in seconds since last frame (use for frame-rate independent movement)
    fn update(&mut self, _ctx: &mut ScriptContext, _dt: f32) {}

    /// Called after all regular `update()` calls for the current tick.
    ///
    /// Use this for dependent logic that must observe the final results of gameplay updates,
    /// such as follow cameras and post-movement alignment.
    fn late_update(&mut self, _ctx: &mut ScriptContext, _dt: f32) {}
}

impl<T: Script> Component for T {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn runtime_kind(&self) -> ComponentRuntimeKind {
        ComponentRuntimeKind::Script
    }

    fn runtime_type_name(&self) -> &'static str {
        std::any::type_name::<T>()
    }

    fn on_start(&mut self, ctx: &mut ScriptContext) {
        Script::start(self, ctx);
    }

    fn on_update(&mut self, ctx: &mut ScriptContext, dt: f32) {
        Script::update(self, ctx, dt);
    }

    fn on_late_update(&mut self, ctx: &mut ScriptContext, dt: f32) {
        Script::late_update(self, ctx, dt);
    }
}
