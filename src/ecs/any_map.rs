use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

pub struct AnyMap {
    pub(crate) map: HashMap<TypeId, Box<dyn Any>>,
}

impl AnyMap {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn set<T: 'static>(&mut self, t: T) {
        self.map.insert(TypeId::of::<T>(), Box::new(t));
    }

    pub fn get<T: 'static>(&self) -> Option<&T> {
        self.map
            .get(&TypeId::of::<T>())
            .and_then(|boxed| boxed.downcast_ref::<T>())
    }

    pub fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.map
            .get_mut(&TypeId::of::<T>())
            .and_then(|boxed| boxed.downcast_mut::<T>())
    }
}
