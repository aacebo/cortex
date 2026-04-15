use std::sync::mpsc;

use crate::*;

pub struct Context<'a> {
    pub banks: bank::Banks<'a>,
    pub countries: country::Countries<'a>,
}

impl<'a> Context<'a> {
    pub(crate) fn new(world: &'a mut World, producer: mpsc::Sender<Action>) -> Self {
        Self {
            banks: bank::Banks::new(&mut world.banks, producer.clone()),
            countries: country::Countries::new(&mut world.countries, producer),
        }
    }
}
