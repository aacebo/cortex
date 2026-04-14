mod action;
pub mod bank;
pub mod country;
pub mod currency;
mod engine;
mod entity;
mod money;
pub mod resource;
mod tick;
mod world;

pub use action::*;
pub use bank::{Bank, BankId};
pub use country::{Country, CountryId};
pub use currency::{Currency, CurrencyId};
pub use engine::*;
pub use entity::Entity;
pub use money::Money;
pub use resource::{Resource, ResourceId};
pub use tick::*;
pub use world::*;

pub fn new() -> Engine {
    Engine::new()
}

pub trait Layer {
    fn tick(&self, world: &mut World);
}

/// ## Effect
/// When implemented a type can listen
/// to world/engine events and optionally
/// make changes to the world based on said events.
pub trait Effect {
    #![allow(unused_variables)]

    fn on_action(&self, world: &mut World, action: &Action) {
        match action {
            Action::Bank(a) => self.on_bank_action(world, a),
            Action::Country(a) => self.on_country_action(world, a),
            Action::Currency(a) => self.on_currency_action(world, a),
        }
    }

    fn on_bank_create(&self, world: &mut World, action: &bank::CreateBankAction) {}
    fn on_bank_delete(&self, world: &mut World, action: &bank::DeleteBankAction) {}
    fn on_bank_action(&self, world: &mut World, action: &bank::BankAction) {
        match action {
            bank::BankAction::Create(a) => self.on_bank_create(world, a),
            bank::BankAction::Delete(a) => self.on_bank_delete(world, a),
        }
    }

    fn on_country_create(&self, world: &mut World, action: &country::CreateCountryAction) {}
    fn on_country_delete(&self, world: &mut World, action: &country::DeleteCountryAction) {}
    fn on_country_action(&self, world: &mut World, action: &country::CountryAction) {
        match action {
            country::CountryAction::Create(a) => self.on_country_create(world, a),
            country::CountryAction::Delete(a) => self.on_country_delete(world, a),
        }
    }

    fn on_currency_create(&self, world: &mut World, action: &currency::CreateCurrencyAction) {}
    fn on_currency_delete(&self, world: &mut World, action: &currency::DeleteCurrencyAction) {}
    fn on_currency_action(&self, world: &mut World, action: &currency::CurrencyAction) {
        match action {
            currency::CurrencyAction::Create(a) => self.on_currency_create(world, a),
            currency::CurrencyAction::Delete(a) => self.on_currency_delete(world, a),
        }
    }
}
