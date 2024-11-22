use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

use crate::ecs::entity_manager::Entity;

pub trait DynStorage: Any {
    fn remove(&mut self, entity: Entity);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl dyn DynStorage {
    pub fn downcast_ref<T: DynStorage>(&self) -> Option<&T> {
        self.as_any().downcast_ref::<T>()
    }

    pub fn downcast_mut<T: DynStorage>(&mut self) -> Option<&mut T> {
        self.as_any_mut().downcast_mut::<T>()
    }
}

struct Entry<T> {
    value: T,
    generation: u32,
}

pub struct ComponentStorage<T> {
    components: Vec<Option<Entry<T>>>,
}

impl<T: 'static> ComponentStorage<T> {
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
        }
    }

    pub fn insert(&mut self, entity: Entity, value: T) {
        let id = entity.id();

        if id >= self.components.len() {
            self.components.resize_with(id + 1, || None);
        }

        self.components[id] = Some(Entry {
            value,
            generation: entity.generation(),
        })
    }

    pub fn get(&self, entity: Entity) -> Option<&T> {
        self.components
            .get(entity.id())?
            .as_ref()
            .filter(|entry| entry.generation == entity.generation())
            .map(|entry| &entry.value)
    }

    pub fn get_mut(&mut self, entity: Entity) -> Option<&mut T> {
        self.components
            .get_mut(entity.id())?
            .as_mut()
            .filter(|entry| entry.generation == entity.generation())
            .map(|entry| &mut entry.value)
    }
}

impl<T: 'static> DynStorage for ComponentStorage<T> {
    fn remove(&mut self, entity: Entity) {
        if let Some(entry) = self.components.get_mut(entity.id()) {
            if let Some(component_entry) = entry {
                if component_entry.generation == entity.generation() {
                    *entry = None;
                }
            }
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

pub type ComponentMap = HashMap<TypeId, Box<dyn DynStorage>>;
