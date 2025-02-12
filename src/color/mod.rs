use makepad_widgets::*;

use crate::utils::hex_to_vec4;

#[derive(Debug, Clone, Live, LiveHook)]
#[live_ignore]
pub enum Color {
    #[pick(Default::default())]
    Hex(Vec4),
    #[live(Default::default())]
    Rgb(Vec3),
    #[live(Default::default())]
    Rgba(Vec4),
}

impl Default for Color {
    fn default() -> Self {
        Color::Hex(Vec4::default())
    }
}

pub fn hex<V>(v: V) -> Color where V: Into<Vec4> {
    Color::Hex(v.into())
}

pub fn rgb<V>(v: V) -> Color where V: Into<Vec3> {
    Color::Rgb(v.into())
}

pub fn rgba<V>(v: V) -> Color where V: Into<Vec4> {
    Color::Rgba(v.into())
}

// impl From<&str> for Color {
//     fn from(value: &str) -> Self {
//         hex_to_vec4(hex)
//     }
// }