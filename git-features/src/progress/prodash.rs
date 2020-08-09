use crate::progress::{MessageLevel, Progress};

use prodash::tree::{Item, MessageLevel as TreeMessageLevel};

impl Progress for Item {
    type SubProgress = Item;

    fn add_child(&mut self, name: impl Into<String>) -> Self::SubProgress {
        Item::add_child(self, name)
    }

    fn init(&mut self, max: Option<usize>, unit: Option<&'static str>) {
        Item::init(self, max, unit.map(|u| u.into()))
    }

    fn set(&mut self, step: usize) {
        Item::set(self, step)
    }

    fn inc_by(&mut self, step: usize) {
        self.inc_by(step)
    }

    fn message(&mut self, level: MessageLevel, message: impl Into<String>) {
        Item::message(
            self,
            match level {
                MessageLevel::Success => TreeMessageLevel::Success,
                MessageLevel::Failure => TreeMessageLevel::Failure,
                MessageLevel::Info => TreeMessageLevel::Info,
            },
            message,
        )
    }
}
