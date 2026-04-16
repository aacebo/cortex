mod action;
mod clock;
mod command;
mod context;
mod engine;
mod entity;
pub mod error;
mod money;
mod snapshot;
mod tick;
mod world;

pub use action::*;
pub use clock::*;
pub use command::*;
pub use context::*;
pub use engine::*;
pub use entity::*;
pub use money::*;
pub use snapshot::*;
pub use tick::*;
pub use world::*;

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

    fn on_action(&self, ctx: &mut Context, action: &Action) {
        match action {
            Action::Bank(a) => self.on_bank_action(ctx, a),
            Action::Country(a) => self.on_country_action(ctx, a),
            Action::Currency(a) => self.on_currency_action(ctx, a),
        }
    }

    fn on_bank_create(&self, ctx: &mut Context, action: &CreateBankAction) {}
    fn on_bank_delete(&self, ctx: &mut Context, action: &DeleteBankAction) {}
    fn on_bank_action(&self, ctx: &mut Context, action: &BankAction) {
        match action {
            BankAction::Create(a) => self.on_bank_create(ctx, a),
            BankAction::Delete(a) => self.on_bank_delete(ctx, a),
        }
    }

    fn on_country_create(&self, ctx: &mut Context, action: &CreateCountryAction) {}
    fn on_country_delete(&self, ctx: &mut Context, action: &DeleteCountryAction) {}
    fn on_country_action(&self, ctx: &mut Context, action: &CountryAction) {
        match action {
            CountryAction::Create(a) => self.on_country_create(ctx, a),
            CountryAction::Delete(a) => self.on_country_delete(ctx, a),
        }
    }

    fn on_currency_create(&self, ctx: &mut Context, action: &CreateCurrencyAction) {}
    fn on_currency_delete(&self, ctx: &mut Context, action: &DeleteCurrencyAction) {}
    fn on_currency_action(&self, ctx: &mut Context, action: &CurrencyAction) {
        match action {
            CurrencyAction::Create(a) => self.on_currency_create(ctx, a),
            CurrencyAction::Delete(a) => self.on_currency_delete(ctx, a),
        }
    }
}
