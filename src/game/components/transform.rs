use super::{Position, Rotation, Scale};

pub struct Transform {
    pub position: Position,
    pub rotation: Rotation,
    pub scale: Scale,
}

impl Transform {
    pub fn new(position: Position, rotation: Rotation, scale: Scale) -> Self {
        Self {
            position,
            rotation,
            scale,
        }
    }
}
