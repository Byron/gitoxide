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

struct Item<D> {
    offset: u64,
    _data: Option<D>,
    // TODO: figure out average amount of children per node and use smallvec instead
    children: Vec<usize>,
}

pub(crate) struct Tree<D> {
    items: Vec<Item<D>>,
    last_added_offset: u64,
    // assure we truly create only one iterator, ever, to avoid violating access rules
    iterator_created: bool,
}

impl<D> Tree<D> {
    pub fn new(num_objects: usize) -> Result<Self, Error> {
        if num_objects == 0 {
            return Err(Error::InvariantNonEmpty);
        }
        Ok(Tree {
            items: Vec::with_capacity(num_objects),
            last_added_offset: 0,
            iterator_created: false,
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
        assert!(
            !self.iterator_created,
            "Cannot mutate after the iterator was created as it assumes exclusive access"
        );
        let offset = self.assert_is_incrementing(offset)?;
        self.items.push(Item {
            offset,
            _data: Some(data),
            children: Default::default(),
        });
        Ok(())
    }
    pub fn add_child(&mut self, base_offset: u64, offset: u64, data: D) -> Result<(), Error> {
        assert!(
            !self.iterator_created,
            "Cannot mutate after the iterator was created as it assumes exclusive access"
        );
        let offset = self.assert_is_incrementing(offset)?;
        let base_index = self
            .items
            .binary_search_by_key(&base_offset, |e| e.offset)
            .map_err(|_| Error::InvariantBasesBeforeDeltasNeedThem(offset, base_offset))?;
        let child_index = self.items.len();
        self.items[base_index].children.push(child_index);
        self.items.push(Item {
            offset,
            _data: Some(data),
            children: Default::default(),
        });
        Ok(())
    }

    pub fn iter_chunks(&mut self, size: usize) -> Chunks<D> {
        // We would love to consume the tree, of course, but if we can't hand out items that borrow from ourselves,
        // it's nothing we can use effectively. Thus it's better to check at runtime…
        assert!(
            !self.iterator_created,
            "Can only create a single iterator to avoid aliasing mutable tree nodes"
        );
        self.iterator_created = true;
        Chunks { inner: self, size }
    }
}

pub struct Node<'a, D> {
    _owner: &'a Tree<D>,
    pub _data: D,
    // TODO: figure out average amount of children per node and use smallvec instead
    children: Vec<usize>,
}

pub struct Chunks<'a, D> {
    inner: &'a Tree<D>,
    size: usize,
}

impl<'a, D> Iterator for Chunks<'a, D> {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}
