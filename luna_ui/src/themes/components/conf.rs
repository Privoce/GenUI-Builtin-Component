use toml_edit::Item;

use crate::components::label::LabelProp;
use crate::components::view::ViewProp;
use crate::error::Error;

#[derive(Debug, Clone, Default)]
pub struct ComponentsConf {
    pub label: LabelProp,
    pub view: ViewProp,
}

impl TryFrom<&Item> for ComponentsConf {
    type Error = Error;

    fn try_from(value: &Item) -> Result<Self, Self::Error> {
        todo!()
    }
}
