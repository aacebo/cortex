#[derive(Debug, Clone, PartialEq)]
pub struct Resource {
    /// ex. Gold, Silver, Coal
    pub name: &'static str,

    /// ex. $35 (gold value per unit)
    pub value: f32,
}
