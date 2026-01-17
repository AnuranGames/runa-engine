use crate::ocs::object::Object;

/// Impl to script object
pub trait Script: 'static {
    fn construct(&self, _object: &mut Object) {}
    fn start(&mut self, _object: &mut Object) {}
    fn update(&mut self, _object: &mut Object, _dt: f32) {}
}
