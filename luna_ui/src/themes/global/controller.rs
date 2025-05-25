use makepad_widgets::{Margin, Padding, Vec2};
use toml_edit::{InlineTable, Item, Value};

use crate::{error::Error, styles::BorderRadius, themes::TomlValueTo};

#[derive(Clone, Debug)]
pub struct ControllerConf {
    pub height: f32,
    pub border_radius: BorderRadius,
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
        fn get<U, D, F>(v: &InlineTable, key: &str, default: D, f: F) -> U
        where
            D: FnOnce() -> U,
            F: FnOnce(&Value) -> U,
        {
            v.get(key).map_or_else(default, f)
        }

        let inline_table = value.as_inline_table().ok_or(Error::ThemeStyleParse(
            "[global.controller] configuration should be a table".to_string(),
        ))?;

        let height = get(inline_table, "height", || 24.0, |item| item.to_f32(24.0));

        let border_radius = get(
            inline_table,
            "border_radius",
            || Ok(BorderRadius::new(8.0)),
            |item| item.try_into(),
        )?;

        let border_width = get(
            inline_table,
            "border_width",
            || 0.0,
            |item| item.to_f32(0.0),
        );

        let spread_radius = get(
            inline_table,
            "spread_radius",
            || 0.0,
            |item| item.to_f32(0.0),
        );

        let blur_radius = get(inline_table, "blur_radius", || 0.0, |item| item.to_f32(0.0));

        let shadow_offset = get(
            inline_table,
            "shadow_offset",
            || Vec2 { x: 0.0, y: 0.0 },
            |item| item.to_vec2(Vec2 { x: 0.0, y: 0.0 }),
        );

        let default_margin = Margin {
            left: 12.0,
            top: 8.0,
            right: 12.0,
            bottom: 8.0,
        };

        let margin = get(
            inline_table,
            "margin",
            || default_margin,
            |item| item.to_margin(default_margin),
        );

        let default_padding = Padding {
            left: 12.0,
            top: 8.0,
            right: 12.0,
            bottom: 8.0,
        };

        let padding = get(
            inline_table,
            "padding",
            || default_padding,
            |item| item.to_padding(default_padding),
        );

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
            border_radius: BorderRadius::new(8.0),
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
