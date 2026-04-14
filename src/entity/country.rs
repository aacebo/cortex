use std::collections::BTreeSet;

use crate::entity;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CountryId(&'static str);

impl CountryId {
    pub fn as_str(&self) -> &str {
        self.0
    }
}

impl std::fmt::Display for CountryId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone)]
pub struct Country {
    /// ex. USA
    pub id: CountryId,

    /// ex. United States of America
    pub name: &'static str,

    /// the banks of the country.
    pub banks: BTreeSet<entity::BankId>,

    /// the countries official currency.
    pub currencies: BTreeSet<entity::CurrencyId>,
}
