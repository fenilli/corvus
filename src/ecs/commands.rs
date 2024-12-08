use super::World;

#[allow(dead_code)]
pub struct Commands {
    commands: Vec<Box<dyn FnOnce(&mut World) + Send + Sync>>,
}

#[allow(dead_code)]
impl Commands {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }

    pub fn schedule(&mut self, command: impl FnOnce(&mut World) + Send + Sync + 'static) {
        self.commands.push(Box::new(command));
    }

    pub fn execute(&mut self, world: &mut World) {
        for command in self.commands.drain(..) {
            (command)(world)
        }
    }
}
