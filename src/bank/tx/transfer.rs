use crate::bank::account;
use crate::bank::tx;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TransferStyle {
    /// Moving money between your own accounts at the same bank (e.g., from checking to savings).
    Internal,

    /// Moving money between different financial institutions.
    External,
}

impl TransferStyle {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Internal => "internal",
            Self::External => "external",
        }
    }
}

impl std::fmt::Display for TransferStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Activities that transfer money between accounts.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Transfer {
    pub id: tx::TxId,
    pub account_id: account::AccountId,
    pub recipient_id: account::AccountId,
    pub style: TransferStyle,
    pub amount: u64,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
