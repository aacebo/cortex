#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ResourceId(&'static str);

impl ResourceId {
    pub fn as_str(&self) -> &str {
        self.0
    }
}

impl std::fmt::Display for ResourceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ResourceType {
    Energy,
    Metal,
    Agricultural,
}

impl ResourceType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Energy => "energy",
            Self::Metal => "metal",
            Self::Agricultural => "agricultural",
        }
    }
}

impl std::fmt::Display for ResourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone)]
pub struct Resource {
    /// ex. gold, silver, coal
    pub id: ResourceId,

    /// ex. Gold, Silver, Coal
    pub name: &'static str,

    /// the type of resource
    pub ty: ResourceType,

    /// unit of measurement
    /// ex. barrel, troy_oz, metric_ton, bushel
    pub unit: &'static str,
}

// Energy

pub const OIL: Resource = Resource {
    id: ResourceId("oil"),
    name: "Oil",
    ty: ResourceType::Energy,
    unit: "barrel",
};

pub const NATURAL_GAS: Resource = Resource {
    id: ResourceId("natural_gas"),
    name: "Natural Gas",
    ty: ResourceType::Energy,
    unit: "mmbtu",
};

pub const COAL: Resource = Resource {
    id: ResourceId("coal"),
    name: "Coal",
    ty: ResourceType::Energy,
    unit: "metric_ton",
};

// Metal

pub const GOLD: Resource = Resource {
    id: ResourceId("gold"),
    name: "Gold",
    ty: ResourceType::Metal,
    unit: "troy_oz",
};

pub const SILVER: Resource = Resource {
    id: ResourceId("silver"),
    name: "Silver",
    ty: ResourceType::Metal,
    unit: "troy_oz",
};

pub const COPPER: Resource = Resource {
    id: ResourceId("copper"),
    name: "Copper",
    ty: ResourceType::Metal,
    unit: "metric_ton",
};

pub const IRON: Resource = Resource {
    id: ResourceId("iron"),
    name: "Iron",
    ty: ResourceType::Metal,
    unit: "metric_ton",
};

pub const STEEL: Resource = Resource {
    id: ResourceId("steel"),
    name: "Steel",
    ty: ResourceType::Metal,
    unit: "metric_ton",
};

pub const ALUMINUM: Resource = Resource {
    id: ResourceId("aluminum"),
    name: "Aluminum",
    ty: ResourceType::Metal,
    unit: "metric_ton",
};

// Agricultural

pub const WHEAT: Resource = Resource {
    id: ResourceId("wheat"),
    name: "Wheat",
    ty: ResourceType::Agricultural,
    unit: "bushel",
};

pub const CORN: Resource = Resource {
    id: ResourceId("corn"),
    name: "Corn",
    ty: ResourceType::Agricultural,
    unit: "bushel",
};

pub const COTTON: Resource = Resource {
    id: ResourceId("cotton"),
    name: "Cotton",
    ty: ResourceType::Agricultural,
    unit: "bale",
};

pub const LUMBER: Resource = Resource {
    id: ResourceId("lumber"),
    name: "Lumber",
    ty: ResourceType::Agricultural,
    unit: "board_foot",
};
