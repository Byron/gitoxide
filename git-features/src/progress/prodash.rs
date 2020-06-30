use crate::progress::{MessageLevel, Progress};

use prodash::tree::{Item, MessageLevel as TreeMessageLevel};

impl Progress for Item {
    type SubProgress = Item;

    fn add_child(&mut self, name: impl Into<String>) -> Self::SubProgress {
        Item::add_child(self, name)
    }

    fn init(&mut self, max: Option<u32>, unit: Option<&'static str>) {
        Item::init(self, max, unit)
    }

    fn set(&mut self, step: u32) {
        Item::set(self, step)
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
