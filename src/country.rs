use std::{
    collections::{BTreeMap, BTreeSet},
    sync::mpsc,
};

use crate::*;

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
    pub name: String,

    /// the banks of the country.
    pub banks: BTreeSet<BankId>,

    /// the countries official currency.
    pub currencies: BTreeSet<CurrencyId>,

    /// when the country was first created.
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CountryAction {
    Create(CreateCountryAction),
    Delete(DeleteCountryAction),
}

impl From<CreateCountryAction> for CountryAction {
    fn from(value: CreateCountryAction) -> Self {
        Self::Create(value)
    }
}

impl From<DeleteCountryAction> for CountryAction {
    fn from(value: DeleteCountryAction) -> Self {
        Self::Delete(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreateCountryAction {
    pub id: CountryId,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DeleteCountryAction {
    pub id: CountryId,
}

pub struct Countries<'a> {
    store: &'a mut BTreeMap<CountryId, Country>,
    producer: mpsc::Sender<Action>,
}

impl<'a> Countries<'a> {
    pub(crate) fn new(
        store: &'a mut BTreeMap<CountryId, Country>,
        producer: mpsc::Sender<Action>,
    ) -> Self {
        Self { store, producer }
    }

    pub fn get(&self, id: &CountryId) -> Option<&Country> {
        self.store.get(id)
    }

    pub fn get_mut(&mut self, id: &CountryId) -> Option<&mut Country> {
        self.store.get_mut(id)
    }

    pub fn dispatch(&mut self, action: CountryAction) -> Result<(), mpsc::SendError<Action>> {
        self.producer.send(action.into())
    }
}
