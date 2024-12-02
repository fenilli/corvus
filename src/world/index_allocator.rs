#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Index {
    pub(super) id: usize,
    pub(super) generation: u32,
}

#[derive(Debug)]
enum AllocatorEntry {
    Free(usize),
    Occupied,
}

#[derive(Debug)]
pub struct IndexAllocator {
    entries: Vec<(AllocatorEntry, u32)>,
    free_head: usize,
}

impl IndexAllocator {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            free_head: 0,
        }
    }

    pub fn allocate(&mut self) -> Index {
        match self.entries.get_mut(self.free_head) {
            // Already used Entry
            Some(entry) => match entry.0 {
                AllocatorEntry::Free(next_free) => {
                    let index = Index {
                        id: self.free_head,
                        generation: entry.1,
                    };
                    self.free_head = next_free;
                    *entry = (AllocatorEntry::Occupied, entry.1);

                    index
                }
                AllocatorEntry::Occupied => {
                    panic!("Trying to allocate an already occupied index")
                }
            },
            // New Entry
            None => {
                let generation = 0;
                let id = self.entries.len();
                self.entries.push((AllocatorEntry::Occupied, generation));
                self.free_head = id + 1;
                Index { id, generation }
            }
        }
    }

    pub fn deallocate(&mut self, index: Index) -> bool {
        let entry = &mut self.entries[index.id];

        match entry.0 {
            AllocatorEntry::Occupied => {
                if entry.1 != index.generation {
                    return false;
                }

                *entry = (AllocatorEntry::Free(self.free_head), entry.1 + 1);
                self.free_head = index.id;

                true
            }
            AllocatorEntry::Free(_) => false,
        }
    }
}
