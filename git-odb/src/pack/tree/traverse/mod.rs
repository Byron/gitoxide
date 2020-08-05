use crate::{
    pack::index::write::EntrySlice,
    pack::tree::{Item, Tree},
};
use git_features::{parallel, parallel::in_parallel_if, progress::Progress};
use git_object::HashKind;

mod resolve;

impl<T> Tree<T>
where
    T: Default + Send,
{
    pub fn traverse<F, P, MBFN, BR, MCFN>(
        mut self,
        should_run_in_parallel: impl FnOnce() -> bool,
        resolve: F,
        mut progress: P,
        thread_limit: Option<usize>,
        pack_entries_end: u64,
        hash_kind: HashKind,
        modify_base: MBFN,
        modify_child: MCFN,
    ) -> Result<Vec<Item<T>>, Box<dyn std::error::Error + Send + Sync>>
    where
        F: for<'r> Fn(EntrySlice, &'r mut Vec<u8>) -> Option<()> + Send + Sync,
        P: Progress,
        <P as Progress>::SubProgress: Send,
        MBFN: for<'r> Fn(&'r mut T, &'r [u8], HashKind) -> BR + Send + Sync,
        BR: Clone,
        MCFN: for<'r> Fn(&'r mut T, BR) + Send + Sync,
    {
        self.pack_entries_end = Some(pack_entries_end);
        let (chunk_size, thread_limit, _) = parallel::optimize_chunk_size_and_thread_limit(1, None, thread_limit, None);
        let reduce_progress = parking_lot::Mutex::new(progress.add_child("Resolving"));

        // SAFETY: We are owning 'self', and it's the UnsafeCell which we are supposed to use requiring unsafe on every access now.
        #[allow(unsafe_code)]
        let num_objects = unsafe { (*self.items.get()).len() } as u32;
        in_parallel_if(
            should_run_in_parallel,
            self.iter_root_chunks(chunk_size),
            thread_limit,
            |thread_index| {
                (
                    Vec::<u8>::with_capacity(4096),
                    reduce_progress.lock().add_child(format!("thread {}", thread_index)),
                )
            },
            |root_nodes, state| {
                resolve::deltas(root_nodes, state, &resolve, hash_kind, &modify_base, &modify_child).map_err(Into::into)
            },
            Reducer::new(num_objects, &reduce_progress),
        )?;
        Ok(self.into_items())
    }
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
