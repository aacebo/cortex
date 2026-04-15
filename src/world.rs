use std::collections::BTreeMap;

use crate::*;

#[derive(Debug, Default, Clone)]
pub struct World {
    pub banks: BTreeMap<BankId, Bank>,
    pub countries: BTreeMap<CountryId, Country>,
    pub currencies: BTreeMap<CurrencyId, Currency>,
    pub resources: BTreeMap<ResourceId, Resource>,
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }
}
