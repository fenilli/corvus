pub struct Asset<T: std::any::Any> {
    pub index: u32,
    path: &'static str,
    _marker: std::marker::PhantomData<T>,
}

impl<T: std::any::Any> Asset<T> {
    pub(super) fn new(path: &'static str) -> Self {
        Self {
            index: 0,
            path,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T: std::any::Any> PartialEq for Asset<T> {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl<T: std::any::Any> Eq for Asset<T> {}

impl<T: std::any::Any> std::hash::Hash for Asset<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.path.hash(state);
    }
}

impl<T: std::any::Any> Copy for Asset<T> {}

impl<T: std::any::Any> Clone for Asset<T> {
    fn clone(&self) -> Asset<T> {
        *self
    }
}
