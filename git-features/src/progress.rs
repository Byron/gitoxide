/// The severity of a message
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum MessageLevel {
    /// Rarely sent information related to the progress, not to be confused with the progress itself
    Info,
    /// Used to indicate that a task has failed, along with the reason
    Failure,
    /// Indicates a task was completed successfully
    Success,
}

pub trait Progress {
    type SubProgress: Progress;

    /// Adds a new child, whose parent is this instance, with the given name.
    ///
    /// This will make the child progress to appear contained in the parent progress.
    fn add_child(&mut self, name: impl Into<String>) -> Self::SubProgress;

    /// Initialize the Item for receiving progress information.
    ///
    /// If `max` is `Some(…)`, it will be treated as upper bound. When progress is [set(…)](./struct.Item.html#method.set)
    /// it should not exceed the given maximum.
    /// If `max` is `None`, the progress is unbounded. Use this if the amount of work cannot accurately
    /// be determined in advance.
    ///
    /// If `unit` is `Some(…)`, it is used for display purposes only. It should be using the plural.
    ///
    /// If this method is never called, this `Progress` instance will serve as organizational unit, useful to add more structure
    /// to the progress tree (e.g. a headline).
    ///
    /// **Note** that this method can be called multiple times, changing the bounded-ness and unit at will.
    fn init(&mut self, max: Option<u32>, unit: Option<&'static str>);

    /// Set the current progress to the given `step`. The cost of this call is negligible,
    /// making manual throttling *not* necessary.
    ///
    /// **Note**: that this call has no effect unless `init(…)` was called before.
    fn set(&mut self, step: u32);

    /// Create a `message` of the given `level` and store it with the progress tree.
    ///
    /// Use this to provide additional,human-readable information about the progress
    /// made, including indicating success or failure.
    fn message(&self, level: MessageLevel, message: impl Into<String>);

    /// Create a message providing additional information about the progress thus far.
    fn info(&self, message: impl Into<String>) {
        self.message(MessageLevel::Info, message)
    }
    /// Create a message indicating the task is done successfully
    fn done(&self, message: impl Into<String>) {
        self.message(MessageLevel::Success, message)
    }
    /// Create a message indicating the task failed
    fn fail(&self, message: impl Into<String>) {
        self.message(MessageLevel::Failure, message)
    }
}

#[cfg(feature = "progress-log")]
mod log {
    use crate::progress::{MessageLevel, Progress};
    use std::time::Duration;

    pub struct LogProgress {
        name: String,
        max: Option<u32>,
        unit: Option<&'static str>,
        last_set: Option<std::time::SystemTime>,
    }

    const EMIT_LOG_EVERY_S: f32 = 0.5;

    impl LogProgress {
        pub fn new(name: impl Into<String>) -> Self {
            LogProgress {
                name: name.into(),
                max: None,
                unit: None,
                last_set: None,
            }
        }
    }

    impl Progress for LogProgress {
        type SubProgress = LogProgress;

        fn add_child(&mut self, name: impl Into<String>) -> Self::SubProgress {
            LogProgress::new(format!("{}::{}", self.name, Into::<String>::into(name)))
        }

        fn init(&mut self, max: Option<u32>, unit: Option<&'static str>) {
            self.max = max;
            self.unit = unit;
        }

        fn set(&mut self, step: u32) {
            let now = std::time::SystemTime::now();
            let last = self.last_set.unwrap_or(now);
            if now
                .duration_since(last)
                .unwrap_or_else(|_| Duration::default())
                .as_secs_f32()
                > EMIT_LOG_EVERY_S
            {
                match (self.max, self.unit) {
                    (Some(max), Some(unit)) => {
                        log::info!("{} → {} / {} {}", self.name, step, max, unit)
                    }
                    (None, Some(unit)) => log::info!("{} → {} {}", self.name, step, unit),
                    (Some(max), None) => log::info!("{} → {} / {}", self.name, step, max),
                    (None, None) => log::info!("{} → {}", self.name, step),
                }
            }
            self.last_set = Some(now);
        }

        fn message(&self, level: MessageLevel, message: impl Into<String>) {
            let message: String = message.into();
            match level {
                MessageLevel::Info => log::info!("ℹ{} → {}", self.name, message),
                MessageLevel::Failure => log::error!("𐄂{} → {}", self.name, message),
                MessageLevel::Success => log::info!("✓{} → {}", self.name, message),
            }
        }
    }
}

#[cfg(feature = "progress-log")]
pub use self::log::LogProgress;
