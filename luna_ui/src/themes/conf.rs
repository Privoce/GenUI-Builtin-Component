use toml_edit::DocumentMut;

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
        let get = |key: &str| {
            value
                .get(key)
                .map_or_else(|| Ok(GlobalConf::default()), |item| item.try_into())
        };

        let global = get("global")?;
        let theme = get("theme")?;
        let components = get("components")?;

        Ok(Conf {
            global,
            theme,
            components,
        })
    }
}
