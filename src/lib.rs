mod context;
mod engine;
pub mod entity;
mod event;
mod tick;
mod world;

pub use context::*;
pub use engine::*;
pub use event::*;
pub use tick::*;
pub use world::*;

pub fn new() -> Engine {
    Engine::new()
}

pub trait Layer {
    fn tick(&self, world: &mut World);
}

// pub trait Store {
//     fn append(&mut self, events: impl IntoIterator<Item = Event<>>);
// }

/// ## Effect
/// When implemented a type can listen
/// to world/engine events and optionally
/// make changes to the world based on said events.
pub trait Effect {
    #![allow(unused_variables)]

    fn on_change(&self, world: &mut World, event: Event<entity::Entity>) {
        match event.body {
            entity::Entity::Resource(v) => self.on_resource_change(world, event.with_body(v)),
            entity::Entity::Country(v) => self.on_country_change(world, event.with_body(v)),
            entity::Entity::Currency(v) => self.on_currency_change(world, event.with_body(v)),
            entity::Entity::Bank(v) => self.on_bank_change(world, event.with_body(v)),
        }
    }

    fn on_resource_create(&self, world: &mut World, event: Event<entity::Resource>) {}
    fn on_resource_update(&self, world: &mut World, event: Event<entity::Resource>) {}
    fn on_resource_delete(&self, world: &mut World, event: Event<entity::Resource>) {}
    fn on_resource_change(&self, world: &mut World, event: Event<entity::Resource>) {
        match event.action {
            Action::Create => self.on_resource_create(world, event),
            Action::Update => self.on_resource_update(world, event),
            Action::Delete => self.on_resource_delete(world, event),
        }
    }

    fn on_country_create(&self, world: &mut World, event: Event<entity::Country>) {}
    fn on_country_update(&self, world: &mut World, event: Event<entity::Country>) {}
    fn on_country_delete(&self, world: &mut World, event: Event<entity::Country>) {}
    fn on_country_change(&self, world: &mut World, event: Event<entity::Country>) {
        match event.action {
            Action::Create => self.on_country_create(world, event),
            Action::Update => self.on_country_update(world, event),
            Action::Delete => self.on_country_delete(world, event),
        }
    }

    fn on_currency_create(&self, world: &mut World, event: Event<entity::Currency>) {}
    fn on_currency_update(&self, world: &mut World, event: Event<entity::Currency>) {}
    fn on_currency_delete(&self, world: &mut World, event: Event<entity::Currency>) {}
    fn on_currency_change(&self, world: &mut World, event: Event<entity::Currency>) {
        match event.action {
            Action::Create => self.on_currency_create(world, event),
            Action::Update => self.on_currency_update(world, event),
            Action::Delete => self.on_currency_delete(world, event),
        }
    }

    fn on_bank_create(&self, world: &mut World, event: Event<entity::Bank>) {}
    fn on_bank_update(&self, world: &mut World, event: Event<entity::Bank>) {}
    fn on_bank_delete(&self, world: &mut World, event: Event<entity::Bank>) {}
    fn on_bank_change(&self, world: &mut World, event: Event<entity::Bank>) {
        match event.action {
            Action::Create => self.on_bank_create(world, event),
            Action::Update => self.on_bank_update(world, event),
            Action::Delete => self.on_bank_delete(world, event),
        }
    }
}
