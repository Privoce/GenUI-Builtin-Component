use makepad_widgets::*;

use crate::themes::{Color, Theme};

#[derive(Debug, Clone)]

pub struct LabelProp {
    pub theme: Theme,
    pub color: Vec4,
    pub font_size: f32,
    pub line_spacing: f32,
    pub margin: Margin,
    pub padding: Padding,
}

impl Default for LabelProp {
    fn default() -> Self {
        Self {
            theme: Default::default(),
            color: Color::Hex("#FF0000E6".parse().unwrap()).into(),
            font_size: 12.0,
            line_spacing: 1.2,
            margin: Margin {
                top: 0.0,
                right: 0.0,
                bottom: 0.0,
                left: 0.0,
            },
            padding: Padding {
                top: 0.0,
                right: 0.0,
                bottom: 0.0,
                left: 0.0,
            },
        }
    }
}
