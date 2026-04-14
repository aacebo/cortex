use crate::entity;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum WithdrawalStyle {
    /// Using a debit or credit card for in-person or online shopping.
    PointOfSale,

    /// Taking out physical cash from your account.
    Mobile,
}

impl WithdrawalStyle {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::PointOfSale => "point-of-sale",
            Self::Mobile => "mobile",
        }
    }
}

impl std::fmt::Display for WithdrawalStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Activities that decrease your account balance.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Withdrawal {
    pub id: entity::tx::TxId,
    pub account_id: entity::account::AccountId,
    pub style: WithdrawalStyle,
    pub amount: u64,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
