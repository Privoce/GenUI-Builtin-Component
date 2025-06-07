use super::{ToF32, ToVec, ToBool};
use makepad_widgets::{vec2, vec3, vec4, Vec2, Vec3, Vec4};

impl ToBool for f32 {
    fn to_bool(&self) -> bool {
        *self != 0.0
    }
}

impl ToF32 for bool {
    fn to_f32(&self) -> f32 {
        *self as u8 as f32
    }
}

impl ToVec for f64 {
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

impl ToVec for f32 {
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
