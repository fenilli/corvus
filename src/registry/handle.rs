#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Handle<T> {
    pub id: u64,
    _marker: std::marker::PhantomData<T>,
}

impl<T> Handle<T> {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            _marker: std::marker::PhantomData,
        }
    }
}
