mod component_manager;
mod entity_pool;
mod sparse_set;

use std::cell::{Ref, RefMut};

use component_manager::ComponentManager;
use entity_pool::{Entity, EntityPool};

pub struct World {
    entity_pool: EntityPool,
    component_manager: ComponentManager,
}

impl World {
    pub fn new() -> Self {
        Self {
            entity_pool: EntityPool::new(),
            component_manager: ComponentManager::new(),
        }
    }

    pub fn create_entity(&mut self) -> Entity {
        self.entity_pool.allocate()
    }

    pub fn destroy_entity(&mut self, entity: Entity) {
        if self.entity_pool.deallocate(entity) {
            self.component_manager.clear(entity);
        }
    }

    pub fn register_component<T: 'static>(&mut self) {
        self.component_manager.register::<T>();
    }

    pub fn set_component<T: 'static>(&mut self, entity: Entity, component: T) {
        if !self.entity_pool.is_valid(entity) {
            return;
        }

        self.component_manager.insert::<T>(entity, component);
    }

    pub fn remove_component<T: 'static>(&mut self, entity: Entity) {
        if !self.entity_pool.is_valid(entity) {
            return;
        }

        self.component_manager.remove::<T>(entity);
    }

    pub fn iter_components<T: 'static>(&self) -> Option<impl Iterator<Item = (Entity, Ref<T>)>> {
        self.component_manager.iter::<T>()
    }

    pub fn iter_components_mut<T: 'static>(
        &self,
    ) -> Option<impl Iterator<Item = (Entity, RefMut<T>)>> {
        self.component_manager.iter_mut::<T>()
    }
}

pub trait JoinIterators<'a, A, B> {
    fn join(
        self,
        other: Option<impl Iterator<Item = (Entity, B)> + 'a>,
    ) -> Option<impl Iterator<Item = (Entity, (A, B))> + 'a>;
}

impl<'a, A, B, I> JoinIterators<'a, A, B> for Option<I>
where
    I: Iterator<Item = (Entity, A)> + 'a,
{
    fn join(
        self,
        other: Option<impl Iterator<Item = (Entity, B)> + 'a>,
    ) -> Option<impl Iterator<Item = (Entity, (A, B))> + 'a> {
        match (self, other) {
            (Some(iter_a), Some(iter_b)) => {
                Some(iter_a.zip(iter_b).filter_map(|((e_a, a), (e_b, b))| {
                    if e_a == e_b {
                        Some((e_a, (a, b)))
                    } else {
                        None
                    }
                }))
            }
            _ => None,
        }
    }
}
