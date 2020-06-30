pub trait Progress {
    /// Adds a new child, whose parent is this instance, with the given name.
    ///
    /// This will make the child progress to appear contained in the parent progress.
    fn add_child(&self, name: impl Into<String>) -> Self;

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

    /// Set the current progress to the given `step`.
    ///
    /// **Note**: that this call has no effect unless `init(…)` was called before.
    fn set(&mut self, step: u32);

    /// Create a message providing additional information about the progress thus far.
    fn info(&self, name: impl Into<String>) -> Self;
    /// Create a message indicating the task is done successfully
    fn done(&self, name: impl Into<String>) -> Self;
    /// Create a message indicating the task failed
    fn fail(&self, name: impl Into<String>) -> Self;
}
