use toml_edit::Item;

use crate::components::label::LabelProp;
use crate::components::view::ViewProp;
use crate::error::Error;
use crate::utils::{get_from_itable, get_from_table};

#[derive(Debug, Clone, Default)]
pub struct ComponentsConf {
    pub label: LabelProp,
    pub view: ViewProp,
}

impl TryFrom<&Item> for ComponentsConf {
    type Error = Error;

    fn try_from(value: &Item) -> Result<Self, Self::Error> {
        let table = value.as_table().ok_or(Error::ThemeStyleParse(
            "[component] should be a table".to_string(),
        ))?;

        let label = get_from_table(
            table,
            "label",
            || Ok(LabelProp::default()),
            |item| item.try_into(),
        )?;

        let view = get_from_table(
            table,
            "view",
            || Ok(ViewProp::default()),
            |item| item.try_into(),
        )?;

        Ok(ComponentsConf { label, view })
    }
}
