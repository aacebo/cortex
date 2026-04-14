use std::collections::BTreeSet;

use crate::entity;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AccountId(u64);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum AccountType {
    Checking,
    Savings,
}

impl AccountType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Checking => "checking",
            Self::Savings => "savings",
        }
    }
}

impl std::fmt::Display for AccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone)]
pub struct Account {
    pub id: AccountId,
    pub ty: AccountType,
    pub balance: entity::Money,
    pub ledger: BTreeSet<entity::tx::Tx>,
    pub opened_at: chrono::Utc,
    pub closed_at: Option<chrono::Utc>,
}
