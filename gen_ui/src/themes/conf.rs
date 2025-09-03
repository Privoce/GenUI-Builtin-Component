use std::env::current_dir;
use std::fmt::Display;

use toml_edit::{DocumentMut, Item};

use super::{components::conf::ComponentsConf, theme::conf::ThemeConf};
use crate::error::Error;
use crate::prop::manuel::{COMPONENTS, THEME};
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
            THEME,
            || Ok(ThemeConf::default()),
            |item| item.try_into(),
        )?;
        let components = get(
            &value,
            COMPONENTS,
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

impl From<&Conf> for DocumentMut {
    fn from(value: &Conf) -> Self {
        let mut doc = DocumentMut::new();
        doc.insert(THEME, Item::Table((&value.theme).into()));
        doc.insert(COMPONENTS, Item::Table((&value.components).into()));
        doc
    }
}

impl Conf {
    pub fn components(&self) -> &ComponentsConf {
        &self.components
    }
    /// 从项目根路径加载配置文件
    /// - path:
    ///     - None时加载默认配置
    ///     - Some时加载指定路径的配置文件(根路径，不需要增加文件路径)
    pub fn load<P>(path: Option<P>) -> Result<Self, Error>
    where
        P: AsRef<std::path::Path>,
    {
        let conf_path = path
            .map_or_else(
                || current_dir().map_err(|e| Error::ThemeStyleFileLoad(e.to_string())),
                |path| Ok(path.as_ref().to_path_buf()),
            )?
            .join("genui.theme.toml");
        let content = std::fs::read_to_string(&conf_path)
            .map_err(|e| Error::ThemeStyleFileLoad(e.to_string()))?;
        let doc = content
            .parse::<DocumentMut>()
            .map_err(|e| Error::ThemeStyleParse(e.to_string()))?;
        doc.try_into()
    }
}

impl Display for Conf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(DocumentMut::from(self).to_string().as_str())
    }
}
