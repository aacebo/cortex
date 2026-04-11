#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BankId(&'static str);

/// A trusted storage service
/// for a given currency.
#[derive(Debug, Clone)]
pub struct Bank {
    /// ex. jpm
    pub id: BankId,

    /// ex. J.P. Morgan Chase
    pub name: &'static str,
}
