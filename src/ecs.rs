use std::cell::{Ref, RefCell, RefMut};

trait ComponentVec {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
    fn push_none(&mut self);
}

impl<T: 'static> ComponentVec for RefCell<Vec<Option<T>>> {
    fn as_any(&self) -> &dyn std::any::Any {
        self as &dyn std::any::Any
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self as &mut dyn std::any::Any
    }

    fn push_none(&mut self) {
        self.get_mut().push(None);
    }
}

pub struct ECS {
    entities_count: usize,
    component_vecs: Vec<Box<dyn ComponentVec>>,
}

impl ECS {
    pub fn new() -> Self {
        Self {
            entities_count: 0,
            component_vecs: Vec::new(),
        }
    }

    pub fn create_entity(&mut self) -> usize {
        let entity = self.entities_count;
        for component in self.component_vecs.iter_mut() {
            component.push_none();
        }

        self.entities_count += 1;
        entity
    }

    pub fn set_component<T: 'static>(&mut self, entity: usize, component: T) {
        for component_vec in self.component_vecs.iter_mut() {
            if let Some(component_vec) = component_vec
                .as_any_mut()
                .downcast_mut::<RefCell<Vec<Option<T>>>>()
            {
                component_vec.get_mut()[entity] = Some(component);

                return;
            }
        }

        let mut new_componet_vec: Vec<Option<T>> = Vec::with_capacity(self.entities_count);
        for _ in 0..self.entities_count {
            new_componet_vec.push(None);
        }

        new_componet_vec[entity] = Some(component);
        self.component_vecs
            .push(Box::new(RefCell::new(new_componet_vec)));
    }

    pub fn get_components<T: 'static>(&self) -> Option<Ref<Vec<Option<T>>>> {
        for component_vec in self.component_vecs.iter() {
            if let Some(component_vec) = component_vec
                .as_any()
                .downcast_ref::<RefCell<Vec<Option<T>>>>()
            {
                return Some(component_vec.borrow());
            }
        }

        None
    }

    pub fn get_components_mut<T: 'static>(&self) -> Option<RefMut<Vec<Option<T>>>> {
        for component_vec in self.component_vecs.iter() {
            if let Some(component_vec) = component_vec
                .as_any()
                .downcast_ref::<RefCell<Vec<Option<T>>>>()
            {
                return Some(component_vec.borrow_mut());
            }
        }

        None
    }
}
