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

    pub fn with(mut self, layer: impl Layer + 'static) -> Self {
        self.layers.push(Arc::new(layer));
        self
    }

    pub fn push(&mut self, layer: impl Layer + 'static) {
        self.layers.push(Arc::new(layer));
    }

    pub fn run(&mut self) {
        let mut world = if let Some(last) = self.history.last_mut() {
            last.next()
        } else {
            World::default()
        };

        for layer in &self.layers {
            layer.run(&mut world);
        }
    }
}
