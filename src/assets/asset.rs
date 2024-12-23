pub struct Asset<T: std::any::Any> {
    id: &'static str,
    _marker: std::marker::PhantomData<T>,
}

impl<T: std::any::Any> Asset<T> {
    pub(super) fn new(id: &'static str) -> Self {
        Self {
            id,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn id(&self) -> &'static str {
        self.id
    }
}

impl<T: std::any::Any> PartialEq for Asset<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<T: std::any::Any> Eq for Asset<T> {}

impl<T: std::any::Any> std::hash::Hash for Asset<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl<T: std::any::Any> Copy for Asset<T> {}

impl<T: std::any::Any> Clone for Asset<T> {
    fn clone(&self) -> Asset<T> {
        *self
    }
}
