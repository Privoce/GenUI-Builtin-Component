use toml_edit::{DocumentMut, Item};

use crate::error::Error;

use super::{components::conf::ComponentsConf, global::conf::GlobalConf, theme::conf::ThemeConf};

#[derive(Debug, Clone, Default)]
pub struct Conf {
    pub global: GlobalConf,
    pub theme: ThemeConf,
    pub components: ComponentsConf,
}

impl TryFrom<DocumentMut> for Conf {
    type Error = Error;

    fn try_from(value: DocumentMut) -> Result<Self, Self::Error> {
        fn get<U, D, F>(doc: &DocumentMut, key: &str, default: D, f: F) -> U
        where
            D: FnOnce() -> U,
            F: FnOnce(&Item) -> U,
        {
            doc.get(key).map_or_else(default, f)
        }

        let global = get(
            &value,
            "global",
            || Ok(GlobalConf::default()),
            |item| item.try_into(),
        )?;
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
            global,
            theme,
            components,
        })
    }
}
