use std::{
    hash::{Hash, Hasher},
    marker::PhantomData,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HandleId(String);

impl HandleId {
    pub fn new(identifier: &str) -> Self {
        Self(identifier.to_string())
    }

    pub fn id(&self) -> &str {
        &self.0
    }
}

#[derive(Debug)]
pub struct Handle<T> {
    id: HandleId,
    _marker: PhantomData<T>,
}

impl<T> Handle<T> {
    pub fn new(id: HandleId) -> Self {
        Self {
            id,
            _marker: PhantomData,
        }
    }

    pub fn id(&self) -> HandleId {
        self.id.clone()
    }
}

impl<T> PartialEq for Handle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<T> Eq for Handle<T> {}

impl<T> Hash for Handle<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl<T> Clone for Handle<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            _marker: PhantomData,
        }
    }
}
