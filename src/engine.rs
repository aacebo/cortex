use std::sync::{Arc, mpsc};

use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Message {
    Action(Action),
    Command(Command),
}

impl From<Action> for Message {
    fn from(value: Action) -> Self {
        Self::Action(value)
    }
}

impl From<Command> for Message {
    fn from(value: Command) -> Self {
        Self::Command(value)
    }
}

pub struct EngineBuilder {
    tick: Tick,
    world: World,
    layers: Vec<Arc<dyn Layer>>,
    observers: Vec<Arc<dyn Observer>>,
    clock: Box<dyn Clock>,
    rate: Option<TickRate>,
}

impl EngineBuilder {
    pub(crate) fn new() -> Self {
        Self {
            tick: Tick::default(),
            world: World::default(),
            layers: vec![],
            observers: vec![],
            clock: Box::new(SystemClock::new()),
            rate: None,
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

    pub fn clock(mut self, clock: impl Clock + 'static) -> Self {
        self.clock = Box::new(clock);
        self
    }

    pub fn rate(mut self, rate: TickRate) -> Self {
        self.rate = Some(rate);
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
            clock: self.clock,
            rate: self.rate,
            tick_started_at: None,
        }
    }
}

pub struct Engine {
    tick: Tick,
    world: World,
    clock: Box<dyn Clock>,
    rate: Option<TickRate>,
    tick_started_at: Option<chrono::DateTime<chrono::Utc>>,
    layers: Vec<Arc<dyn Layer>>,
    observers: Vec<Arc<dyn Observer>>,
    producer: mpsc::Sender<Message>,
    consumer: mpsc::Receiver<Message>,
}

impl Engine {
    pub fn tick(&self) -> Tick {
        self.tick
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn snapshot(&self) -> Snapshot<World> {
        let ts = self.tick_started_at.unwrap_or_else(|| self.clock.now());

        Snapshot {
            tick: self.tick,
            data: self.world.clone(),
            started_at: ts,
            ended_at: self.clock.now(),
        }
    }

    pub fn run(&mut self) -> Shutdown {
        loop {
            match self.next() {
                None => self.clock.wait_until_next_tick(self.rate),
                Some(cmd) => match cmd {
                    Command::Shutdown(v) => return Shutdown::Requested(v),
                },
            };
        }
    }

    pub fn next(&mut self) -> Option<Command> {
        self.tick = self.tick.next();
        self.tick_started_at = Some(self.clock.now());

        let mut ctx = Context::new(&mut self.world, self.producer.clone());

        for layer in &self.layers {
            layer.on_tick(&mut ctx);
        }

        while let Ok(message) = self.consumer.try_recv() {
            match message {
                Message::Command(cmd) => return Some(cmd),
                Message::Action(action) => {
                    for observer in &self.observers {
                        observer.on_action(&mut ctx, &action);
                    }
                }
            };
        }

        None
    }
}
