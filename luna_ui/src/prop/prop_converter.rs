use makepad_widgets::{vec2, vec3, vec4, LiveId, LiveValue, Vec2, Vec3, Vec4};

use crate::prop::manuel::{
    ALIGN, BORDER_RADIUS, BOTTOM, LEFT, MARGIN, PADDING, RIGHT, TOP, W, X, Y, Z,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VecValue {
    X,
    Y,
    Z,
    W,
}

impl VecValue {
    /// 创建出一个LiveValue
    pub fn create(&self, key: &FatherKey, value: f32) -> LiveValue {
        match key {
            FatherKey::Margin | FatherKey::Padding | FatherKey::BorderRadius => {
                LiveValue::Vec4(self.to_vec4(value))
            }
            FatherKey::Align => LiveValue::Vec2(self.to_vec2(value)),
            FatherKey::Other(_) => LiveValue::Vec4(self.to_vec4(value)),
        }
    }
    pub fn to_vec2(&self, value: f32) -> Vec2 {
        match self {
            VecValue::X => vec2(value, 0.0),
            VecValue::Y => vec2(0.0, value),
            _ => panic!("Invalid VecValue for Vec2: {:?}", self),
        }
    }
    #[allow(unused)]
    pub fn to_vec3(&self, value: f32) -> Vec3 {
        match self {
            VecValue::X => vec3(value, 0.0, 0.0),
            VecValue::Y => vec3(0.0, value, 0.0),
            VecValue::Z => vec3(0.0, 0.0, value),
            _ => panic!("Invalid VecValue for Vec3: {:?}", self),
        }
    }

    pub fn to_vec4(&self, value: f32) -> Vec4 {
        match self {
            VecValue::X => vec4(value, 0.0, 0.0, 0.0),
            VecValue::Y => vec4(0.0, value, 0.0, 0.0),
            VecValue::Z => vec4(0.0, 0.0, value, 0.0),
            VecValue::W => vec4(0.0, 0.0, 0.0, value),
        }
    }

    /// 合并LiveValue
    pub fn merge(&self, vector: LiveValue, key: &FatherKey, value: f32) -> LiveValue {
        match key {
            FatherKey::Margin | FatherKey::Padding | FatherKey::BorderRadius => match vector {
                LiveValue::Vec4(mut vec4) => {
                    match self {
                        VecValue::X => vec4.x = value,
                        VecValue::Y => vec4.y = value,
                        VecValue::Z => vec4.z = value,
                        VecValue::W => vec4.w = value,
                    };
                    LiveValue::Vec4(vec4)
                }
                _ => panic!(
                    "Expected Vec4 for Margin/Padding/BorderRadius, got: {:?}",
                    vector
                ),
            },
            FatherKey::Align => match vector {
                LiveValue::Vec2(mut vec2) => {
                    match self {
                        VecValue::X => vec2.x = value,
                        VecValue::Y => vec2.y = value,
                        _ => panic!("Invalid VecValue for Align: {:?}", self),
                    };
                    LiveValue::Vec2(vec2)
                }
                LiveValue::Vec4(mut vec4) => {
                    match self {
                        VecValue::X => vec4.x = value,
                        VecValue::Y => vec4.y = value,
                        _ => {}
                    };
                    LiveValue::Vec4(vec4)
                }
                _ => panic!("Expected Vec2/Vec4 for Align, got: {:?}", vector),
            },
            FatherKey::Other(other) => match vector {
                LiveValue::Vec2(mut vec2) => {
                    match self {
                        VecValue::X => vec2.x = value,
                        VecValue::Y => vec2.y = value,
                        _ => panic!("Invalid VecValue for {}: {:?}", other, self),
                    };
                    LiveValue::Vec2(vec2)
                }
                LiveValue::Vec3(mut vec3) => {
                    match self {
                        VecValue::X => vec3.x = value,
                        VecValue::Y => vec3.y = value,
                        VecValue::Z => vec3.z = value,
                        _ => panic!("Invalid VecValue for {}: {:?}", other, self),
                    };
                    LiveValue::Vec3(vec3)
                }
                LiveValue::Vec4(mut vec4) => {
                    match self {
                        VecValue::X => vec4.x = value,
                        VecValue::Y => vec4.y = value,
                        VecValue::Z => vec4.z = value,
                        VecValue::W => vec4.w = value,
                    };
                    LiveValue::Vec4(vec4)
                }
                _ => panic!("Expected Vec2/Vec3/Vec4 for {}, got: {:?}", other, vector),
            },
        }
    }
}

#[derive(Debug, Clone)]
pub enum FatherKey {
    Margin,
    Padding,
    Align,
    BorderRadius,
    Other(String),
}

impl FatherKey {
    pub fn match_k_vec(&self, key: &str) -> VecValue {
        match self {
            FatherKey::Margin | FatherKey::Padding | FatherKey::BorderRadius => match key {
                TOP => VecValue::X,
                RIGHT => VecValue::Y,
                BOTTOM => VecValue::Z,
                LEFT => VecValue::W,
                _ => panic!("Invalid key for Margin/Padding/BorderRadius: {}", key),
            },
            FatherKey::Align => match key {
                X => VecValue::X,
                Y => VecValue::Y,
                _ => panic!("Invalid key for Align: {}", key),
            },
            FatherKey::Other(_) => match key {
                X => VecValue::X,
                Y => VecValue::Y,
                Z => VecValue::Z,
                W => VecValue::W,
                TOP => VecValue::X,
                RIGHT => VecValue::Y,
                BOTTOM => VecValue::Z,
                LEFT => VecValue::W,
                _ => panic!("Invalid key for Other: {}", key),
            },
        }
    }
}

impl From<&str> for FatherKey {
    fn from(value: &str) -> Self {
        match value {
            MARGIN => FatherKey::Margin,
            PADDING => FatherKey::Padding,
            ALIGN => FatherKey::Align,
            BORDER_RADIUS => FatherKey::BorderRadius,
            _ => FatherKey::Other(value.to_string()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PropVecConverter {
    pub key: FatherKey,
    pub value: LiveValue,
    pub vec_value: VecValue,
}

impl PropVecConverter {
    pub fn new(father_key: &str, key: LiveId, value: LiveValue) -> Self {
        let father_key = FatherKey::from(father_key);
        let vec_value = father_key.match_k_vec(&key.to_string());
        Self {
            key: father_key,
            value,
            vec_value,
        }
    }
    /// 消耗自己转化出一个LiveValue，这个LiveValue是真正可以使用的值
    pub fn value(&self, value: Option<LiveValue>) -> LiveValue {
        let v = match self.value {
            LiveValue::Float32(f) => f,
            LiveValue::Float64(f) => f as f32,
            _ => panic!(
                "Expected Float32 or Float64 for value, got: {:?}",
                self.value
            ),
        };

        if let Some(value) = value {
            self.vec_value.merge(value, &self.key, v)
        } else {
            // 说明之前没有这个值
            self.vec_value.create(&self.key, v)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert() {
        let father_key = FatherKey::from("margin");
        let vec_value = father_key.match_k_vec("top");
        dbg!(vec_value);
    }
}
