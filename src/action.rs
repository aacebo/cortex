use crate::{bank, country, currency};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Action {
    Bank(bank::BankAction),
    Country(country::CountryAction),
    Currency(currency::CurrencyAction),
}

impl From<bank::BankAction> for Action {
    fn from(value: bank::BankAction) -> Self {
        Self::Bank(value)
    }
}

impl From<country::CountryAction> for Action {
    fn from(value: country::CountryAction) -> Self {
        Self::Country(value)
    }
}

impl From<currency::CurrencyAction> for Action {
    fn from(value: currency::CurrencyAction) -> Self {
        Self::Currency(value)
    }
}
