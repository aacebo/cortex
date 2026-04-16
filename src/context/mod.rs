pub mod banks;
pub mod countries;

pub use banks::*;
pub use countries::*;

use std::sync::mpsc;

use crate::prelude::*;

pub struct Context<'a> {
    pub banks: Banks<'a>,
    pub countries: Countries<'a>,

    messages: mpsc::Sender<Message>,
}

impl<'a> Context<'a> {
    pub(crate) fn new(world: &'a mut World, messages: mpsc::Sender<Message>) -> Self {
        Self {
            banks: Banks::new(&mut world.banks, messages.clone()),
            countries: Countries::new(&mut world.countries, messages.clone()),
            messages,
        }
    }

    pub fn shutdown(&mut self, command: ShutdownRequest) {
        let _ = self.messages.send(Command::Shutdown(command).into());
    }
}
