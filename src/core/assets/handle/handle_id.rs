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
