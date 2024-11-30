mod component_manager;
mod index_allocator;
mod sparse_set;

use std::cell::{Ref, RefMut};

use component_manager::ComponentManager;
use index_allocator::{Index, IndexAllocator};

pub struct World {
    index_allocator: IndexAllocator,
    component_manager: ComponentManager,
}

impl World {
    pub fn new() -> Self {
        Self {
            index_allocator: IndexAllocator::new(),
            component_manager: ComponentManager::new(),
        }
    }

    pub fn create_entity(&mut self) -> Index {
        self.index_allocator.allocate()
    }

    pub fn destroy_entity(&mut self, index: Index) {
        if self.index_allocator.deallocate(index) {
            self.component_manager.clear(index);
        }
    }

    pub fn register_component<T: 'static>(&mut self) {
        self.component_manager.register::<T>();
    }

    pub fn set_component<T: 'static>(&mut self, index: Index, component: T) {
        if !self.index_allocator.is_valid(index) {
            return;
        }

        self.component_manager.insert::<T>(index, component);
    }

    pub fn remove_component<T: 'static>(&mut self, index: Index) {
        if !self.index_allocator.is_valid(index) {
            return;
        }

        self.component_manager.remove::<T>(index);
    }

    pub fn iter_components<T: 'static>(&self) -> Option<impl Iterator<Item = (Index, Ref<T>)>> {
        self.component_manager.iter::<T>()
    }

    pub fn iter_components_mut<T: 'static>(
        &self,
    ) -> Option<impl Iterator<Item = (Index, RefMut<T>)>> {
        self.component_manager.iter_mut::<T>()
    }
}

pub trait JoinIterators<'a, A, B> {
    fn join(
        self,
        other: Option<impl Iterator<Item = (Index, B)> + 'a>,
    ) -> Option<impl Iterator<Item = (Index, (A, B))> + 'a>;
}

impl<'a, A, B, I> JoinIterators<'a, A, B> for Option<I>
where
    I: Iterator<Item = (Index, A)> + 'a,
{
    fn join(
        self,
        other: Option<impl Iterator<Item = (Index, B)> + 'a>,
    ) -> Option<impl Iterator<Item = (Index, (A, B))> + 'a> {
        let Some(iter_a) = self else { return None };

        let Some(iter_b) = other else {
            return None;
        };

        let zip = iter_a.zip(iter_b);
        Some(zip.filter_map(|((e_a, a), (_, b))| Some((e_a, (a, b)))))
    }
}
