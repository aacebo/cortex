mod bank;
mod country;
mod currency;
mod money;
mod resource;

pub use bank::*;
pub use country::*;
pub use currency::*;
pub use money::*;
pub use resource::*;

#[derive(Debug, Clone)]
pub enum Entity {
    Bank(Bank),
    Country(Country),
    Currency(Currency),
    Resource(Resource),
}
