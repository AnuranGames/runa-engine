#[derive(Clone, Copy, Debug, Default)]
pub struct Sorting {
    pub order: i32,
}

impl Sorting {
    pub fn new(order: i32) -> Self {
        Self { order }
    }
}
