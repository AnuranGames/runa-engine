use crate::ocs::ScriptContext;
use std::any::Any;

pub trait Component: Any {
    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn on_start(&mut self, _ctx: &mut ScriptContext) {}

    fn on_update(&mut self, _ctx: &mut ScriptContext, _dt: f32) {}
}
