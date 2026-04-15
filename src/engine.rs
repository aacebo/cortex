use std::sync::{Arc, mpsc};

use crate::{Action, Context, Effect, Layer, World};

pub struct EngineBuilder {
    layers: Vec<Arc<dyn Layer>>,
    effects: Vec<Arc<dyn Effect>>,
}

impl EngineBuilder {
    pub(crate) fn new() -> Self {
        Self {
            layers: vec![],
            effects: vec![],
        }
    }

    pub fn layer(mut self, layer: impl Layer + 'static) -> Self {
        self.layers.push(Arc::new(layer));
        self
    }

    pub fn effect(mut self, effect: impl Effect + 'static) -> Self {
        self.effects.push(Arc::new(effect));
        self
    }

    pub fn build(self) -> Engine {
        let (producer, consumer) = mpsc::channel();

        Engine {
            layers: self.layers,
            effects: self.effects,
            history: vec![],
            producer,
            consumer,
        }
    }
}

pub struct Engine {
    layers: Vec<Arc<dyn Layer>>,
    effects: Vec<Arc<dyn Effect>>,
    history: Vec<World>,
    producer: mpsc::Sender<Action>,
    consumer: mpsc::Receiver<Action>,
}

impl Engine {
    pub fn run(&mut self) {
        let mut world = if let Some(last) = self.history.last_mut() {
            last.next()
        } else {
            World::default()
        };

        let mut context = Context::new(&mut world, self.producer.clone());

        for layer in &self.layers {
            layer.tick(&mut context);
        }

        self.history.push(world);
    }
}
