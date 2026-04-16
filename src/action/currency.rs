use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CurrencyAction {
    Create(CreateCurrencyAction),
    Delete(DeleteCurrencyAction),
}

impl From<CreateCurrencyAction> for CurrencyAction {
    fn from(value: CreateCurrencyAction) -> Self {
        Self::Create(value)
    }
}

impl From<DeleteCurrencyAction> for CurrencyAction {
    fn from(value: DeleteCurrencyAction) -> Self {
        Self::Delete(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CreateCurrencyAction {
    pub id: CurrencyId,
    pub issuer_id: CountryId,
    pub name: String,
    pub ty: CurrencyType,
    pub scale: u32,
    pub exchange_rate_style: ExchangeRateStyle,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DeleteCurrencyAction {
    pub id: CurrencyId,
}
