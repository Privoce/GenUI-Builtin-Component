use std::fmt::Display;

use toml_edit::{DocumentMut, Item};

use super::{components::conf::ComponentsConf, global::conf::GlobalConf, theme::conf::ThemeConf};
use crate::error::Error;
use crate::utils::get_from_doc as get;

#[derive(Debug, Clone, Default)]
pub struct Conf {
    // pub global: GlobalConf,
    pub theme: ThemeConf,
    pub components: ComponentsConf,
}

impl TryFrom<DocumentMut> for Conf {
    type Error = Error;

    fn try_from(value: DocumentMut) -> Result<Self, Self::Error> {
        // let global = get(
        //     &value,
        //     "global",
        //     || Ok(GlobalConf::default()),
        //     |item| item.try_into(),
        // )?;
        let theme = get(
            &value,
            "theme",
            || Ok(ThemeConf::default()),
            |item| item.try_into(),
        )?;
        let components = get(
            &value,
            "components",
            || Ok(ComponentsConf::default()),
            |item| item.try_into(),
        )?;

        Ok(Conf {
            // global,
            theme,
            components,
        })
    }
}

impl Conf {
    pub fn components(&self) -> &ComponentsConf {
        &self.components
    }
}

impl Display for Conf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
