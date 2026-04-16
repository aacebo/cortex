use crate::CurrencyId;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Money {
    cid: CurrencyId,
    units: i64, // ex. cents
}

impl Money {
    pub fn new(cid: CurrencyId, units: i64) -> Self {
        Self { cid, units }
    }
}

impl std::ops::Add for Money {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        assert_eq!(self.cid, rhs.cid);
        self.units += rhs.units;
        self
    }
}

impl std::ops::Sub for Money {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        assert_eq!(self.cid, rhs.cid);
        self.units -= rhs.units;
        self
    }
}
