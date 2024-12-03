#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Entity {
    pub(super) id: usize,
    pub(super) generation: u32,
}

#[derive(Debug)]
enum AllocatorEntry {
    Free(usize),
    Occupied,
}

#[derive(Debug)]
pub struct EntityAllocator {
    entities: Vec<Entity>,
    entries: Vec<(AllocatorEntry, u32)>,
    free_head: usize,
}

impl EntityAllocator {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            entries: Vec::new(),
            free_head: 0,
        }
    }

    pub fn entities(&self) -> impl Iterator<Item = Entity> + '_ {
        self.entities.iter().copied()
    }

    pub fn allocate(&mut self) -> Entity {
        let entity = match self.entries.get_mut(self.free_head) {
            // Already used Entry
            Some(entry) => match entry.0 {
                AllocatorEntry::Free(next_free) => {
                    let index = Entity {
                        id: self.free_head,
                        generation: entry.1,
                    };
                    self.free_head = next_free;
                    *entry = (AllocatorEntry::Occupied, entry.1);

                    index
                }
                AllocatorEntry::Occupied => {
                    panic!("Trying to allocate an already occupied entity index")
                }
            },
            // New Entry
            None => {
                let generation = 0;
                let id = self.entries.len();
                self.entries.push((AllocatorEntry::Occupied, generation));
                self.free_head = id + 1;
                Entity { id, generation }
            }
        };

        self.entities.push(entity);

        entity
    }

    pub fn deallocate(&mut self, entity: Entity) -> bool {
        let Some(index) = self.find_entity_index(entity) else {
            return false;
        };

        self.entities.swap_remove(index);
        let entry = &mut self.entries[index];

        match entry.0 {
            AllocatorEntry::Occupied => {
                if entry.1 != entity.generation {
                    return false;
                }

                *entry = (AllocatorEntry::Free(self.free_head), entry.1 + 1);
                self.free_head = index;

                true
            }
            AllocatorEntry::Free(_) => false,
        }
    }

    pub fn find_entity_index(&self, entity: Entity) -> Option<usize> {
        self.entities.iter().position(|&item| item == entity)
    }
}
