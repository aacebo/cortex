mod bank;

pub use bank::*;

use crate::{Effect, World};

pub struct Context<'a> {
    world: &'a mut World,
    effects: &'a [&'a dyn Effect],
    banks: BankContext<'a>,
}

impl<'a> Context<'a> {
    pub(crate) fn new(world: &'a mut World, effects: &'a [&'a dyn Effect]) -> Self {
        Self {
            world,
            effects,
            banks: BankContext::new(world, effects),
        }
    }
}
