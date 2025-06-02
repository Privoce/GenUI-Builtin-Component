use makepad_widgets::{vec2, vec3, vec4, Vec2, Vec3, Vec4};
use toml_edit::{DocumentMut, InlineTable, Item, Table, Value};

pub fn get_from_doc<U, D, F>(doc: &DocumentMut, key: &str, default: D, f: F) -> U
where
    D: FnOnce() -> U,
    F: FnOnce(&Item) -> U,
{
    doc.get(key).map_or_else(default, f)
}

pub fn get_from_itable<U, D, F>(v: &InlineTable, key: &str, default: D, f: F) -> U
where
    D: FnOnce() -> U,
    F: FnOnce(&Value) -> U,
{
    v.get(key).map_or_else(default, f)
}

pub fn get_from_table<U, D, F>(v: &Table, key: &str, default: D, f: F) -> U
where
    D: FnOnce() -> U,
    F: FnOnce(&Item) -> U,
{
    v.get(key).map_or_else(default, f)
}


pub trait ToBool {
    /// Transform f32/f64 to bool
    fn to_bool(&self) -> bool;
}

impl ToBool for f32 {
    fn to_bool(&self) -> bool {
        *self != 0.0
    }
}

pub trait BoolToF32 {
    /// Transform bool to f32/f64
    fn to_f32(&self) -> f32;
}

impl BoolToF32 for bool {
    fn to_f32(&self) -> f32 {
        *self as u8 as f32
    }
}

pub trait FloatToVec {
    fn to_vec2(self) -> Vec2;
    fn to_vec3(self) -> Vec3;
    fn to_vec4(self) -> Vec4;
}

impl FloatToVec for f64 {
    fn to_vec2(self) -> Vec2 {
        vec2(self as f32, self as f32)
    }

    fn to_vec3(self) -> Vec3 {
        vec3(self as f32, self as f32, self as f32)
    }

    fn to_vec4(self) -> Vec4 {
        vec4(self as f32, self as f32, self as f32, self as f32)
    }
}

impl FloatToVec for f32 {
    fn to_vec2(self) -> Vec2 {
        vec2(self, self)
    }

    fn to_vec3(self) -> Vec3 {
        vec3(self, self, self)
    }

    fn to_vec4(self) -> Vec4 {
        vec4(self, self, self, self)
    }
}
