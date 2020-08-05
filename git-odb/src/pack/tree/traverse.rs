use crate::{
    pack::index::write::EntrySlice,
    pack::tree::{Item, Tree},
};
use git_features::{parallel, parallel::in_parallel_if, progress::Progress};
use git_object::HashKind;

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
        MBFN: for<'r> Fn(&'r mut T, &'r [u8], HashKind) -> BR,
        BR: Clone,
        MCFN: for<'r> Fn(&'r mut T, BR),
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
            // |root_nodes, state| apply_deltas(root_nodes, state, &resolver, kind.hash()),
            |_root_nodes, _state| Ok(0),
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

mod apply {
    use crate::{
        hash, loose,
        pack::{self, index::write::EntrySlice, index::write::Error},
        zlib,
    };
    use git_features::progress::Progress;
    use git_object::{owned, HashKind};
    use std::{cell::RefCell, collections::BTreeMap, io};

    pub(crate) fn apply_deltas<T, F, P, MBFN, BR, MCFN>(
        nodes: Vec<pack::tree::Node<T>>,
        (bytes_buf, progress): &mut (Vec<u8>, P),
        resolve: F,
        hash_kind: HashKind,
        modify_base: MBFN,
        modify_child: MCFN,
    ) -> Result<usize, Error>
    where
        F: for<'r> Fn(EntrySlice, &'r mut Vec<u8>) -> Option<()> + Send + Sync,
        P: Progress,
        MBFN: for<'r> Fn(&'r mut T, &'r [u8], HashKind) -> BR,
        BR: Clone,
        MCFN: for<'r> Fn(&'r mut T, BR),
        T: Default,
    {
        let mut decompressed_bytes_by_pack_offset = BTreeMap::new();
        let bytes_buf = RefCell::new(bytes_buf);
        let mut num_objects = 0;
        let decompress_from_resolver = |slice: EntrySlice| -> Result<Vec<u8>, Error> {
            let mut bytes_buf = bytes_buf.borrow_mut();
            bytes_buf.resize((slice.end - slice.start) as usize, 0);
            resolve(slice.clone(), &mut bytes_buf).ok_or_else(|| Error::ConsumeResolveFailed(slice.start))?;
            let entry = pack::data::Entry::from_bytes(&bytes_buf, slice.start);
            decompress_all_at_once(
                &bytes_buf[entry.header_size() as usize..],
                entry.decompressed_size as usize,
            )
        };

        // Traverse the tree breadth first and loose the data produced for the base as it won't be needed anymore.
        progress.init(None, Some("objects"));

        // each node is a base, and its children always start out as deltas which become a base after applying them.
        // These will be pushed onto our stack until all are processed
        let root_level = 0;
        let mut nodes: Vec<_> = nodes.into_iter().map(|n| (root_level, n)).collect();
        while let Some((level, mut base)) = nodes.pop() {
            let base_bytes = if level == root_level {
                decompress_from_resolver(base.entry_slice())?
            } else {
                decompressed_bytes_by_pack_offset
                    .remove(&base.offset())
                    .expect("we store the resolved delta buffer when done")
            };

            let base_result = modify_base(&mut base.data, &base_bytes, hash_kind);
            num_objects += 1;
            for mut child in base.store_changes_then_into_child_iter() {
                let delta_bytes = decompress_from_resolver(child.entry_slice())?;
                let (base_size, consumed) = pack::data::decode::delta_header_size_ofs(&delta_bytes);
                let mut header_ofs = consumed;
                assert_eq!(
                    base_bytes.len(),
                    base_size as usize,
                    "recorded base size in delta does not match"
                );
                let (result_size, consumed) = pack::data::decode::delta_header_size_ofs(&delta_bytes[consumed..]);
                header_ofs += consumed;

                let mut fully_resolved_delta_bytes = bytes_buf.borrow_mut();
                fully_resolved_delta_bytes.resize(result_size as usize, 0);
                pack::data::decode::apply_delta(
                    &base_bytes,
                    &mut fully_resolved_delta_bytes,
                    &delta_bytes[header_ofs..],
                );

                decompressed_bytes_by_pack_offset.insert(child.offset(), fully_resolved_delta_bytes.to_owned());
                modify_child(&mut child.data, base_result.clone());
                nodes.push((level + 1, child));
            }
        }

        Ok(num_objects)
    }

    fn compute_hash(kind: git_object::Kind, bytes: &[u8], hash_kind: HashKind) -> owned::Id {
        let mut write = hash::Write::new(io::sink(), hash_kind);
        loose::object::header::encode(kind, bytes.len() as u64, &mut write)
            .expect("write to sink and hash cannot fail");
        write.hash.update(bytes);
        owned::Id::from(write.hash.digest())
    }

    fn decompress_all_at_once(b: &[u8], decompressed_len: usize) -> Result<Vec<u8>, Error> {
        let mut out = Vec::new();
        out.resize(decompressed_len, 0);
        zlib::Inflate::default()
            .once(&b, &mut out, true)
            .map_err(|err| Error::ConsumeZlibInflate(err, "Failed to decompress entry"))?;
        Ok(out)
    }
}
