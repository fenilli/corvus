use std::{
    any::TypeId,
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Entity {
    id: usize,
    generation: u32,
}

trait ComponentVec {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
    fn add(&mut self, entity: usize, component: Box<dyn std::any::Any>);
    fn remove(&mut self, entity: usize);
}

struct DenseComponentVec<T> {
    dense: Vec<T>,
    entity_to_component: HashMap<usize, usize>,
    component_to_entity: Vec<usize>,
}

impl<T: 'static> ComponentVec for DenseComponentVec<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self as &dyn std::any::Any
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self as &mut dyn std::any::Any
    }

    fn add(&mut self, entity: usize, component: Box<dyn std::any::Any>) {
        let component = *component.downcast::<T>().unwrap();

        if let Some(&index) = self.entity_to_component.get(&entity) {
            self.dense[index] = component;
        } else {
            let index = self.dense.len();
            self.dense.push(component);
            self.entity_to_component.insert(entity, index);
            self.component_to_entity.push(entity);
        }
    }

    fn remove(&mut self, entity: usize) {
        if let Some(&index) = self.entity_to_component.get(&entity) {
            let last_index = self.dense.len() - 1;
            self.dense.swap(index, last_index);

            let swapped_entity = self.component_to_entity[last_index];
            self.entity_to_component.insert(swapped_entity, index);
            self.component_to_entity[index] = swapped_entity;

            self.dense.pop();
            self.entity_to_component.remove(&entity);
            self.component_to_entity.pop();
        }
    }
}

pub struct ECS {
    generations: Vec<u32>,
    free_list: Vec<usize>,

    component_map: HashMap<TypeId, RefCell<Box<dyn ComponentVec>>>,
}

impl ECS {
    pub fn new() -> Self {
        Self {
            generations: Vec::new(),
            free_list: Vec::new(),

            component_map: HashMap::new(),
        }
    }

    pub fn invalid_entity(&self, entity: Entity) -> bool {
        entity.id >= self.generations.len() || self.generations[entity.id] != entity.generation
    }

    pub fn create_entity(&mut self) -> Entity {
        let id = if let Some(free_id) = self.free_list.pop() {
            free_id
        } else {
            self.generations.push(0);
            self.generations.len() - 1
        };

        Entity {
            id,
            generation: self.generations[id],
        }
    }

    pub fn destroy_entity(&mut self, entity: Entity) {
        if self.invalid_entity(entity) {
            return;
        }

        self.generations[entity.id] += 1;
        self.free_list.push(entity.id);

        for (_, dyn_component_vec) in self.component_map.iter_mut() {
            dyn_component_vec.borrow_mut().remove(entity.id);
        }
    }

    pub fn set_component<T: 'static>(&mut self, entity: Entity, component: T) {
        if self.invalid_entity(entity) {
            return;
        }

        // Update
        if let Some(dyn_component_vec) = self.component_map.get_mut(&TypeId::of::<T>()) {
            if let Some(component_vec) = dyn_component_vec
                .borrow_mut()
                .as_any_mut()
                .downcast_mut::<DenseComponentVec<T>>()
            {
                component_vec.add(entity.id, Box::new(component));

                return;
            }
        }

        // Create
        let mut dense_component_vec: DenseComponentVec<T> = DenseComponentVec {
            dense: Vec::new(),
            entity_to_component: HashMap::new(),
            component_to_entity: Vec::new(),
        };

        dense_component_vec.add(entity.id, Box::new(component));
        self.component_map.insert(
            TypeId::of::<T>(),
            RefCell::new(Box::new(dense_component_vec)),
        );
    }

    pub fn get_components<T: 'static>(&self) -> Option<Ref<Vec<T>>> {
        if let Some(dyn_component_vec) = self.component_map.get(&TypeId::of::<T>()) {
            return Some(Ref::map(dyn_component_vec.borrow(), |component_vec| {
                &component_vec
                    .as_any()
                    .downcast_ref::<DenseComponentVec<T>>()
                    .unwrap()
                    .dense
            }));
        }

        None
    }

    pub fn get_components_mut<T: 'static>(&self) -> Option<RefMut<Vec<T>>> {
        if let Some(dyn_component_vec) = self.component_map.get(&TypeId::of::<T>()) {
            return Some(RefMut::map(
                dyn_component_vec.borrow_mut(),
                |component_vec| {
                    &mut component_vec
                        .as_any_mut()
                        .downcast_mut::<DenseComponentVec<T>>()
                        .unwrap()
                        .dense
                },
            ));
        }

        None
    }
}
