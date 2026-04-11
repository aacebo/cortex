use crate::{Bank, Currency, Exchange};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CountryId(&'static str);

#[derive(Debug, Clone)]
pub struct Country {
    /// ex. USA
    pub id: CountryId,

    /// ex. United States of America
    pub name: &'static str,

    /// the countries official currency.
    pub currency: Currency,

    /// the exchange rates accepted
    /// by the country for external
    /// currencies.
    pub exchange: Exchange,

    /// the banks of the country.
    pub banks: Vec<Bank>,
}
