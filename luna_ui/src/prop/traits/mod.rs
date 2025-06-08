mod margin;
mod padding;
mod align;
mod vec;
mod bool;

use makepad_widgets::{Vec2, Vec3, Vec4};

pub trait NewFrom {
    fn from_f64(uni: f64) -> Self;
    fn from_xy(x: f64, y: f64) -> Self;
    fn from_all(x: f64, y: f64, z: f64, w: f64) -> Self;
}

pub trait ToBool {
    /// Transform f32/f64 to bool
    fn to_bool(&self) -> bool;
}


pub trait ToFloat {
    /// Transform bool to f32/f64
    fn to_f32(&self) -> f32;
    fn to_f64(&self) -> f64;
}


pub trait ToVec {
    fn to_vec2(self) -> Vec2;
    fn to_vec3(self) -> Vec3;
    fn to_vec4(self) -> Vec4;
}

pub trait ToU32 {
    fn to_u32(self) -> u32;
}
