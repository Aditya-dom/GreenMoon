use specs::prelude::*;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Camera;

impl Camera {
    pub fn new() -> Self {
        Self { }
    }
}