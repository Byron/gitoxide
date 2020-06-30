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
    fn message(&mut self, level: MessageLevel, message: impl Into<String>);

    /// Create a message providing additional information about the progress thus far.
    fn info(&mut self, message: impl Into<String>) {
        self.message(MessageLevel::Info, message)
    }
    /// Create a message indicating the task is done successfully
    fn done(&mut self, message: impl Into<String>) {
        self.message(MessageLevel::Success, message)
    }
    /// Create a message indicating the task failed
    fn fail(&mut self, message: impl Into<String>) {
        self.message(MessageLevel::Failure, message)
    }
}

#[cfg(feature = "progress-log")]
mod log;

#[cfg(feature = "progress-log")]
pub use self::log::LogProgress;

#[cfg(feature = "progress-prodash")]
mod prodash;

pub struct DiscardProgress;

impl Progress for DiscardProgress {
    type SubProgress = DiscardProgress;

    fn add_child(&mut self, _name: impl Into<String>) -> Self::SubProgress {
        DiscardProgress
    }

    fn init(&mut self, _max: Option<u32>, _unit: Option<&'static str>) {}

    fn set(&mut self, _step: u32) {}

    fn message(&mut self, _level: MessageLevel, _message: impl Into<String>) {}
}
