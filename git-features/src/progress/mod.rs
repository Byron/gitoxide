use std::time::Instant;

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

    /// Increment the current progress to the given `step`. The cost of this call is negligible,
    /// making manual throttling *not* necessary.
    ///
    /// **Note**: that this call has no effect unless `init(…)` was called before.
    fn inc_by(&mut self, step: u32);

    /// Increment the current progress to the given 1. The cost of this call is negligible,
    /// making manual throttling *not* necessary.
    ///
    /// **Note**: that this call has no effect unless `init(…)` was called before.
    fn inc(&mut self) {
        self.inc_by(1)
    }

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
    /// A shorthand to print throughput information
    fn show_throughput(&mut self, start: Instant, total_items: u32, item_name: &str) {
        let elapsed = start.elapsed().as_secs_f32();
        self.info(format!(
            "done {} {} in {:.02}s ({} {}/s)",
            total_items,
            item_name,
            elapsed,
            total_items as f32 / elapsed,
            item_name
        ));
    }
}

pub struct Discard;

impl Progress for Discard {
    type SubProgress = Discard;

    fn add_child(&mut self, _name: impl Into<String>) -> Self::SubProgress {
        Discard
    }

    fn init(&mut self, _max: Option<u32>, _unit: Option<&'static str>) {}

    fn set(&mut self, _step: u32) {}

    fn inc_by(&mut self, _step: u32) {}

    fn message(&mut self, _level: MessageLevel, _message: impl Into<String>) {}
}

pub enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<L, R> Progress for Either<L, R>
where
    L: Progress,
    R: Progress,
{
    type SubProgress = Either<L::SubProgress, R::SubProgress>;

    fn add_child(&mut self, name: impl Into<String>) -> Self::SubProgress {
        match self {
            Either::Left(l) => Either::Left(l.add_child(name)),
            Either::Right(r) => Either::Right(r.add_child(name)),
        }
    }

    fn init(&mut self, max: Option<u32>, unit: Option<&'static str>) {
        match self {
            Either::Left(l) => l.init(max, unit),
            Either::Right(r) => r.init(max, unit),
        }
    }

    fn set(&mut self, step: u32) {
        match self {
            Either::Left(l) => l.set(step),
            Either::Right(r) => r.set(step),
        }
    }

    fn inc_by(&mut self, step: u32) {
        match self {
            Either::Left(l) => l.inc_by(step),
            Either::Right(r) => r.inc_by(step),
        }
    }

    fn message(&mut self, level: MessageLevel, message: impl Into<String>) {
        match self {
            Either::Left(l) => l.message(level, message),
            Either::Right(r) => r.message(level, message),
        }
    }
}

pub struct DoOrDiscard<T>(Either<T, Discard>);

impl<T> From<Option<T>> for DoOrDiscard<T>
where
    T: Progress,
{
    fn from(p: Option<T>) -> Self {
        match p {
            Some(p) => DoOrDiscard(Either::Left(p)),
            None => DoOrDiscard(Either::Right(Discard)),
        }
    }
}
impl<T> DoOrDiscard<T> {
    pub fn into_inner(self) -> Option<T> {
        match self {
            DoOrDiscard(Either::Left(p)) => Some(p),
            DoOrDiscard(Either::Right(_)) => None,
        }
    }
}

impl<T> Progress for DoOrDiscard<T>
where
    T: Progress,
{
    type SubProgress = DoOrDiscard<T::SubProgress>;

    fn add_child(&mut self, name: impl Into<String>) -> Self::SubProgress {
        DoOrDiscard(self.0.add_child(name))
    }

    fn init(&mut self, max: Option<u32>, unit: Option<&'static str>) {
        self.0.init(max, unit)
    }

    fn set(&mut self, step: u32) {
        self.0.set(step)
    }

    fn inc_by(&mut self, step: u32) {
        self.0.inc_by(step)
    }

    fn message(&mut self, level: MessageLevel, message: impl Into<String>) {
        self.0.message(level, message)
    }
}

#[cfg(feature = "progress-log")]
mod log;

#[cfg(feature = "progress-log")]
pub use self::log::Log;

#[cfg(feature = "progress-prodash")]
mod prodash;
