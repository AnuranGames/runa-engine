use crate::ocs::{Object, ObjectId, World};

pub enum WorldCommand {
    Despawn(ObjectId),
    Spawn(Object),
}

pub struct ScriptCommands {
    world_ptr: *mut World,
}

impl ScriptCommands {
    pub(crate) fn new(world_ptr: *mut World) -> Self {
        Self { world_ptr }
    }

    pub fn despawn(&mut self, object_id: ObjectId) {
        unsafe {
            if let Some(world) = self.world_ptr.as_mut() {
                world.queue_command(WorldCommand::Despawn(object_id));
            }
        }
    }

    pub fn spawn(&mut self, object: Object) {
        unsafe {
            if let Some(world) = self.world_ptr.as_mut() {
                world.queue_command(WorldCommand::Spawn(object));
            }
        }
    }
}
