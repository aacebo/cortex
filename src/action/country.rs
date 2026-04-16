use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CountryAction {
    Create(CreateCountryAction),
    Delete(DeleteCountryAction),
}

impl CountryAction {
    pub fn to_action(self) -> Action {
        self.into()
    }

    pub fn to_message(self) -> Message {
        self.to_action().into()
    }
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
