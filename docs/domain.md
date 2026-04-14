# Economic Simulator Reference

This is the latest distilled model from our design thread, with the current assumptions:

- No `Market` yet
- Core entities only: `Currency`, `Country`, `Bank`, `CurrencyExchange`, `Resource`
- Money stored as signed `i64` **minor units**
- `Bank` kept as one outer struct with nested `profile` and `state`
- `World` owns the full in-memory simulation state
- `Engine` is a thin tick runner; economics logic lives in systems, not in `World`

---

## Crate layout

```text
src/
  lib.rs
  main.rs

  sim/
    mod.rs
    world.rs
    tick.rs
    engine.rs

  domain/
    mod.rs
    country.rs
    currency.rs
    bank.rs
    exchange.rs
    resource.rs

  systems/
    mod.rs
    taxation.rs
    settlement.rs
    trade.rs

  value/
    mod.rs
    money.rs

  ids/
    mod.rs
```

---

## `ids.rs`

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CurrencyId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CountryId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BankId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CurrencyExchangeId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ResourceId(pub u32);
```

---

## `value/money.rs`

Use signed minor units. Do not pass raw `i64` around the domain.

```rust
use crate::ids::CurrencyId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Money {
    pub currency: CurrencyId,
    minor_units: i64,
}

impl Money {
    pub fn new(currency: CurrencyId, minor_units: i64) -> Self {
        Self { currency, minor_units }
    }

    pub fn minor_units(self) -> i64 {
        self.minor_units
    }

    pub fn checked_add(self, other: Self) -> Option<Self> {
        if self.currency != other.currency {
            return None;
        }

        Some(Self {
            currency: self.currency,
            minor_units: self.minor_units.checked_add(other.minor_units)?,
        })
    }

    pub fn checked_sub(self, other: Self) -> Option<Self> {
        if self.currency != other.currency {
            return None;
        }

        Some(Self {
            currency: self.currency,
            minor_units: self.minor_units.checked_sub(other.minor_units)?,
        })
    }
}
```

### Naming
- `minor_units` = generic name for cents / subunits
- `major units` = whole dollars, euros, etc.

---

## `domain/currency.rs`

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CurrencyKind {
    Fiat,
    CommodityBacked,
    Crypto,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExchangeRateRegime {
    Floating,
    Pegged { anchor: CurrencyId },
    ManagedFloat,
    CapitalControlled,
}

#[derive(Debug, Clone)]
pub struct Currency {
    pub id: CurrencyId,
    pub code: String,       // e.g. "USD"
    pub name: String,       // e.g. "US Dollar"
    pub symbol: String,     // e.g. "$"
    pub kind: CurrencyKind,
    pub issuer_country: Option<CountryId>,
    pub minor_unit_scale: u32, // 2 => cents, 0 => yen-style, 3 => mills
    pub regime: ExchangeRateRegime,
}
```

### Notes
- `code` may be a natural key, but for flexibility we kept a separate `CurrencyId`
- `minor_unit_scale` defines how `Money.minor_units` maps to display units

---

## `domain/country.rs`

```rust
use std::collections::BTreeSet;

#[derive(Debug, Clone)]
pub struct Country {
    pub id: CountryId,
    pub iso2: String,
    pub iso3: String,
    pub name: String,

    // structure / relationships
    pub legal_tender: BTreeSet<CurrencyId>,
    pub central_bank: Option<BankId>,
    pub domestic_banks: BTreeSet<BankId>,
    pub currency_exchanges: BTreeSet<CurrencyExchangeId>,
    pub owned_resources: BTreeSet<ResourceId>,
}
```

### Notes
At this stage, `Country` is mostly identity plus structural relationships.  
We intentionally did **not** add GDP, debt, tax revenue, inflation, etc. yet.

---

## `domain/bank.rs`

Current preferred model: one outer `Bank` struct, but with nested `profile` and `state`.

