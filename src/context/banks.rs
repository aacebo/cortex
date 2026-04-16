use std::{collections::BTreeMap, sync::mpsc};

use crate::prelude::*;

pub struct Banks<'a> {
    store: &'a mut BTreeMap<BankId, Bank>,
    producer: mpsc::Sender<Message>,
}

impl<'a> Banks<'a> {
    pub(crate) fn new(
        store: &'a mut BTreeMap<BankId, Bank>,
        producer: mpsc::Sender<Message>,
    ) -> Self {
        Self { store, producer }
    }

    pub fn get(&self, id: &BankId) -> Option<&Bank> {
        self.store.get(id)
    }

    pub fn get_mut(&mut self, id: &BankId) -> Option<&mut Bank> {
        self.store.get_mut(id)
    }

    pub fn create(&mut self, id: BankId, name: impl Into<String>, ty: BankType) {
        let _ = self.producer.send(
            BankAction::from(CreateBankAction {
                id,
                name: name.into(),
                ty,
            })
            .to_message(),
        );
    }

    pub fn delete(&mut self, id: BankId) {
        let _ = self
            .producer
            .send(BankAction::from(DeleteBankAction { id }).to_message());
    }
}
