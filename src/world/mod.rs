mod index_allocator;
mod sparse_set;

use std::{any::TypeId, collections::HashMap, fmt::Debug};

use index_allocator::{Index, IndexAllocator};
use sparse_set::SparseSet;

pub type Entity = Index;

pub trait Component: Debug + Send + Sync + 'static {}
impl<T: Debug + Send + Sync + 'static> Component for T {}

pub struct World {
    index_allocator: IndexAllocator,
    components: HashMap<TypeId, SparseSet<Box<dyn Component>>>,
}

impl Debug for World {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("World")
            .field("index_allocator", &self.index_allocator)
            .field("components", &self.components)
            .finish()
    }
}

impl World {
    pub fn new() -> Self {
        Self {
            index_allocator: IndexAllocator::new(),
            components: HashMap::new(),
        }
    }

    pub fn register_component<T: Component>(&mut self) {
        let type_id = TypeId::of::<T>();

        if self.components.contains_key(&type_id) {
            panic!(
                "Component \"{}\" was already registered!",
                std::any::type_name::<T>()
            );
        }

        self.components.insert(type_id, SparseSet::new());
    }

    pub fn unregister_component<T: Component>(&mut self) {
        let type_id = TypeId::of::<T>();

        if !self.components.contains_key(&type_id) {
            panic!(
                "Component \"{}\" was never registered!",
                std::any::type_name::<T>()
            );
        }

        self.components.remove(&type_id);
    }

    pub fn create_entity(&mut self) -> Entity {
        self.index_allocator.allocate()
    }

    pub fn set_component<T: Component>(&mut self, entity: Entity, component: T) {
        if !self.index_allocator.is_valid(entity) {
            panic!("Trying to add a component to an invalid entity")
        }

        let Some(component_set) = self.components.get_mut(&TypeId::of::<T>()) else {
            panic!(
                "Component {} is not registered!",
                std::any::type_name::<T>()
            )
        };

        component_set.insert(entity, Box::new(component));
    }
}

// mod component_manager;
// mod index_allocator;
// mod sparse_set;

// use std::cell::{Ref, RefMut};

// use component_manager::ComponentManager;
// use index_allocator::{Index, IndexAllocator};

// pub struct World {
//     index_allocator: IndexAllocator,
//     component_manager: ComponentManager,
// }

// impl World {
//     pub fn new() -> Self {
//         Self {
//             index_allocator: IndexAllocator::new(),
//             component_manager: ComponentManager::new(),
//         }
//     }

//     pub fn create_entity(&mut self) -> Index {
//         self.index_allocator.allocate()
//     }

//     pub fn destroy_entity(&mut self, index: Index) {
//         if self.index_allocator.deallocate(index) {
//             self.component_manager.clear(index);
//         }
//     }

//     pub fn register_component<T: 'static>(&mut self) {
//         self.component_manager.register::<T>();
//     }

//     pub fn set_component<T: 'static>(&mut self, index: Index, component: T) {
//         if !self.index_allocator.is_valid(index) {
//             return;
//         }

//         self.component_manager.insert::<T>(index, component);
//     }

//     pub fn remove_component<T: 'static>(&mut self, index: Index) {
//         if !self.index_allocator.is_valid(index) {
//             return;
//         }

//         self.component_manager.remove::<T>(index);
//     }

//     pub fn iter_components<T: 'static>(&self) -> Option<impl Iterator<Item = (Index, Ref<T>)>> {
//         self.component_manager.iter::<T>()
//     }

//     pub fn iter_components_mut<T: 'static>(
//         &self,
//     ) -> Option<impl Iterator<Item = (Index, RefMut<T>)>> {
//         self.component_manager.iter_mut::<T>()
//     }
// }

// pub trait JoinIterators<'a, A, B> {
//     fn join(
//         self,
//         other: Option<impl Iterator<Item = (Index, B)> + 'a>,
//     ) -> Option<impl Iterator<Item = (Index, (A, B))> + 'a>;
// }

// impl<'a, A, B, I> JoinIterators<'a, A, B> for Option<I>
// where
//     I: Iterator<Item = (Index, A)> + 'a,
// {
//     fn join(
//         self,
//         other: Option<impl Iterator<Item = (Index, B)> + 'a>,
//     ) -> Option<impl Iterator<Item = (Index, (A, B))> + 'a> {
//         let Some(iter_a) = self else { return None };

//         let Some(iter_b) = other else {
//             return None;
//         };

//         let zip = iter_a.zip(iter_b);
//         Some(zip.filter_map(|((e_a, a), (_, b))| Some((e_a, (a, b)))))
//     }
// }
