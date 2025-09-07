use toml_edit::Item;

use crate::{error::Error, themes::container::ContainerConf};

use super::controller::ControllerConf;

#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct GlobalConf {
    // pub theme: Theme,
    pub controller: ControllerConf,
    pub container: ContainerConf,
}

impl TryFrom<&Item> for GlobalConf {
    type Error = Error;

    fn try_from(value: &Item) -> Result<Self, Self::Error> {
        let table = value.as_table().ok_or(Error::ThemeStyleParse(
            "[global] configuration should be a table".to_string(),
        ))?;

        // let theme = table
        //     .get("theme")
        //     .map_or_else(|| Ok(Theme::default()), |item| item.try_into())?;

        let container = table
            .get("container")
            .map_or_else(|| Ok(ContainerConf::default()), |item| item.try_into())?;

        let controller = table
            .get("controller")
            .map_or_else(|| Ok(ControllerConf::default()), |item| item.try_into())?;

        Ok(GlobalConf {
            // theme,
            controller,
            container,
        })
    }
}
