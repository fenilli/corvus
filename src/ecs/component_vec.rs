use std::cell::{Ref, RefCell, RefMut};

use super::world::Component;

pub struct ComponentVec<T> {
    components: RefCell<Vec<Option<T>>>,
}

impl<T: Component> ComponentVec<T> {
    pub fn new() -> Self {
        Self {
            components: RefCell::new(Vec::new()),
        }
    }

    pub fn insert(&mut self, index: usize, component: T) {
        self.components.borrow_mut()[index] = Some(component);
    }

    pub fn remove(&mut self, index: usize) {
        self.components.borrow_mut()[index] = None;
    }

    pub fn components(&self) -> Ref<Vec<Option<T>>> {
        Ref::map(self.components.borrow(), |components| components)
    }

    pub fn components_mut(&self) -> RefMut<Vec<Option<T>>> {
        RefMut::map(self.components.borrow_mut(), |components| components)
    }
}

pub trait AnyVec {
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
        self.components.borrow_mut().push(None);
    }

    fn swap_remove(&mut self, index: usize) {
        self.components.borrow_mut().swap_remove(index);
    }
}
