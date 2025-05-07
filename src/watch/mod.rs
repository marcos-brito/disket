pub struct Event {
    kind: EventKind,
}

pub enum EventKind {
    Some,
    Other,
}

pub trait EventHandler {
    fn handle_event(&mut self, event: Event);
}

pub fn watch<T: EventHandler>(handler: T) {
    todo!()
}
