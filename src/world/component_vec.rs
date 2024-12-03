use std::{
    cell::{Ref, RefCell, RefMut},
    fmt::Debug,
};

use super::world::Component;

#[derive(Debug)]
pub struct ComponentVec<T> {
    components: Vec<Option<RefCell<T>>>,
}

impl<T: Component> ComponentVec<T> {
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
        }
    }

    pub fn insert(&mut self, index: usize, component: T) {
        self.components[index] = Some(RefCell::new(component));
    }

    pub fn remove(&mut self, index: usize) {
        self.components[index] = None;
    }

    pub fn components(&self) -> impl Iterator<Item = Option<Ref<T>>> {
        self.components.iter().map(|component| match component {
            Some(component) => Some(Ref::map(component.borrow(), |data| data)),
            None => None,
        })
    }

    pub fn components_mut(&self) -> impl Iterator<Item = Option<RefMut<T>>> {
        self.components.iter().map(|component| match component {
            Some(component) => Some(RefMut::map(component.borrow_mut(), |data| data)),
            None => None,
        })
    }
}

pub trait AnyVec: Debug {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
    fn default(&mut self);
    fn swap_remove(&mut self, index: usize);
}

impl<T: Component> AnyVec for ComponentVec<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn default(&mut self) {
        self.components.push(None);
    }

    fn swap_remove(&mut self, index: usize) {
        self.components.swap_remove(index);
    }
}
