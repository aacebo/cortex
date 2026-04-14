use crate::entity;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum WithdrawlStyle {
    /// Using a debit or credit card for in-person or online shopping.
    PointOfSale,

    /// Taking out physical cash from your account.
    Mobile,
}

impl WithdrawlStyle {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::PointOfSale => "point-of-sale",
            Self::Mobile => "mobile",
        }
    }
}

impl std::fmt::Display for WithdrawlStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Activities that increase your account balance.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Withdrawl {
    pub id: entity::tx::TxId,
    pub account_id: entity::account::AccountId,
    pub style: WithdrawlStyle,
    pub ammount: u64,
    pub created_at: chrono::Utc,
}