```rust
use std::collections::{BTreeMap, BTreeSet};

use crate::ids::{BankId, CountryId, CurrencyId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BankKind {
    Central,
    Commercial,
    Investment,
    Development,
}

#[derive(Debug, Clone)]
pub struct BankProfile {
    pub name: String,
    pub kind: BankKind,
    pub domicile: CountryId,
    pub base_currency: CurrencyId,
    pub supported_currencies: BTreeSet<CurrencyId>,
}

#[derive(Debug, Clone, Default)]
pub struct BankBalances {
    pub cash_minor_units: BTreeMap<CurrencyId, i64>,
    pub reserves_minor_units: BTreeMap<CurrencyId, i64>,
    pub deposits_minor_units: BTreeMap<CurrencyId, i64>,
    pub loans_outstanding_minor_units: BTreeMap<CurrencyId, i64>,
    pub debt_funding_minor_units: BTreeMap<CurrencyId, i64>,
}

#[derive(Debug, Clone, Default)]
pub struct BankState {
    pub balances: BankBalances,
    pub capital_minor_units: i64,
    pub liquidity_ratio_bps: i64,
    pub capital_ratio_bps: i64,
    pub insolvent: bool,
    pub illiquid: bool,
}

#[derive(Debug, Clone)]
pub struct Bank {
    pub id: BankId,
    pub profile: BankProfile,
    pub state: BankState,
}

impl Bank {
    pub fn supports_currency(&self, currency: CurrencyId) -> bool {
        self.profile.supported_currencies.contains(&currency)
    }

    pub fn cash_in_minor_units(&self, currency: CurrencyId) -> i64 {
        self.state
            .balances
            .cash_minor_units
            .get(&currency)
            .copied()
            .unwrap_or_default()
    }
}
```

### Notes
- `Bank` and `BankState` are logically separated, but still packaged together in one entity
- This is the right middle ground for an early simulator
- Ratios are stored in basis points-like integer form rather than `f64`

---

## `domain/exchange.rs`

This is a **currency exchange institution**, not a stock exchange and not a generic market.

```rust
use std::collections::{BTreeMap, BTreeSet};

use crate::ids::{BankId, CountryId, CurrencyExchangeId, CurrencyId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CurrencyExchangeKind {
    Retail,
    BankDesk,
    Interbank,
    CentralBankWindow,
    Digital,
}

#[derive(Debug, Clone)]
pub struct CurrencyExchange {
    pub id: CurrencyExchangeId,
    pub name: String,
    pub kind: CurrencyExchangeKind,
    pub domicile: CountryId,

    // optional backing institution
    pub operator_bank: Option<BankId>,

    // what it can convert
    pub supported_currencies: BTreeSet<CurrencyId>,

    // local inventory / float
    pub reserves_minor_units: BTreeMap<CurrencyId, i64>,

    // friction
    pub fee_bps: u32,
    pub spread_bps: u32,

    // controls
    pub capital_controls_enforced: bool,
    pub active: bool,
}
```

### Notes
This models:
- airport booths
- commercial-bank FX desks
- central bank conversion windows
- digital FX providers

It does **not** model price discovery. No `Market` yet.

---

## `domain/resource.rs`

```rust
use std::collections::BTreeSet;

use crate::ids::{CountryId, ResourceId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceKind {
    Energy,
    Metal,
    Agriculture,
    Water,
    Labor,
}

#[derive(Debug, Clone)]
pub struct Resource {
    pub id: ResourceId,
    pub name: String,     // e.g. "Oil"
    pub symbol: String,   // e.g. "OIL"
    pub kind: ResourceKind,
    pub unit: String,     // e.g. "barrel", "ton", "MWh"
    pub producing_countries: BTreeSet<CountryId>,
}
```

### Notes
This is intentionally simple:
- identity
- category
- unit
- producers

No pricing or inventory yet.

---

## `sim/tick.rs`

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Tick(pub u64);

