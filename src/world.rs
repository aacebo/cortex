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
}
