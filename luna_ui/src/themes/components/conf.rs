use toml_edit::Item;

use crate::error::Error;
use crate::components::label::LabelProp;

#[derive(Debug, Clone, Default)]
pub struct ComponentsConf {
    pub label: LabelProp,
}

impl TryFrom<&Item> for ComponentsConf {
    type Error = Error;

    fn try_from(value: &Item) -> Result<Self, Self::Error> {
        todo!()
    }
}
