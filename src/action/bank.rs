use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BankAction {
    Create(CreateBankAction),
    Delete(DeleteBankAction),
}

impl BankAction {
    pub fn to_action(self) -> Action {
        self.into()
    }

    pub fn to_message(self) -> Message {
        self.to_action().into()
    }
}

impl From<CreateBankAction> for BankAction {
    fn from(value: CreateBankAction) -> Self {
        Self::Create(value)
    }
}

impl From<DeleteBankAction> for BankAction {
    fn from(value: DeleteBankAction) -> Self {
        Self::Delete(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreateBankAction {
    pub id: BankId,
    pub name: String,
    pub ty: BankType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DeleteBankAction {
    pub id: BankId,
}
