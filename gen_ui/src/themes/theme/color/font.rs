use std::fmt::Display;

use toml_edit::Item;

use crate::prop::manuel::{DISABLED, PLACEHOLDER, PRIMARY, SECONDARY};

use super::Color;

#[derive(Debug, Clone)]
pub struct ColorFontConf {
    pub primary: Color,
    pub secondary: Color,
    pub placeholder: Color,
    pub disabled: Color,
}

impl Default for ColorFontConf {
    fn default() -> Self {
        fn color(c: &str) -> Color {
            Color::Hex(c.parse().unwrap())
        }

        Self {
            primary: color("#FFFFFFE6"),
            secondary: color("#ffffff99"),
            placeholder: color("#ffffff66"),
            disabled: color("#ffffff42"),
        }
    }
}

impl ColorFontConf {
    pub fn from_key(s: &str) -> Color {
        Color::Hex(
            match s {
                PRIMARY => "#FFFFFFE6",
                SECONDARY => "#ffffff99",
                PLACEHOLDER => "#ffffff66",
                DISABLED => "#ffffff42",
                _ => unreachable!("Invalid color key"),
            }
            .parse()
            .unwrap(),
        )
    }
}

impl TryFrom<&Item> for ColorFontConf {
    type Error = crate::error::Error;

    fn try_from(value: &Item) -> Result<Self, Self::Error> {
        let inline_table = value
            .as_inline_table()
            .ok_or(crate::error::Error::ThemeStyleParse(
                "[theme.font] configuration should be an inline table".to_string(),
            ))?;

        let color = |key: &str| -> Result<Color, crate::error::Error> {
            inline_table
                .get(key)
                .map_or_else(|| Ok(ColorFontConf::from_key(key)), |s| s.try_into())
        };

        let primary = color(PRIMARY)?;
        let secondary = color(SECONDARY)?;
        let placeholder = color(PLACEHOLDER)?;
        let disabled = color(DISABLED)?;

        Ok(ColorFontConf {
            primary,
            secondary,
            placeholder,
            disabled,
        })
    }
}

impl From<&ColorFontConf> for Item {
    fn from(value: &ColorFontConf) -> Self {
        let mut inline_table = toml_edit::InlineTable::new();
        inline_table.insert(PRIMARY, (value.primary).into());
        inline_table.insert(SECONDARY, (value.secondary).into());
        inline_table.insert(PLACEHOLDER, (value.placeholder).into());
        inline_table.insert(DISABLED, (value.disabled).into());
        Item::Value(inline_table.into())
    }
}

impl Display for ColorFontConf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(Item::from(self).to_string().as_str())
    }
}

#[cfg(test)]
mod tests {
    use crate::themes::ColorFontConf;

    #[test]
    fn color_font_conf_fmt() {
        let conf = ColorFontConf::default();
        dbg!(conf.to_string());
    }
}
