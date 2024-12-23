pub struct Label(pub String);

impl Label {
    pub fn new(label: String) -> Self {
        Self(label)
    }
}
