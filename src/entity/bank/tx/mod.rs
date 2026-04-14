mod deposit;
mod transfer;
mod withdrawl;

pub use deposit::*;
pub use transfer::*;
pub use withdrawl::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TxId(u64);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Tx {
    Deposit(Deposit),
    Withdrawl(Withdrawl),
    Transfer(Transfer),
}

impl Tx {
    pub fn id(&self) -> TxId {
        match self {
            Self::Deposit(v) => v.id,
            Self::Withdrawl(v) => v.id,
            Self::Transfer(v) => v.id,
        }
    }

    pub fn account_id(&self) -> super::account::AccountId {
        match self {
            Self::Deposit(v) => v.account_id,
            Self::Withdrawl(v) => v.account_id,
            Self::Transfer(v) => v.account_id,
        }
    }

    pub fn ammount(&self) -> u64 {
        match self {
            Self::Deposit(v) => v.ammount,
            Self::Withdrawl(v) => v.ammount,
            Self::Transfer(v) => v.ammount,
        }
    }

    pub fn created_at(&self) -> &chrono::DateTime<chrono::Utc> {
        match self {
            Self::Deposit(v) => &v.created_at,
            Self::Withdrawl(v) => &v.created_at,
            Self::Transfer(v) => &v.created_at,
        }
    }

    pub fn as_deposit(&self) -> Option<&Deposit> {
        match self {
            Self::Deposit(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_withdrawl(&self) -> Option<&Withdrawl> {
        match self {
            Self::Withdrawl(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_transfer(&self) -> Option<&Transfer> {
        match self {
            Self::Transfer(v) => Some(v),
            _ => None,
        }
    }
}

impl From<Deposit> for Tx {
    fn from(value: Deposit) -> Self {
        Self::Deposit(value)
    }
}

impl From<Withdrawl> for Tx {
    fn from(value: Withdrawl) -> Self {
        Self::Withdrawl(value)
    }
}

impl From<Transfer> for Tx {
    fn from(value: Transfer) -> Self {
        Self::Transfer(value)
    }
}

impl std::cmp::Ord for Tx {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Equal)
    }
}

impl std::cmp::PartialOrd for Tx {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.created_at().partial_cmp(other.created_at())
    }
}
