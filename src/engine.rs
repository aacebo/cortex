use std::sync::{Arc, mpsc};

use crate::{Action, Context, Effect, Layer, Snapshot, Tick, World};

pub struct EngineBuilder {
    tick: Tick,
    world: World,
    layers: Vec<Arc<dyn Layer>>,
    effects: Vec<Arc<dyn Effect>>,
}

impl EngineBuilder {
    pub(crate) fn new() -> Self {
        Self {
            tick: Tick::default(),
            world: World::default(),
            layers: vec![],
            effects: vec![],
        }
    }

    pub fn restore(mut self, snapshot: Snapshot<World>) -> Self {
        self.tick = snapshot.tick;
        self.world = snapshot.data;
        self
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
            tick: self.tick,
            layers: self.layers,
            effects: self.effects,
            world: self.world,
            producer,
            consumer,
        }
    }
}

pub struct Engine {
    tick: Tick,
    layers: Vec<Arc<dyn Layer>>,
    effects: Vec<Arc<dyn Effect>>,
    world: World,
    producer: mpsc::Sender<Action>,
    consumer: mpsc::Receiver<Action>,
}

impl Engine {
    pub fn next(&mut self) -> Snapshot<World> {
        self.tick = self.tick.next();
        let started_at = chrono::Utc::now();
        let mut ctx = Context::new(&mut self.world, self.producer.clone());

        for layer in &self.layers {
            layer.tick(&mut ctx);

            while let Ok(action) = self.consumer.try_recv() {
                for effect in &self.effects {
                    effect.on_action(&mut ctx, &action);
                }

                if let Action::Shutdown(a) = action {
                    std::process::exit(a.code());
                }
            }
        }

        Snapshot {
            tick: self.tick,
            data: self.world.clone(),
            started_at,
            ended_at: chrono::Utc::now(),
        }
    }
}
