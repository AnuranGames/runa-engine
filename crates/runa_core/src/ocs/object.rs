use crate::components::{Collider2D, Component, Transform};
use crate::ocs::{ScriptContext, World};
use glam::Vec2;
use std::any::TypeId;
use std::collections::HashMap;

pub type ObjectId = u64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ObjectHandle {
    id: ObjectId,
}

impl ObjectHandle {
    pub fn new(id: ObjectId) -> Self {
        Self { id }
    }

    pub fn id(self) -> ObjectId {
        self.id
    }
}

pub struct Object {
    id: Option<ObjectId>,
    pub name: String,
    components: HashMap<TypeId, Box<dyn Component>>,
    world_ptr: *mut World,
}

impl Object {
    pub fn new(name: impl Into<String>) -> Self {
        let mut components: HashMap<TypeId, Box<dyn Component>> = HashMap::new();
        components.insert(TypeId::of::<Transform>(), Box::new(Transform::default()));

        Self {
            id: None,
            name: name.into(),
            components,
            world_ptr: std::ptr::null_mut(),
        }
    }

    pub fn empty() -> Self {
        Self::new("")
    }

    pub fn id(&self) -> Option<ObjectId> {
        self.id
    }

    pub fn handle(&self) -> Option<ObjectHandle> {
        self.id.map(ObjectHandle::new)
    }

    pub(crate) fn set_world(&mut self, world: &mut World) {
        self.world_ptr = world as *mut World;
    }

    pub(crate) fn get_world_ptr(&mut self) -> *mut World {
        self.world_ptr
    }

    pub(crate) fn set_id(&mut self, id: ObjectId) {
        self.id = Some(id);
    }

    pub fn with<T: Component>(mut self, part: T) -> Self {
        self.add_component(part);
        self
    }

    pub(crate) fn get_world(&self) -> Option<&World> {
        if self.world_ptr.is_null() {
            None
        } else {
            Some(unsafe { &*self.world_ptr })
        }
    }

    /// Add a component to the object. Only one component of a given type is allowed.
    pub fn add_component<T: Component>(&mut self, component: T) -> &mut Object {
        let type_id = TypeId::of::<T>();
        if type_id == TypeId::of::<Transform>() {
            self.components.insert(type_id, Box::new(component));
            return self;
        }

        assert!(
            !self.components.contains_key(&type_id),
            "Component already exists {type_id:?}"
        );
        self.components.insert(type_id, Box::new(component));
        self
    }

    pub fn get_component<T: 'static>(&self) -> Option<&T> {
        self.components
            .get(&TypeId::of::<T>())
            .and_then(|c| c.as_any().downcast_ref())
    }

    pub fn get_component_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.components
            .get_mut(&TypeId::of::<T>())
            .and_then(|c| c.as_any_mut().downcast_mut())
    }

    pub fn has_component<T: 'static>(&self) -> bool {
        self.get_component::<T>().is_some()
    }

    pub(crate) fn run_start(&mut self) {
        let component_ids: Vec<TypeId> = self.components.keys().copied().collect();
        for type_id in component_ids {
            let Some(mut component) = self.components.remove(&type_id) else {
                continue;
            };
            let mut ctx = ScriptContext::new(self);
            component.on_start(&mut ctx);
            self.components.insert(type_id, component);
        }
    }

    pub(crate) fn run_update(&mut self, dt: f32) {
        let component_ids: Vec<TypeId> = self.components.keys().copied().collect();
        for type_id in component_ids {
            let Some(mut component) = self.components.remove(&type_id) else {
                continue;
            };
            let mut ctx = ScriptContext::new(self);
            component.on_update(&mut ctx, dt);
            self.components.insert(type_id, component);
        }
    }

    pub fn is_colliding_2d(&mut self) -> bool {
        let center = self
            .get_component::<Transform>()
            .map(|transform| transform.position.truncate())
            .unwrap_or(Vec2::ZERO);
        self.would_collide_2d_at(center)
    }

    pub fn would_collide_2d_at(&mut self, center: Vec2) -> bool {
        let Some(collider) = self.get_component::<Collider2D>().copied() else {
            return false;
        };

        let self_ptr = self as *const Object;
        self.get_world()
            .map(|world| world.overlaps_collider_2d(center, &collider, Some(self_ptr)))
            .unwrap_or(false)
    }
}
