use crate::{Tick, bank, country, currency};

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

#[derive(Debug, Clone)]
pub struct ActionRecord {
    pub id: ulid::Ulid,
    pub tick: Tick,
    pub action: Action,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl ActionRecord {
    pub(crate) fn new(tick: Tick, action: Action) -> Self {
        Self {
            id: ulid::Ulid::new(),
            tick,
            action,
            created_at: chrono::Utc::now(),
        }
    }
}
