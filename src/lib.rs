mod engine;
pub mod entity;
mod tick;
mod world;

pub use engine::*;
pub use tick::*;
pub use world::*;

pub fn new() -> Engine {
    Engine::new()
}

pub trait Layer {
    fn run(&mut self, world: &mut World);
}
