use crate::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EntityId {
    Bank(BankId),
    Country(CountryId),
    Currency(CurrencyId),
    Resource(ResourceId),
}

impl EntityId {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Bank(v) => v.as_str(),
            Self::Country(v) => v.as_str(),
            Self::Currency(v) => v.as_str(),
            Self::Resource(v) => v.as_str(),
        }
    }
}

impl From<BankId> for EntityId {
    fn from(value: BankId) -> Self {
        Self::Bank(value)
    }
}

impl From<CountryId> for EntityId {
    fn from(value: CountryId) -> Self {
        Self::Country(value)
    }
}

impl From<CurrencyId> for EntityId {
    fn from(value: CurrencyId) -> Self {
        Self::Currency(value)
    }
}

impl From<ResourceId> for EntityId {
    fn from(value: ResourceId) -> Self {
        Self::Resource(value)
    }
}

impl std::fmt::Display for EntityId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone)]
pub enum Entity {
    Bank(Bank),
    Country(Country),
    Currency(Currency),
    Resource(Resource),
}

impl Entity {
    pub fn id(&self) -> EntityId {
        match self {
            Self::Bank(v) => v.id.into(),
            Self::Country(v) => v.id.into(),
            Self::Currency(v) => v.id.into(),
            Self::Resource(v) => v.id.into(),
        }
    }
}
