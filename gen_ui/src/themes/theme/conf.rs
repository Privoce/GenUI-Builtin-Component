use std::fmt::Display;

use toml_edit::{Formatted, Item, Table, Value};

use crate::error::Error;

use super::{
    color::{Color, ColorFontConf},
    Theme,
};

#[derive(Debug, Clone)]
pub struct ThemeConf {
    dark: ThemeColorItemConf,
    primary: ThemeColorItemConf,
    error: ThemeColorItemConf,
    warning: ThemeColorItemConf,
    success: ThemeColorItemConf,
    info: ThemeColorItemConf,
    font: ColorFontConf,
}

impl Default for ThemeConf {
    fn default() -> Self {
        Self {
            dark: ThemeColorItemConf::dark(),
            primary: ThemeColorItemConf::primary(),
            error: ThemeColorItemConf::error(),
            warning: ThemeColorItemConf::warning(),
            success: ThemeColorItemConf::success(),
            info: ThemeColorItemConf::info(),
            font: ColorFontConf::default(),
        }
    }
}

impl TryFrom<&Item> for ThemeConf {
    type Error = Error;

    fn try_from(value: &Item) -> Result<Self, Self::Error> {
        let table = value.as_table().ok_or(Error::ThemeStyleParse(
            "[theme] configuration should be a table".to_string(),
        ))?;

        let color =
            |theme: &str, default: ThemeColorItemConf| -> Result<ThemeColorItemConf, Error> {
                table
                    .get(theme)
                    .map_or_else(|| Ok(default), |v| v.try_into())
            };

        let dark = color("dark", ThemeColorItemConf::dark())?;
        let primary = color("primary", ThemeColorItemConf::primary())?;
        let error = color("error", ThemeColorItemConf::error())?;
        let warning = color("warning", ThemeColorItemConf::warning())?;
        let success = color("success", ThemeColorItemConf::success())?;
        let info = color("info", ThemeColorItemConf::info())?;

        let font = table
            .get("font")
            .map_or_else(|| Ok(ColorFontConf::default()), |v| v.try_into())?;

        Ok(Self {
            dark,
            primary,
            error,
            warning,
            success,
            info,
            font,
        })
    }
}

impl From<&ThemeConf> for Table {
    fn from(value: &ThemeConf) -> Self {
        let mut table = Table::new();
        table.insert("dark", (&value.dark).into());
        table.insert("primary", (&value.primary).into());
        table.insert("error", (&value.error).into());
        table.insert("warning", (&value.warning).into());
        table.insert("success", (&value.success).into());
        table.insert("info", (&value.info).into());
        table.insert("font", (&value.font).into());
        table
    }
}

impl Display for ThemeConf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(Table::from(self).to_string().as_str())
    }
}

/// # Theme Color Item Configuration
/// range: `[50, 900]` - (50, 100, 200, 300, 400, 500, 600, 700, 800, 900)
#[derive(Debug, Clone)]
pub struct ThemeColorItemConf {
    pub c_50: Color,
    pub c_100: Color,
    pub c_200: Color,
    pub c_300: Color,
    pub c_400: Color,
    pub c_500: Color,
    pub c_600: Color,
    pub c_700: Color,
    pub c_800: Color,
    pub c_900: Color,
}

impl ThemeColorItemConf {
    pub fn dark() -> Self {
        Theme::Dark.into()
    }
    pub fn primary() -> Self {
        Theme::Primary.into()
    }
    pub fn error() -> Self {
        Theme::Error.into()
    }
    pub fn warning() -> Self {
        Theme::Warning.into()
    }
    pub fn success() -> Self {
        Theme::Success.into()
    }
    pub fn info() -> Self {
        Theme::Info.into()
    }
}

impl From<Theme> for ThemeColorItemConf {
    fn from(value: Theme) -> Self {
        let [c_50, c_100, c_200, c_300, c_400, c_500, c_600, c_700, c_800, c_900] = value.colors();

        Self {
            c_50,
            c_100,
            c_200,
            c_300,
            c_400,
            c_500,
            c_600,
            c_700,
            c_800,
            c_900,
        }
    }
}

impl TryFrom<&Item> for ThemeColorItemConf {
    type Error = Error;

    fn try_from(value: &Item) -> Result<Self, Self::Error> {
        let inline_table = value.as_inline_table().ok_or(Error::ThemeStyleParse(
            "[theme.$type] configuration should be a inline table".to_string(),
        ))?;

        let get = |level: u32| -> Result<Color, Error> {
            inline_table
                .get(&format!("c_{}", level))
                .ok_or(Error::ThemeStyleParse(format!(
                    "Missing color level {} in theme configuration",
                    level
                )))?
                .try_into()
        };

        let c_50 = get(50)?;
        let c_100 = get(100)?;
        let c_200 = get(200)?;
        let c_300 = get(300)?;
        let c_400 = get(400)?;
        let c_500 = get(500)?;
        let c_600 = get(600)?;
        let c_700 = get(700)?;
        let c_800 = get(800)?;
        let c_900 = get(900)?;

        Ok(ThemeColorItemConf {
            c_50,
            c_100,
            c_200,
            c_300,
            c_400,
            c_500,
            c_600,
            c_700,
            c_800,
            c_900,
        })
    }
}

impl From<&ThemeColorItemConf> for Item {
    fn from(value: &ThemeColorItemConf) -> Self {
        let mut inline_table = toml_edit::InlineTable::new();

        for (key, v) in [
            ("c_50", value.c_50.to_string()),
            ("c_100", value.c_100.to_string()),
            ("c_200", value.c_200.to_string()),
            ("c_300", value.c_300.to_string()),
            ("c_400", value.c_400.to_string()),
            ("c_500", value.c_500.to_string()),
            ("c_600", value.c_600.to_string()),
            ("c_700", value.c_700.to_string()),
            ("c_800", value.c_800.to_string()),
            ("c_900", value.c_900.to_string()),
        ] {
            inline_table.insert(key, Value::String(Formatted::new(v)));
        }

        Item::Value(toml_edit::Value::InlineTable(inline_table))
    }
}

impl Display for ThemeColorItemConf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(Item::from(self).to_string().as_str())
    }
}

#[cfg(test)]
mod tests {
    use toml_edit::Item;

    use crate::themes::theme::conf::{ThemeColorItemConf, ThemeConf};

    #[test]
    fn theme_color_conf_to_fmt() {
        let conf = ThemeConf::default();
        dbg!(conf.to_string());
    }

    #[test]
    fn theme_color_item_conf_to_item_fmt() {
        let conf = ThemeColorItemConf::dark();
        let item: Item = (&conf).into();
        dbg!(item.to_string());
    }
}
