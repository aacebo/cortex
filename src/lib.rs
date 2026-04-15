mod action;
pub mod bank;
mod context;
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
pub use context::*;
pub use country::{Country, CountryId};
pub use currency::{Currency, CurrencyId};
pub use engine::*;
pub use entity::Entity;
pub use money::Money;
pub use resource::{Resource, ResourceId};
pub use tick::*;
pub use world::*;

pub fn new() -> EngineBuilder {
    EngineBuilder::new()
}

pub trait Layer: Send + Sync {
    fn tick(&self, ctx: &mut Context);
}

/// ## Effect
/// When implemented a type can listen
/// to world/engine events and optionally
/// make changes to the world based on said events.
pub trait Effect: Send + Sync {
    #![allow(unused_variables)]

    fn on_shutdown(&self, ctx: &mut Context, action: &ShutdownAction) {}
    fn on_action(&self, ctx: &mut Context, action: &Action) {
        match action {
            Action::Shutdown(a) => self.on_shutdown(ctx, a),
            Action::Bank(a) => self.on_bank_action(ctx, a),
            Action::Country(a) => self.on_country_action(ctx, a),
            Action::Currency(a) => self.on_currency_action(ctx, a),
        }
    }

    fn on_bank_create(&self, ctx: &mut Context, action: &bank::CreateBankAction) {}
    fn on_bank_delete(&self, ctx: &mut Context, action: &bank::DeleteBankAction) {}
    fn on_bank_action(&self, ctx: &mut Context, action: &bank::BankAction) {
        match action {
            bank::BankAction::Create(a) => self.on_bank_create(ctx, a),
            bank::BankAction::Delete(a) => self.on_bank_delete(ctx, a),
        }
    }

    fn on_country_create(&self, ctx: &mut Context, action: &country::CreateCountryAction) {}
    fn on_country_delete(&self, ctx: &mut Context, action: &country::DeleteCountryAction) {}
    fn on_country_action(&self, ctx: &mut Context, action: &country::CountryAction) {
        match action {
            country::CountryAction::Create(a) => self.on_country_create(ctx, a),
            country::CountryAction::Delete(a) => self.on_country_delete(ctx, a),
        }
    }

    fn on_currency_create(&self, ctx: &mut Context, action: &currency::CreateCurrencyAction) {}
    fn on_currency_delete(&self, ctx: &mut Context, action: &currency::DeleteCurrencyAction) {}
    fn on_currency_action(&self, ctx: &mut Context, action: &currency::CurrencyAction) {
        match action {
            currency::CurrencyAction::Create(a) => self.on_currency_create(ctx, a),
            currency::CurrencyAction::Delete(a) => self.on_currency_delete(ctx, a),
        }
    }
}
