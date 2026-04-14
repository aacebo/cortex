#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ResourceId(&'static str);

#[derive(Debug, Clone, PartialEq)]
pub struct Resource {
    /// ex. gold, silver, coal
    pub id: ResourceId,

    /// ex. Gold, Silver, Coal
    pub name: &'static str,
}
