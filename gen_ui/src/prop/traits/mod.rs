mod align;
mod bool;
mod cursor;
mod flow;
mod margin;
mod number;
mod padding;
mod size;
mod vec;
mod image_fit;
mod path;
mod area;
mod string;

use makepad_widgets::{LiveValue, Vec2, Vec3, Vec4};
pub use path::*;
use toml_edit::Value;
use crate::{error::Error, themes::Color};
pub use area::*;
pub use vec::*;

pub trait NewFrom {
    fn from_f64(uni: f64) -> Self;
    fn from_xy(x: f64, y: f64) -> Self;
    fn from_all(x: f64, y: f64, z: f64, w: f64) -> Self;
    fn from_vec4(vec4: &Vec4) -> Self
    where
        Self: Sized,
    {
        Self::from_all(vec4.x as f64, vec4.y as f64, vec4.z as f64, vec4.w as f64)
    }
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

pub trait ToCursor {
    fn from_str(s: &str) -> Self;
}

pub trait FromLiveValue {
    fn from_live_value(v: &LiveValue) -> Option<Self>
    where
        Self: Sized;
}

pub trait FromLiveColor {
    fn from_live_color(v: &LiveValue) -> Option<Vec4>
    where
        Self: Sized;
}

pub trait ToColor {
    fn to_color(self) -> Color;
    fn to_hex_string(self) -> String;
    fn from_hex(s: &str) -> Result<Self, Error>
    where
        Self: Sized;
}


pub trait ToTomlValue {
    fn to_toml_value(&self) -> Value;
}