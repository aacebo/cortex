#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CurrencyId(&'static str);

/// A representation of value that can
/// be exchanged or traded for goods/services.
#[derive(Debug, Clone, PartialEq)]
pub struct Currency {
    /// ex. usd
    pub id: CurrencyId,

    /// ex. Dollar
    pub name: &'static str,

    /// currency value according to
    /// owning country.
    pub value: f32,
}
