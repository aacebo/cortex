use std::sync::{Arc, mpsc};

use crate::{Action, Context, Layer, Observer, Snapshot, Tick, World};

pub struct EngineBuilder {
    tick: Tick,
    world: World,
    layers: Vec<Arc<dyn Layer>>,
    observers: Vec<Arc<dyn Observer>>,
}

impl EngineBuilder {
    pub(crate) fn new() -> Self {
        Self {
            tick: Tick::default(),
            world: World::default(),
            layers: vec![],
            observers: vec![],
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

    pub fn observer(mut self, observer: impl Observer + 'static) -> Self {
        self.observers.push(Arc::new(observer));
        self
    }

    pub fn build(self) -> Engine {
        let (producer, consumer) = mpsc::channel();

        Engine {
            tick: self.tick,
            layers: self.layers,
            observers: self.observers,
            world: self.world,
            producer,
            consumer,
        }
    }
}

pub struct Engine {
    tick: Tick,
    world: World,
    layers: Vec<Arc<dyn Layer>>,
    observers: Vec<Arc<dyn Observer>>,
    producer: mpsc::Sender<Action>,
    consumer: mpsc::Receiver<Action>,
}

impl Engine {
    pub fn start(&mut self) {
        loop {
            self.next();
        }
    }

    pub fn next(&mut self) -> Snapshot<World> {
        self.tick = self.tick.next();
        let started_at = chrono::Utc::now();
        let mut ctx = Context::new(&mut self.world, self.producer.clone());

        for layer in &self.layers {
            layer.on_tick(&mut ctx);

            while let Ok(action) = self.consumer.try_recv() {
                for observer in &self.observers {
                    observer.on_action(&mut ctx, &action);
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
