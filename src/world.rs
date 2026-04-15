use std::collections::BTreeMap;

use crate::*;

#[derive(Debug, Default, Clone)]
pub struct World {
    pub(crate) banks: BTreeMap<BankId, Bank>,
    pub(crate) countries: BTreeMap<CountryId, Country>,
    pub(crate) currencies: BTreeMap<CurrencyId, Currency>,
    pub(crate) resources: BTreeMap<ResourceId, Resource>,
}
