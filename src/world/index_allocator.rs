#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Index {
    pub(super) id: usize,
    pub(super) generation: u32,
}

impl std::fmt::Display for Index {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Index")
            .field("id", &self.id)
            .field("generation", &self.generation)
            .finish()
    }
}

#[derive(Debug)]
enum AllocatorEntry {
    Free(u32),
    Occupied(u32),
}

#[derive(Debug)]
pub struct IndexAllocator {
    entries: Vec<AllocatorEntry>,
    free_list: Vec<usize>,
}

impl IndexAllocator {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            free_list: Vec::new(),
        }
    }

    pub fn allocate(&mut self) -> Index {
        let Some(id) = self.free_list.pop() else {
            let id = self.entries.len();
            self.entries.push(AllocatorEntry::Occupied(0));
            return Index { id, generation: 0 };
        };

        match self.entries[id] {
            AllocatorEntry::Occupied(_) => {
                panic!("Trying to allocate an already occupied index!");
            }
            AllocatorEntry::Free(generation) => {
                self.entries[id] = AllocatorEntry::Occupied(generation);
                Index { id, generation }
            }
        }
    }

    pub fn deallocate(&mut self, index: Index) -> bool {
        let Some(entry) = self.entries.get(index.id) else {
            return false;
        };

        let AllocatorEntry::Occupied(generation) = entry else {
            return false;
        };

        self.entries[index.id] = AllocatorEntry::Free(generation + 1);
        self.free_list.push(index.id);

        true
    }

    pub fn is_valid(&self, index: Index) -> bool {
        matches!(
            self.entries.get(index.id),
            Some(AllocatorEntry::Occupied(generation)) if *generation == index.generation
        )
    }
}
