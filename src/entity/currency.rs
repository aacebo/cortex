#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CurrencyId(&'static str);

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

/// A representation of value that can
/// be exchanged or traded for goods/services.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Currency {
    /// ex. usd
    pub id: CurrencyId,

    /// ex. Dollar
    pub name: &'static str,

    /// the type
    pub ty: CurrencyType,

    /// minor units scaling
    /// 1 major unit = 10^N minor units
    /// where N = scale.
    pub scale: u32,
}
