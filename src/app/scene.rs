use super::context::AppContext;

#[allow(dead_code)]
pub trait Scene: 'static {
    fn enter(&mut self, context: &mut AppContext);

    fn fixed_update(&mut self, delta_time: f32, context: &mut AppContext);

    fn update(&mut self, delta_time: f32, context: &mut AppContext);

    fn exit(&mut self, context: &mut AppContext);
}

pub struct SceneManager {
    current_scene: Option<Box<dyn Scene>>,
    next_scene: Option<Box<dyn Scene>>,
}

#[allow(dead_code)]
impl SceneManager {
    pub fn new() -> Self {
        SceneManager {
            current_scene: None,
            next_scene: None,
        }
    }

    pub fn change<T: Scene>(&mut self, new_scene: T) {
        self.next_scene = Some(Box::new(new_scene));
    }

    pub fn process(&mut self, context: &mut AppContext) {
        let Some(mut next) = self.next_scene.take() else {
            return;
        };

        if let Some(ref mut current_scene) = self.current_scene {
            current_scene.exit(context);
        };

        next.enter(context);
        self.current_scene = Some(next);
    }

    pub fn fixed_update(&mut self, delta_time: f32, context: &mut AppContext) {
        let Some(ref mut current_scene) = self.current_scene else {
            return;
        };

        current_scene.fixed_update(delta_time, context);
    }

    pub fn update(&mut self, delta_time: f32, context: &mut AppContext) {
        let Some(ref mut current_scene) = self.current_scene else {
            return;
        };

        current_scene.update(delta_time, context);
    }
}
