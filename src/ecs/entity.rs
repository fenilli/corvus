#[derive(Eq, PartialEq, Clone, Copy)]
pub struct GenerationalIndex {
    pub(self) id: usize,
    pub(self) generation: u32,
}

struct AllocatorEntry {
    is_live: bool,
    generation: u32,
}

pub struct GenerationalIndexAllocator {
    entries: Vec<AllocatorEntry>,
    free: Vec<usize>,
}

impl GenerationalIndexAllocator {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            free: Vec::new(),
        }
    }

    pub fn allocate(&mut self) -> GenerationalIndex {
        if let Some(id) = self.free.pop() {
            let entry = &mut self.entries[id];
            entry.is_live = true;

            GenerationalIndex {
                id,
                generation: entry.generation,
            }
        } else {
            let id = self.entries.len();
            self.entries.push(AllocatorEntry {
                is_live: true,
                generation: 0,
            });

            GenerationalIndex { id, generation: 0 }
        }
    }

    pub fn deallocate(&mut self, index: GenerationalIndex) -> bool {
        if let Some(entry) = self.entries.get_mut(index.id as usize) {
            if entry.is_live && entry.generation == index.generation {
                entry.is_live = false;
                entry.generation += 1;
                self.free.push(index.id);

                return true;
            }
        }

        false
    }
}

struct VecEntry<T> {
    value: T,
    generation: u32,
}

pub struct GenerationalIndexVec<T> {
    entries: Vec<Option<VecEntry<T>>>,
}

impl<T> GenerationalIndexVec<T> {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn set(&mut self, index: GenerationalIndex, value: T) {
        if index.id >= self.entries.len() {
            self.entries.resize_with(index.id + 1, || None);
        }

        self.entries[index.id] = Some(VecEntry {
            value,
            generation: index.generation,
        });
    }

    pub fn get(&self, index: GenerationalIndex) -> Option<&T> {
        self.entries.get(index.id).and_then(|entry| {
            entry
                .as_ref()
                .filter(|entry| entry.generation == index.generation)
                .map(|entry| &entry.value)
        })
    }

    pub fn get_mut(&mut self, index: GenerationalIndex) -> Option<&mut T> {
        self.entries.get_mut(index.id).and_then(|entry| {
            entry
                .as_mut()
                .filter(|entry| entry.generation == index.generation)
                .map(|entry| &mut entry.value)
        })
    }
}
