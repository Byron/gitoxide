use crate::{pack::index::write::EntrySlice, pack::tree::Node};
use git_features::{parallel, parallel::in_parallel_if, progress::Progress};

pub fn chunks<'a, T: 'a + Send, F, P>(
    it: impl Iterator<Item = Vec<Node<'a, T>>> + Send,
    num_objects: u32,
    should_run_in_parallel: impl FnOnce() -> bool,
    resolve: F,
    mut progress: P,
    thread_limit: Option<usize>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>>
where
    F: for<'r> Fn(EntrySlice, &'r mut Vec<u8>) -> Option<()> + Send + Sync,
    P: Progress,
    <P as Progress>::SubProgress: Send,
{
    let (chunk_size, thread_limit, _) = parallel::optimize_chunk_size_and_thread_limit(1, None, thread_limit, None);
    let reduce_progress = parking_lot::Mutex::new(progress.add_child("Resolving"));
    in_parallel_if(
        should_run_in_parallel,
        it,
        thread_limit,
        |thread_index| {
            (
                Vec::<u8>::with_capacity(4096),
                reduce_progress.lock().add_child(format!("thread {}", thread_index)),
            )
        },
        // |root_nodes, state| apply_deltas(root_nodes, state, &resolver, kind.hash()),
        |_root_nodes, _state| Ok(0),
        Reducer::new(num_objects, &reduce_progress),
    )
}

pub(crate) struct Reducer<'a, P> {
    item_count: usize,
    progress: &'a parking_lot::Mutex<P>,
    start: std::time::Instant,
}

impl<'a, P> Reducer<'a, P>
where
    P: Progress,
{
    pub fn new(num_objects: u32, progress: &'a parking_lot::Mutex<P>) -> Self {
        progress.lock().init(Some(num_objects), Some("objects"));
        Reducer {
            item_count: 0,
            progress,
            start: std::time::Instant::now(),
        }
    }
}

impl<'a, P> parallel::Reducer for Reducer<'a, P>
where
    P: Progress,
{
    type Input = Result<usize, Box<dyn std::error::Error + Send + Sync>>;
    type Output = ();
    type Error = Box<dyn std::error::Error + Send + Sync>;

    fn feed(&mut self, input: Self::Input) -> Result<(), Self::Error> {
        let input = input?;
        self.item_count += input;
        self.progress.lock().set(self.item_count as u32);
        Ok(())
    }

    fn finalize(self) -> Result<Self::Output, Self::Error> {
        self.progress
            .lock()
            .show_throughput(self.start, self.item_count as u32, "objects");
        Ok(())
    }
}
