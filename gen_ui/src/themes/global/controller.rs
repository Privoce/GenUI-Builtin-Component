use makepad_widgets::{Margin, Padding, Vec2};
use toml_edit::Item;

use crate::{
    error::Error, prop::Radius, themes::TomlValueTo, utils::get_from_itable as get,
};

#[derive(Clone, Debug)]
pub struct ControllerConf {
    pub height: f32,
    pub border_radius: Radius,
    pub border_width: f32,
    pub spread_radius: f32,
    pub blur_radius: f32,
    pub shadow_offset: Vec2,
    pub margin: Margin,
    pub padding: Padding,
}

impl TryFrom<&Item> for ControllerConf {
    type Error = Error;

    fn try_from(value: &Item) -> Result<Self, Self::Error> {
        let inline_table = value.as_inline_table().ok_or(Error::ThemeStyleParse(
            "[global.controller] configuration should be a table".to_string(),
        ))?;

        let height = get(inline_table, "height", || Ok(24.0), |item| item.to_f32())?;

        let border_radius = get(
            inline_table,
            "border_radius",
            || Ok(Radius::new(8.0)),
            |item| item.try_into(),
        )?;

        let border_width = get(
            inline_table,
            "border_width",
            || Ok(0.0),
            |item| item.to_f32(),
        )?;

        let spread_radius = get(
            inline_table,
            "spread_radius",
            || Ok(0.0),
            |item| item.to_f32(),
        )?;

        let blur_radius = get(
            inline_table,
            "blur_radius",
            || Ok(0.0),
            |item| item.to_f32(),
        )?;

        let shadow_offset = get(
            inline_table,
            "shadow_offset",
            || Ok(Vec2 { x: 0.0, y: 0.0 }),
            |item| item.to_vec2(Vec2 { x: 0.0, y: 0.0 }),
        )?;

        let default_margin = Margin {
            left: 12.0,
            top: 8.0,
            right: 12.0,
            bottom: 8.0,
        };

        let margin = get(
            inline_table,
            "margin",
            || Ok(default_margin),
            |item| item.to_margin(default_margin),
        )?;

        let default_padding = Padding {
            left: 12.0,
            top: 8.0,
            right: 12.0,
            bottom: 8.0,
        };

        let padding = get(
            inline_table,
            "padding",
            || Ok(default_padding),
            |item| item.to_padding(default_padding),
        )?;

        Ok(ControllerConf {
            height,
            border_radius,
            border_width,
            spread_radius,
            blur_radius,
            shadow_offset,
            padding,
            margin,
        })
    }
}

impl Default for ControllerConf {
    fn default() -> Self {
        Self {
            height: 24.0,
            border_radius: Radius::new(8.0),
            border_width: 0.0,
            spread_radius: 0.0,
            blur_radius: 0.0,
            shadow_offset: Vec2 { x: 0.0, y: 0.0 },
            margin: Margin {
                left: 12.0,
                top: 8.0,
                right: 12.0,
                bottom: 8.0,
            },
            padding: Padding {
                left: 12.0,
                top: 8.0,
                right: 12.0,
                bottom: 8.0,
            },
        }
    }
}
