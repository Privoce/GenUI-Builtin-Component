use toml_edit::Item;

use crate::{error::Error, themes::Theme};

use super::controller::ControllerConf;

#[derive(Clone, Debug, Default)]
pub struct GlobalConf {
    pub theme: Theme,
    pub controller: ControllerConf,
}

impl TryFrom<&Item> for GlobalConf {
    type Error = Error;

    fn try_from(value: &Item) -> Result<Self, Self::Error> {
        let mut table = value.as_table().ok_or(Error::ThemeStyleParse(
            "[global] configuration should be a table".to_string(),
        ))?;

        let theme = table
            .get("theme")
            .map_or_else(|| Ok(Theme::default()), |item| item.try_into())?;

        Ok(GlobalConf {
            theme,
            controller: todo!(),
        })
    }
}
