use std::cell::{Ref, RefCell, RefMut};

pub trait Component: 'static {}
impl<T: 'static> Component for T {}

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

    pub fn get(&self, index: usize) -> Option<Ref<T>> {
        let Some(ref component) = self.components[index] else {
            return None;
        };

        Some(Ref::map(component.borrow(), |component| component))
    }

    pub fn get_mut(&self, index: usize) -> Option<RefMut<T>> {
        let Some(ref component) = self.components[index] else {
            return None;
        };

        Some(RefMut::map(component.borrow_mut(), |component| component))
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
        self.components.push(None);
    }

    fn swap_remove(&mut self, index: usize) {
        self.components.swap_remove(index);
    }
}
