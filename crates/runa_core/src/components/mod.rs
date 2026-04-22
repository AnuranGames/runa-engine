pub mod active_camera;
pub mod audio_listener;
pub mod audio_source;
mod camera;
mod collider2d;
mod component;
mod cursor_interactable;
mod mesh_renderer;
mod object_definition_instance;
mod physics_collision;
mod serialized_type_storage;
mod sprite_renderer;
mod tilemap;
mod transform;
pub mod ui;

pub use active_camera::ActiveCamera;
pub use audio_listener::AudioListener;
pub use audio_source::AudioSource;
pub use camera::Camera;
pub use camera::ProjectionType;
pub use collider2d::Collider2D;
pub use component::{
    Component, ComponentRuntimeKind, SerializedField, SerializedFieldAccess, SerializedFieldValue,
};
pub use cursor_interactable::CursorInteractable;
pub use mesh_renderer::BuiltinMeshPrimitive;
pub use mesh_renderer::Mesh;
pub use mesh_renderer::MeshRenderer;
pub use object_definition_instance::ObjectDefinitionInstance;
pub use physics_collision::PhysicsCollision;
pub use serialized_type_storage::{SerializedTypeEntry, SerializedTypeKind, SerializedTypeStorage};
pub use sprite_renderer::{SpriteRenderer, DEFAULT_SPRITE_PIXELS_PER_UNIT};
pub use tilemap::Rect;
pub use tilemap::Tile;
pub use tilemap::Tilemap;
pub use tilemap::TilemapLayer;
pub use tilemap::TilemapRenderer;
pub use transform::Transform;

pub use ui::Canvas;

macro_rules! impl_component {
    ($($ty:ty),+ $(,)?) => {
        $(
        impl SerializedFieldAccess for $ty {}

        impl Component for $ty {
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }

            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                self
            }
        })+
    };
}

impl_component!(
    ActiveCamera,
    AudioListener,
    AudioSource,
    Camera,
    Collider2D,
    CursorInteractable,
    MeshRenderer,
    ObjectDefinitionInstance,
    PhysicsCollision,
    SerializedTypeStorage,
    SpriteRenderer,
    Tilemap,
    TilemapRenderer,
    Transform,
    Canvas,
);
