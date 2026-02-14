use crate::ocs::Object;

/// Impl to scripting object
pub trait Script: 'static {
    /// s
    fn construct(&self, _object: &mut Object) {}
    fn start(&mut self, _object: &mut Object) {}
    fn update(&mut self, _object: &mut Object, _dt: f32) {}
}
