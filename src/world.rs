use std::collections::BTreeMap;

use crate::*;

#[derive(Debug, Default, Clone)]
pub struct World {
    pub tick: Tick,
    pub banks: BTreeMap<BankId, Bank>,
    pub countries: BTreeMap<CountryId, Country>,
    pub currencies: BTreeMap<CurrencyId, Currency>,
    pub resources: BTreeMap<ResourceId, Resource>,
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn next(&self) -> Self {
        let mut next = self.clone();
        next.tick = self.tick.next();
        next
    }

    pub fn bank(&self, id: &BankId) -> Option<&Bank> {
        self.banks.get(id)
    }

    pub fn bank_mut(&mut self, id: &BankId) -> Option<&mut Bank> {
        self.banks.get_mut(id)
    }

    pub fn country(&self, id: &CountryId) -> Option<&Country> {
        self.countries.get(id)
    }

    pub fn country_mut(&mut self, id: &CountryId) -> Option<&mut Country> {
        self.countries.get_mut(id)
    }

    pub fn currency(&self, id: &CurrencyId) -> Option<&Currency> {
        self.currencies.get(id)
    }

    pub fn currency_mut(&mut self, id: &CurrencyId) -> Option<&mut Currency> {
        self.currencies.get_mut(id)
    }

    pub fn resource(&self, id: &ResourceId) -> Option<&Resource> {
        self.resources.get(id)
    }

    pub fn resource_mut(&mut self, id: &ResourceId) -> Option<&mut Resource> {
        self.resources.get_mut(id)
    }
}
