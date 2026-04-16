use std::{collections::BTreeMap, sync::mpsc};

use crate::prelude::*;

pub struct Countries<'a> {
    store: &'a mut BTreeMap<CountryId, Country>,
    producer: mpsc::Sender<Message>,
}

impl<'a> Countries<'a> {
    pub(crate) fn new(
        store: &'a mut BTreeMap<CountryId, Country>,
        producer: mpsc::Sender<Message>,
    ) -> Self {
        Self { store, producer }
    }

    pub fn get(&self, id: &CountryId) -> Option<&Country> {
        self.store.get(id)
    }

    pub fn get_mut(&mut self, id: &CountryId) -> Option<&mut Country> {
        self.store.get_mut(id)
    }

    pub fn create(&mut self, id: CountryId, name: impl Into<String>) {
        let _ = self.producer.send(
            CountryAction::Create(CreateCountryAction {
                id,
                name: name.into(),
            })
            .to_action()
            .into(),
        );
    }

    pub fn delete(&mut self, id: CountryId) {
        let _ = self
            .producer
            .send(CountryAction::from(DeleteCountryAction { id }).to_message());
    }
}
