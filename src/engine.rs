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
        let mut binding = self.history.last().cloned();
        let mut world = binding.get_or_insert_default();

        for mut layer in &mut self.layers {
            if let Some(v) = Arc::get_mut(&mut layer) {
                v.run(&mut world);
            }
        }
    }
}
