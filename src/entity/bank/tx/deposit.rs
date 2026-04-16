use super::super::account;
use super::TxId;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DepositStyle {
    /// Automated transfers, often used for paychecks or government benefits.
    Direct,

    /// Adding funds via checks (scanned by phone) or cash at a physical machine.
    Mobile,
}

impl DepositStyle {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Direct => "direct",
            Self::Mobile => "mobile",
        }
    }
}

impl std::fmt::Display for DepositStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Activities that increase your account balance.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Deposit {
    pub id: TxId,
    pub account_id: account::AccountId,
    pub style: DepositStyle,
    pub amount: u64,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
