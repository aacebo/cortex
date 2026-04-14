use std::sync::Arc;

use crate::{Layer, World};

#[derive(Clone)]
pub struct Engine {
    layers: Vec<Arc<dyn Layer>>,
    history: Vec<World>,
}

impl Engine {
    pub(crate) fn new() -> Self {
        Self {
            layers: vec![],
            history: vec![],
        }
    }

    pub fn with(mut self, effect: impl Layer + 'static) -> Self {
        self.layers.push(Arc::new(effect));
        self
    }

    pub fn push(&mut self, effect: impl Layer + 'static) {
        self.layers.push(Arc::new(effect));
    }

    pub fn run(&mut self) {
        let mut world = if let Some(last) = self.history.last_mut() {
            last.next()
        } else {
            World::default()
        };

        for layer in &self.layers {
            layer.tick(&mut world);
        }

        self.history.push(world);
    }
}
