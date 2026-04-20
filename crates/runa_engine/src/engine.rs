use std::sync::Arc;

use runa_core::{
    components::{
        ActiveCamera, AudioListener, AudioSource, Camera, Canvas, Collider2D,
        CursorInteractable, MeshRenderer, ObjectDefinitionInstance, PhysicsCollision,
        SpriteRenderer, Tilemap, TilemapRenderer, Transform, Component,
    },
    ocs::{Object, Script, World},
    registry::{
        ArchetypeKey, ArchetypeMetadata, RunaArchetype, RunaComponentType, RunaScriptType,
        RuntimeRegistry, TypeMetadata,
    },
};

pub trait RunaTypeRegistration {
    fn register(engine: &mut Engine) -> TypeMetadata;
}

pub struct Engine {
    runtime_registry: RuntimeRegistry,
}

impl Engine {
    pub fn new() -> Self {
        let mut engine = Self {
            runtime_registry: RuntimeRegistry::new(),
        };
        engine.register_builtin_types();
        engine
    }

    pub fn runtime_registry(&self) -> &RuntimeRegistry {
        &self.runtime_registry
    }

    pub fn runtime_registry_mut(&mut self) -> &mut RuntimeRegistry {
        &mut self.runtime_registry
    }

    pub fn register_component<T: Component + 'static>(&mut self) -> TypeMetadata {
        self.runtime_registry.register_component::<T>()
    }

    pub fn register_component_named<T: Component + 'static>(
        &mut self,
        type_name: &'static str,
    ) -> TypeMetadata {
        self.runtime_registry
            .register_component_named::<T>(type_name)
    }

    pub fn register_script<T: Script + 'static>(&mut self) -> TypeMetadata {
        self.runtime_registry.register_script::<T>()
    }

    pub fn register<T: RunaTypeRegistration>(&mut self) -> TypeMetadata {
        T::register(self)
    }

    pub fn register_script_named<T: Script + 'static>(
        &mut self,
        type_name: &'static str,
    ) -> TypeMetadata {
        self.runtime_registry.register_script_named::<T>(type_name)
    }

    pub fn register_derived_component<T: RunaComponentType>(&mut self) -> TypeMetadata {
        self.register_component_named::<T>(T::runa_component_type_name())
    }

    pub fn register_derived_script<T: RunaScriptType>(&mut self) -> TypeMetadata {
        self.register_script_named::<T>(T::runa_script_type_name())
    }

    pub fn register_archetype<T>(&mut self) -> ArchetypeMetadata
    where
        T: RunaArchetype,
    {
        self.runtime_registry.register_archetype::<T>()
    }

    pub fn register_archetype_named<F>(
        &mut self,
        name: impl Into<Arc<str>>,
        factory: F,
    ) -> ArchetypeMetadata
    where
        F: Fn() -> Object + Send + Sync + 'static,
    {
        self.runtime_registry.register_archetype_named(name, factory)
    }

    pub fn create_world(&self) -> World {
        let mut world = World::default();
        world.set_runtime_registry(Arc::new(self.runtime_registry.clone()));
        world
    }

    pub fn spawn_archetype<T: RunaArchetype>(&self, world: &mut World) -> u64 {
        T::create(world)
    }

    pub fn spawn_archetype_by_key(&self, world: &mut World, key: &ArchetypeKey) -> Option<u64> {
        self.runtime_registry.spawn_archetype_by_key(world, key)
    }

    pub fn spawn_archetype_by_name(&self, world: &mut World, name: &str) -> Option<u64> {
        self.runtime_registry.spawn_archetype_by_name(world, name)
    }

    fn register_builtin_types(&mut self) {
        self.runtime_registry.register_builtin_component::<Transform>();
        self.runtime_registry.register_builtin_component::<Camera>();
        self.runtime_registry.register_builtin_component::<ActiveCamera>();
        self.runtime_registry.register_builtin_component::<SpriteRenderer>();
        self.runtime_registry.register_builtin_component::<Collider2D>();
        self.runtime_registry.register_builtin_component::<Canvas>();
        self.runtime_registry.register_builtin_component::<AudioListener>();
        self.runtime_registry.register_builtin_component::<AudioSource>();
        self.runtime_registry.register_builtin_component::<CursorInteractable>();
        self.runtime_registry.register_builtin_component::<MeshRenderer>();
        self.runtime_registry.register_builtin_component::<ObjectDefinitionInstance>();
        self.runtime_registry.register_builtin_component::<PhysicsCollision>();
        self.runtime_registry.register_builtin_component::<Tilemap>();
        self.runtime_registry.register_builtin_component::<TilemapRenderer>();
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}
