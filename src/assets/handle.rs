#[derive(Debug, Clone, Copy)]
pub struct Handle<T> {
    pub id: &'static str,
    _marker: std::marker::PhantomData<T>,
}

impl<T> Handle<T> {
    pub fn new(id: &'static str) -> Self {
        Self {
            id,
            _marker: std::marker::PhantomData,
        }
    }
}
