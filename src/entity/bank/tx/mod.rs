mod deposit;
mod transfer;
mod withdrawal;

pub use deposit::*;
pub use transfer::*;
pub use withdrawal::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TxId(u64);

impl std::fmt::Display for TxId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Tx {
    Deposit(Deposit),
    Withdrawal(Withdrawal),
    Transfer(Transfer),
}

impl Tx {
    pub fn id(&self) -> TxId {
        match self {
            Self::Deposit(v) => v.id,
            Self::Withdrawal(v) => v.id,
            Self::Transfer(v) => v.id,
        }
    }

    pub fn account_id(&self) -> super::account::AccountId {
        match self {
            Self::Deposit(v) => v.account_id,
            Self::Withdrawal(v) => v.account_id,
            Self::Transfer(v) => v.sender_id,
        }
    }

    pub fn amount(&self) -> u64 {
        match self {
            Self::Deposit(v) => v.amount,
            Self::Withdrawal(v) => v.amount,
            Self::Transfer(v) => v.amount,
        }
    }

    pub fn created_at(&self) -> &chrono::DateTime<chrono::Utc> {
        match self {
            Self::Deposit(v) => &v.created_at,
            Self::Withdrawal(v) => &v.created_at,
            Self::Transfer(v) => &v.created_at,
        }
    }

    pub fn as_deposit(&self) -> Option<&Deposit> {
        match self {
            Self::Deposit(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_withdrawal(&self) -> Option<&Withdrawal> {
        match self {
            Self::Withdrawal(v) => Some(v),
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

impl From<Withdrawal> for Tx {
    fn from(value: Withdrawal) -> Self {
        Self::Withdrawal(value)
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
