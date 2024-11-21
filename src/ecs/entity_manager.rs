#[derive(Clone, Copy)]
pub struct Entity {
    id: usize,
    generation: u32,
}

impl Entity {
    pub fn id(&self) -> usize {
        self.id
    }

    pub fn generation(&self) -> u32 {
        self.generation
    }
}

struct Entry {
    is_live: bool,
    generation: u32,
}

pub struct EntityManager {
    entries: Vec<Entry>,
    free: Vec<usize>,
}

impl EntityManager {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            free: Vec::new(),
        }
    }

    pub fn allocate(&mut self) -> Entity {
        // Update
        if let Some(id) = self.free.pop() {
            let entry = &mut self.entries[id];
            entry.is_live = true;

            Entity {
                id,
                generation: entry.generation,
            }
        // Insert
        } else {
            let id = self.entries.len();
            self.entries.push(Entry {
                is_live: true,
                generation: 0,
            });

            Entity { id, generation: 0 }
        }
    }

    pub fn deallocate(&mut self, entity: Entity) -> bool {
        match self.entries.get_mut(entity.id) {
            Some(entry) if entry.is_live && entry.generation == entity.generation => {
                entry.is_live = false;
                entry.generation += 1;
                self.free.push(entity.id);

                true
            }
            _ => false,
        }
    }
}
