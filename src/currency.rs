use crate::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CurrencyId(&'static str);

impl CurrencyId {
    pub fn as_str(&self) -> &str {
        self.0
    }
}

impl std::fmt::Display for CurrencyId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CurrencyType {
    Fiat,
    Commodity,
    Crypto,
}

impl CurrencyType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Fiat => "fiat",
            Self::Commodity => "commodity",
            Self::Crypto => "crypto",
        }
    }
}

impl std::fmt::Display for CurrencyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ExchangeRateStyle {
    Floating,
    Pegged(CurrencyId),
    ManagedFloat,
    CapitalControlled,
}

impl ExchangeRateStyle {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Floating => "floating",
            Self::Pegged(_) => "pegged",
            Self::ManagedFloat => "managed-float",
            Self::CapitalControlled => "capital-controlled",
        }
    }
}

impl std::fmt::Display for ExchangeRateStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Floating => "floating".to_string(),
                Self::Pegged(v) => format!("pegged@{}", v),
                Self::ManagedFloat => "managed-float".to_string(),
                Self::CapitalControlled => "capital-controlled".to_string(),
            }
        )
    }
}

/// A representation of value that can
/// be exchanged or traded for goods/services.
#[derive(Debug, Clone)]
pub struct Currency {
    /// ex. usd
    pub id: CurrencyId,

    /// ex. usa
    pub issuer_id: CountryId,

    /// ex. Dollar
    pub name: String,

    /// the type
    pub ty: CurrencyType,

    /// minor units scaling
    /// 1 major unit = 10^N minor units
    /// where N = scale.
    pub scale: u32,

    /// how the exchange rate of this currency is determined.
    pub exchange_rate_style: ExchangeRateStyle,
}

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
