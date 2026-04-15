use std::sync::mpsc;

use crate::*;

pub struct Context<'a> {
    pub banks: bank::Banks<'a>,
    pub countries: country::Countries<'a>,

    messages: mpsc::Sender<Message>,
}

impl<'a> Context<'a> {
    pub(crate) fn new(world: &'a mut World, messages: mpsc::Sender<Message>) -> Self {
        Self {
            banks: bank::Banks::new(&mut world.banks, messages.clone()),
            countries: country::Countries::new(&mut world.countries, messages.clone()),
            messages,
        }
    }

    pub fn shutdown(&mut self, command: ShutdownRequest) {
        let _ = self.messages.send(Command::Shutdown(command).into());
    }
}
