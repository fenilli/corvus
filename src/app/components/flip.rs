pub struct Flip {
    pub horizontal: bool,
    pub vertical: bool,
}

impl Flip {
    pub fn new(horizontal: bool, vertical: bool) -> Self {
        Self {
            horizontal,
            vertical,
        }
    }
}
