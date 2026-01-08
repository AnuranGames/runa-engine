use std::task::Context;

pub trait Component {
    fn start(&mut self, _ctx: &mut Context) {}
    fn update(&mut self, _ctx: &mut Context, _dt: f32) {}
}
