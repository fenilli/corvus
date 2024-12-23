pub struct Transform(pub glam::Mat4);

impl Transform {
    pub fn new(translation: glam::Vec3, rotation: f32, scale: glam::Vec3) -> Self {
        Self(glam::Mat4::from_scale_rotation_translation(
            scale,
            glam::Quat::from_axis_angle(glam::Vec3::Z, rotation.to_radians()),
            translation,
        ))
    }
}
