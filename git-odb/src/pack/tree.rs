use quick_error::quick_error;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        InvariantIncreasingPackOffset(last_pack_offset: u64, pack_offset: u64) {
            display("Pack offsets must only increment. The previous pack offset was {}, the current one is {}", last_pack_offset, pack_offset)
        }
        InvariantNonEmpty {
            display("Is there ever a need to create empty indices? If so, please post a PR.")
        }
        InvariantBasesBeforeDeltasNeedThem(delta_pack_offset: u64, base_pack_offset: u64) {
            display("The delta at pack offset {} could not find its base at {} - it should have been seen already", delta_pack_offset, base_pack_offset)
        }
    }
}

struct TreeItem<D> {
    offset: u64,
    data: D,
    // TODO: figure out average amount of children per node and use smallvec instead
    children: Vec<usize>,
}

pub(crate) struct Tree<D> {
    items: Vec<TreeItem<D>>,
    last_added_offset: u64,
}

impl<D> Tree<D> {
    pub fn new(num_objects: usize) -> Result<Self, Error> {
        if num_objects == 0 {
            return Err(Error::InvariantNonEmpty);
        }
        Ok(Tree {
            items: Vec::with_capacity(num_objects),
            last_added_offset: 0,
        })
    }

    fn assert_is_incrementing(&mut self, offset: u64) -> Result<u64, Error> {
        if offset > self.last_added_offset {
            self.last_added_offset = offset;
            Ok(offset)
        } else {
            Err(Error::InvariantIncreasingPackOffset(self.last_added_offset, offset))
        }
    }

    pub fn add_root(&mut self, offset: u64, data: D) -> Result<(), Error> {
        let offset = self.assert_is_incrementing(offset)?;
        self.items.push(TreeItem {
            offset,
            data,
            children: Default::default(),
        });
        Ok(())
    }
    pub fn add_child(&mut self, base_offset: u64, offset: u64, data: D) -> Result<(), Error> {
        let offset = self.assert_is_incrementing(offset)?;
        let base_index = self
            .items
            .binary_search_by_key(&base_offset, |e| e.offset)
            .map_err(|_| Error::InvariantBasesBeforeDeltasNeedThem(offset, base_offset))?;
        let child_index = self.items.len();
        self.items[base_index].children.push(child_index);
        self.items.push(TreeItem {
            offset,
            data,
            children: Default::default(),
        });
        Ok(())
    }
}
