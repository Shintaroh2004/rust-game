#![cfg_attr(target_arch = "wasm32", allow(clippy::arc_with_non_send_sync))]

pub struct Camera{
    pub position:glam::Vec3,
    pub euler:glam::Mat4,
    pub euler_x:f32,
    pub euler_y:f32,
}

impl Camera{
    pub fn new()->Self{
        Self{
            position:glam::Vec3::new(0.0,0.0,-5.0),
            euler:glam::Mat4::from_euler(glam::EulerRot::XYZ, 0.0, 0.0, 0.0),
            euler_x:0.0,
            euler_y:0.0,
        }
    }
}