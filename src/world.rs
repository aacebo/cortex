use std::collections::BTreeMap;

use crate::{Tick, entity};

#[derive(Debug, Default, Clone)]
pub struct World {
    tick: Tick,
    banks: BTreeMap<entity::BankId, entity::Bank>,
    countries: BTreeMap<entity::CountryId, entity::Country>,
    currencies: BTreeMap<entity::CurrencyId, entity::Currency>,
    resources: BTreeMap<entity::ResourceId, entity::Resource>,
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tick(&self) -> &Tick {
        &self.tick
    }

    pub fn next(&mut self) -> &mut Self {
        self.tick = self.tick.next();
        self
    }

    pub fn bank(&self, id: &entity::BankId) -> Option<&entity::Bank> {
        self.banks.get(id)
    }

    pub fn bank_mut(&mut self, id: &entity::BankId) -> Option<&mut entity::Bank> {
        self.banks.get_mut(id)
    }

    pub fn country(&self, id: &entity::CountryId) -> Option<&entity::Country> {
        self.countries.get(id)
    }

    pub fn country_mut(&mut self, id: &entity::CountryId) -> Option<&mut entity::Country> {
        self.countries.get_mut(id)
    }

    pub fn currency(&self, id: &entity::CurrencyId) -> Option<&entity::Currency> {
        self.currencies.get(id)
    }

    pub fn currency_mut(&mut self, id: &entity::CurrencyId) -> Option<&mut entity::Currency> {
        self.currencies.get_mut(id)
    }

    pub fn resource(&self, id: &entity::ResourceId) -> Option<&entity::Resource> {
        self.resources.get(id)
    }

    pub fn resource_mut(&mut self, id: &entity::ResourceId) -> Option<&mut entity::Resource> {
        self.resources.get_mut(id)
    }
}
