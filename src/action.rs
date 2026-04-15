use crate::{bank, country, currency};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Action {
    Shutdown(ShutdownAction),
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ShutdownAction {
    /// The command completed without errors.
    Success,

    /// Catchall for miscellaneous errors (e.g., divide by zero).
    InternalError(String),

    /// Missing arguments or keyword/syntax errors.
    InputError(String),
}

impl ShutdownAction {
    pub fn code(&self) -> i32 {
        match self {
            Self::Success => 0,
            Self::InternalError(_) => 1,
            Self::InputError(_) => 2,
        }
    }

    pub fn message(&self) -> Option<&str> {
        match self {
            Self::Success => None,
            Self::InternalError(m) => Some(m.as_str()),
            Self::InputError(m) => Some(m.as_str()),
        }
    }
}

impl std::fmt::Display for ShutdownAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.code())?;

        if let Some(m) = self.message() {
            write!(f, " - {}", m)?;
        }

        Ok(())
    }
}
