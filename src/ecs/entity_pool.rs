use std::cell::RefCell;

#[derive(Clone, Copy, PartialEq, Eq)]
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
    entries: RefCell<Vec<PoolEntry>>,
    free_list: RefCell<Vec<usize>>,
}

impl EntityPool {
    pub fn new() -> Self {
        Self {
            entries: RefCell::new(Vec::new()),
            free_list: RefCell::new(Vec::new()),
        }
    }

    pub fn allocate(&self) -> Entity {
        let mut entries = self.entries.borrow_mut();
        let mut free_list = self.free_list.borrow_mut();

        if let Some(id) = free_list.pop() {
            if let PoolEntry::Occupied(generation) = entries[id] {
                Entity::new(id, generation)
            } else {
                panic!("Tried to reference a non-occupied entry from the free list!")
            }
        } else {
            let id = entries.len();
            entries.push(PoolEntry::Occupied(0));
            Entity::new(id, 0)
        }
    }

    pub fn deallocate(&self, entity: Entity) -> bool {
        let mut entries = self.entries.borrow_mut();
        let mut free_list = self.free_list.borrow_mut();

        if let Some(entry) = entries.get_mut(entity.id) {
            if let PoolEntry::Occupied(generation) = entry {
                *generation += 1;
                *entry = PoolEntry::Free;
                free_list.push(entity.id);

                return true;
            }
        }

        false
    }

    pub fn is_valid(&self, entity: Entity) -> bool {
        matches!(
            self.entries.borrow().get(entity.id),
            Some(PoolEntry::Occupied(generation)) if *generation == entity.generation
        )
    }
}
