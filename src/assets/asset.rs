pub struct Asset<T: std::any::Any> {
    id: i32,
    path: &'static str,
    _marker: std::marker::PhantomData<T>,
}

impl<T: std::any::Any> Asset<T> {
    pub(super) fn new(id: i32, path: &'static str) -> Self {
        Self {
            id,
            path,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn id(&self) -> i32 {
        self.id
    }
}

impl<T: std::any::Any> PartialEq for Asset<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.path == other.path
    }
}

impl<T: std::any::Any> Eq for Asset<T> {}

impl<T: std::any::Any> std::hash::Hash for Asset<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.path.hash(state);
    }
}

impl<T: std::any::Any> Copy for Asset<T> {}

impl<T: std::any::Any> Clone for Asset<T> {
    fn clone(&self) -> Asset<T> {
        *self
    }
}
