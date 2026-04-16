pub mod action;
mod clock;
pub mod command;
mod context;
mod engine;
pub mod entity;
pub mod error;
mod money;
mod snapshot;
mod tick;
mod world;

pub use clock::*;
pub use context::*;
pub use engine::*;
pub use money::*;
pub use snapshot::*;
pub use tick::*;
pub use world::*;

pub mod prelude {
    pub use super::action::*;
    pub use super::command::*;
    pub use super::entity::*;
    pub use super::*;
}

pub fn new() -> EngineBuilder {
    EngineBuilder::new()
}

/// A system layer that gets executed with every tick
/// and typically transforms world state.
pub trait Layer: Send + Sync {
    fn on_tick(&self, ctx: &mut Context);
}

/// When implemented a type can listen
/// to world/engine events and optionally
/// make changes to the world based on said events.
pub trait Observer: Send + Sync {
    #![allow(unused_variables)]

    fn on_action(&self, ctx: &mut Context, action: &action::Action) {
        match action {
            action::Action::Bank(a) => self.on_bank_action(ctx, a),
            action::Action::Country(a) => self.on_country_action(ctx, a),
            action::Action::Currency(a) => self.on_currency_action(ctx, a),
        }
    }

    fn on_bank_create(&self, ctx: &mut Context, action: &action::CreateBankAction) {}
    fn on_bank_delete(&self, ctx: &mut Context, action: &action::DeleteBankAction) {}
    fn on_bank_action(&self, ctx: &mut Context, action: &action::BankAction) {
        match action {
            action::BankAction::Create(a) => self.on_bank_create(ctx, a),
            action::BankAction::Delete(a) => self.on_bank_delete(ctx, a),
        }
    }

    fn on_country_create(&self, ctx: &mut Context, action: &action::CreateCountryAction) {}
    fn on_country_delete(&self, ctx: &mut Context, action: &action::DeleteCountryAction) {}
    fn on_country_action(&self, ctx: &mut Context, action: &action::CountryAction) {
        match action {
            action::CountryAction::Create(a) => self.on_country_create(ctx, a),
            action::CountryAction::Delete(a) => self.on_country_delete(ctx, a),
        }
    }

    fn on_currency_create(&self, ctx: &mut Context, action: &action::CreateCurrencyAction) {}
    fn on_currency_delete(&self, ctx: &mut Context, action: &action::DeleteCurrencyAction) {}
    fn on_currency_action(&self, ctx: &mut Context, action: &action::CurrencyAction) {
        match action {
            action::CurrencyAction::Create(a) => self.on_currency_create(ctx, a),
            action::CurrencyAction::Delete(a) => self.on_currency_delete(ctx, a),
        }
    }
}
