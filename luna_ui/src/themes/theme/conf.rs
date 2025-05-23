use toml_edit::Item;

use crate::error::Error;

use super::color::Color;

pub struct ThemeConf {
    dark: ThemeColorConf,
    primary: ThemeColorConf,
    error: ThemeColorConf,
    warning: ThemeColorConf,
    success: ThemeColorConf,
    info: ThemeColorConf,
}

impl TryFrom<&Item> for ThemeColorConf {
    type Error = Error;

    fn try_from(value: &Item) -> Result<Self, Self::Error> {
        todo!()
    }
}

pub struct ThemeColorConf {
    /// color for font text
    pub font: Color,
    /// the weakest color, use when disabled
    pub weakest: Color,
    pub weaker: Color,
    /// use when hover
    pub weak: Color,
    /// the normal color for the component
    pub normal: Color,
    /// use when focus or pressed
    pub deep: Color,
    pub deeper: Color,
    pub deepest: Color,
}

impl TryFrom<&Item> for ThemeColorConf {
    type Error = Error;

    fn try_from(value: &Item) -> Result<Self, Self::Error> {
        todo!()
    }
}
