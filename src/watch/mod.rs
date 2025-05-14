#![allow(dead_code)]
#![allow(unused)]

pub struct Event {
    kind: EventKind,
}

pub enum EventKind {
    Arrival,
    Change,
    Removal,
}

pub trait EventHandler {
    fn handle_event(&mut self, event: Event);
}

pub fn watch<T: EventHandler>(handler: T) {
    todo!()
}
