pub mod account;
pub mod tx;

use std::collections::BTreeMap;

use crate::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BankId(&'static str);

impl BankId {
    pub fn as_str(&self) -> &str {
        self.0
    }
}

impl std::fmt::Display for BankId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum BankType {
    Central,
    Commercial,
    Investment,
    Development,
}

impl BankType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Central => "central",
            Self::Commercial => "commercial",
            Self::Investment => "investment",
            Self::Development => "development",
        }
    }
}

impl std::fmt::Display for BankType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// A trusted storage service
/// for a given currency.
#[derive(Debug, Clone)]
pub struct Bank {
    /// ex. jpm
    pub id: BankId,

    /// ex. J.P. Morgan Chase
    pub name: String,

    /// the type of bank
    pub ty: BankType,

    /// reserves by currency in minor units
    pub reserves: BTreeMap<CurrencyId, i64>,

    /// the banks accounts.
    pub accounts: BTreeMap<account::AccountId, account::Account>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BankAction {
    Create(CreateBankAction),
    Delete(DeleteBankAction),
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
