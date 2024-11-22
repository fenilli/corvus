use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

pub trait DynStorage: Any {
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

pub struct ResourceStorage<T> {
    resource: T,
}

impl<T: 'static> ResourceStorage<T> {
    pub fn insert(resource: T) -> Self {
        Self { resource }
    }

    pub fn get(&self) -> &T {
        &self.resource
    }

    pub fn get_mut(&mut self) -> &mut T {
        &mut self.resource
    }
}

impl<T: 'static> DynStorage for ResourceStorage<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

pub type ResourceMap = HashMap<TypeId, Box<dyn DynStorage>>;
