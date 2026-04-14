use crate::{Action, Effect, Event, Tick, World, entity};

pub struct Banks<'a> {
    tick: Tick,
    world: &'a mut World,
    events: &'a mut Vec<Event<'a, entity::Bank>>,
}

impl<'a> Banks<'a> {
    pub(crate) fn new(
        tick: Tick,
        world: &'a mut World,
        events: &'a mut Vec<Event<'a, entity::Bank>>,
    ) -> Self {
        Self {
            tick,
            world,
            events,
        }
    }

    pub fn get(&self, id: &entity::BankId) -> Option<&entity::Bank> {
        self.world.bank(id)
    }

    pub fn get_mut(&mut self, id: &entity::BankId) -> Option<&mut entity::Bank> {
        self.world.bank_mut(id)
    }

    pub fn create(&mut self, bank: entity::Bank) {
        if !self.world.banks.contains_key(&bank.id) {
            return;
        }

        let world = &self.world;
        let id = bank.id;
        self.world.banks.insert(bank.id, bank);

        for effect in self.effects {
            let event = Event::new(
                self.world.tick,
                Action::Create,
                self.world.bank(&id).unwrap(),
            );
            effect.on_bank_create(world, event);
        }
    }
}
