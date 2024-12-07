#[allow(dead_code)]
pub trait Scene: 'static {
    fn enter(&mut self);

    fn fixed_update(&mut self, delta_time: f32);

    fn update(&mut self, delta_time: f32);

    fn exit(&mut self);
}

pub struct SceneManager {
    current_scene: Box<dyn Scene>,
    next_scene: Option<Box<dyn Scene>>,
}

#[allow(dead_code)]
impl SceneManager {
    pub fn new<T: Scene>(scene: T) -> Self {
        let mut scene = Box::new(scene);
        scene.enter();

        SceneManager {
            current_scene: scene,
            next_scene: None,
        }
    }

    pub fn change<T: Scene>(&mut self, new_scene: T) {
        self.next_scene = Some(Box::new(new_scene));
    }

    pub fn process(&mut self) {
        let Some(mut next) = self.next_scene.take() else {
            return;
        };

        self.current_scene.exit();
        next.enter();

        self.current_scene = next;
    }

    pub fn fixed_update(&mut self, delta_time: f32) {
        self.current_scene.fixed_update(delta_time);
    }

    pub fn update(&mut self, delta_time: f32) {
        self.current_scene.update(delta_time);
    }
}
