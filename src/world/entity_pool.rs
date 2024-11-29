#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Entity {
    id: usize,
    generation: u32,
}

impl Entity {
    pub fn id(&self) -> usize {
        self.id
    }
}

impl Entity {
    fn new(id: usize, generation: u32) -> Self {
        Self { id, generation }
    }
}

#[derive(Debug, Clone, Copy)]
enum PoolEntry {
    Free,
    Occupied(u32),
}

#[derive(Debug)]
pub struct EntityPool {
    entries: Vec<PoolEntry>,
    free_list: Vec<usize>,
}

impl EntityPool {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            free_list: Vec::new(),
        }
    }

    pub fn allocate(&mut self) -> Entity {
        let Some(id) = self.free_list.pop() else {
            let id = self.entries.len();
            self.entries.push(PoolEntry::Occupied(0));
            return Entity::new(id, 0);
        };

        if let PoolEntry::Occupied(generation) = self.entries[id] {
            Entity::new(id, generation)
        } else {
            panic!("Tried to reference a non-occupied entry from the free list!")
        }
    }

    pub fn deallocate(&mut self, entity: Entity) -> bool {
        let Some(entry) = self.entries.get_mut(entity.id) else {
            return false;
        };

        let PoolEntry::Occupied(generation) = entry else {
            return false;
        };

        *generation += 1;
        *entry = PoolEntry::Free;
        self.free_list.push(entity.id);

        true
    }

    pub fn is_valid(&self, entity: Entity) -> bool {
        matches!(
            self.entries.get(entity.id),
            Some(PoolEntry::Occupied(generation)) if *generation == entity.generation
        )
    }
}
