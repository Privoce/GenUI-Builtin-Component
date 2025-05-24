use toml_edit::Item;

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
                "primary" => "#FFFFFFE6",
                "secondary" => "#ffffff99",
                "placeholder" => "#ffffff66",
                "disabled" => "#ffffff42",
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

        let primary = color("primary")?;
        let secondary = color("secondary")?;
        let placeholder = color("placeholder")?;
        let disabled = color("disabled")?;

        Ok(ColorFontConf {
            primary,
            secondary,
            placeholder,
            disabled,
        })
    }
}