impl Tick {
    pub fn next(self) -> Self {
        Self(self.0 + 1)
    }
}
```

---

## `sim/world.rs`

`World` holds the current in-memory simulation state.  
It should **not** contain economic logic.

```rust
use std::collections::BTreeMap;

use crate::domain::{
    bank::Bank,
    country::Country,
    currency::Currency,
    exchange::CurrencyExchange,
    resource::Resource,
};
use crate::ids::{BankId, CountryId, CurrencyExchangeId, CurrencyId, ResourceId};
use crate::sim::tick::Tick;

#[derive(Debug, Default)]
pub struct World {
    pub tick: Tick,

    pub countries: BTreeMap<CountryId, Country>,
    pub currencies: BTreeMap<CurrencyId, Currency>,
    pub banks: BTreeMap<BankId, Bank>,
    pub exchanges: BTreeMap<CurrencyExchangeId, CurrencyExchange>,
    pub resources: BTreeMap<ResourceId, Resource>,
}

impl World {
    pub fn tick(&self) -> Tick {
        self.tick
    }

    pub fn advance_tick(&mut self) {
        self.tick = self.tick.next();
    }
}
```

### Notes
`world.rs` should contain:
- `World`
- maybe small accessors
- tick ownership

It should **not** contain:
- tax logic
- settlement logic
- lending logic
- FX logic
- scenario loading

---

## `sim/engine.rs`

The engine should be thin.  
Its job is to run systems in order each tick.

```rust
use crate::sim::world::World;

pub trait System {
    fn run(&mut self, world: &mut World);
}

#[derive(Default)]
pub struct Engine {
    systems: Vec<Box<dyn System>>,
}

impl Engine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_system(mut self, system: impl System + 'static) -> Self {
        self.systems.push(Box::new(system));
        self
    }

    pub fn push_system(&mut self, system: impl System + 'static) {
        self.systems.push(Box::new(system));
    }

    pub fn step(&mut self, world: &mut World) {
        for system in &mut self.systems {
            system.run(world);
        }

        world.advance_tick();
    }

    pub fn run_for(&mut self, world: &mut World, ticks: u64) {
        for _ in 0..ticks {
            self.step(world);
        }
    }
}
```

### Notes
The engine:
- owns system ordering
- runs each system against `World`
- advances the tick

The engine should **not** own domain policy.

---

## `systems/mod.rs`

Example placeholder systems module:

```rust
pub mod taxation;
pub mod settlement;
pub mod trade;
```

Example no-op system shape:

```rust
use crate::sim::{engine::System, world::World};

pub struct SettlementSystem;

impl System for SettlementSystem {
    fn run(&mut self, world: &mut World) {
        let _ = world;
        // settlement logic goes here
    }
}
```

---

## Design rules

### 1. Prefer IDs over string references
Use:
- `CountryId`
- `CurrencyId`
- `BankId`

Do not use names/codes as relational pointers in the domain.

### 2. Keep world state in memory, logic in systems
- `World` stores
- `System`s mutate

### 3. Use integer money storage
- balances: `i64`
- ratios: integer basis points if possible
- avoid `f64` for cash

### 4. Keep config-ish fields separate from runtime-ish fields
Even when using one outer struct, preserve internal separation:

```rust
Bank { id, profile, state }
```

### 5. Do not add `Market` yet
That belongs later when you want:
- price discovery
- order books
- spreads by venue
- commodity pricing
- securities

---

## Current minimal `domain/mod.rs`

```rust
pub mod bank;
pub mod country;
pub mod currency;
pub mod exchange;
pub mod resource;
```

---

## Current minimal `sim/mod.rs`

```rust
pub mod engine;
pub mod tick;
pub mod world;
```

---

## Summary

Current agreed model:

- `Currency`
- `Country`
- `Bank`
- `CurrencyExchange`
- `Resource`
- `Money`
- `World`
- `Engine`

With these design choices:

- no market yet
- `i64` money storage in minor units
- `Bank` as one outer struct with nested `profile` and `state`
- `World` as the simulation root
- `Engine` as a thin system runner
