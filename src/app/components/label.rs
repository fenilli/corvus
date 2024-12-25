#[derive(Debug)]
pub struct Label(pub &'static str);

impl Label {
    pub fn new(label: &'static str) -> Self {
        Self(label)
    }
}
