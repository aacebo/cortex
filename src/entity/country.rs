use std::collections::BTreeSet;

use crate::entity;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CountryId(&'static str);

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
