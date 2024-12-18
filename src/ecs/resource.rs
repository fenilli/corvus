use std::{
    any::TypeId,
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
};

pub trait Resource: 'static {}
impl<T: 'static> Resource for T {}

trait AnyResource {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

impl<T: Resource> AnyResource for T {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

pub struct Resources {
    resources: HashMap<TypeId, Box<RefCell<dyn AnyResource>>>,
}

impl Resources {
    pub fn new() -> Self {
        Self {
            resources: HashMap::new(),
        }
    }

    pub fn insert_resource<T: Resource>(&mut self, resource: T) {
        self.resources
            .insert(TypeId::of::<T>(), Box::new(RefCell::new(resource)));
    }

    pub fn remove_resource<T: Resource>(&mut self) {
        self.resources.remove(&TypeId::of::<T>());
    }

    pub fn get_resource<T: Resource>(&self) -> Option<Ref<T>> {
        let Some(cell) = self.resources.get(&TypeId::of::<T>()) else {
            return None;
        };

        Ref::filter_map(cell.borrow(), |boxed| boxed.as_any().downcast_ref::<T>()).ok()
    }

    pub fn get_resource_mut<T: Resource>(&self) -> Option<RefMut<T>> {
        let Some(cell) = self.resources.get(&TypeId::of::<T>()) else {
            return None;
        };

        RefMut::filter_map(cell.borrow_mut(), |boxed| {
            boxed.as_any_mut().downcast_mut::<T>()
        })
        .ok()
    }
}
