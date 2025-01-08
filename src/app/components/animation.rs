use crate::assets::atlas::AtlasRegionId;

pub struct AnimationFrame {
    pub region_id: AtlasRegionId,
    pub duration: f32,
}

impl AnimationFrame {
    pub fn new(region_id: AtlasRegionId, duration: f32) -> Self {
        Self {
            region_id,
            duration,
        }
    }
}

pub struct Animation {
    pub frames: Vec<AnimationFrame>,
    pub looping: bool,
}

impl Animation {
    pub fn new(frames: Vec<AnimationFrame>, looping: bool) -> Self {
        Self { frames, looping }
    }

    pub fn with_duration(regions_ids: Vec<AtlasRegionId>, looping: bool, duration: f32) -> Self {
        let frames = regions_ids
            .iter()
            .map(|&region_id| AnimationFrame::new(region_id, duration))
            .collect::<Vec<AnimationFrame>>();

        Self::new(frames, looping)
    }

    pub fn get_frame(&self, frame_index: usize) -> Option<&AnimationFrame> {
        self.frames.get(frame_index)
    }
}

pub struct AnimationSet {
    pub animations: std::collections::HashMap<&'static str, Animation>,
}

impl AnimationSet {
    pub fn new() -> Self {
        Self {
            animations: std::collections::HashMap::new(),
        }
    }

    pub fn add_animation(&mut self, name: &'static str, animation: Animation) {
        self.animations.insert(name, animation);
    }

    pub fn get_animation(&self, name: &str) -> Option<&Animation> {
        self.animations.get(name)
    }
}

pub struct AnimationState {
    pub current_animation: &'static str,
    pub current_frame: usize,
    pub time: f32,
}

impl AnimationState {
    pub fn new(initial_animation: &'static str) -> Self {
        Self {
            current_animation: initial_animation,
            current_frame: 0,
            time: 0.0,
        }
    }

    pub fn change_animation(&mut self, new_animation: &'static str) {
        self.current_animation = new_animation;
        self.current_frame = 0;
        self.time = 0.0;
    }
}
