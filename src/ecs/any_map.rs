use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

pub struct AnyMap(HashMap<TypeId, Box<dyn Any>>);

impl AnyMap {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn set<T: 'static>(&mut self, t: T) {
        self.0.insert(TypeId::of::<T>(), Box::new(t));
    }

    pub fn get<T: 'static>(&self) -> Option<&T> {
        self.0
            .get(&TypeId::of::<T>())
            .and_then(|boxed| boxed.downcast_ref::<T>())
    }

    pub fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.0
            .get_mut(&TypeId::of::<T>())
            .and_then(|boxed| boxed.downcast_mut::<T>())
    }
}
