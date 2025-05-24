use toml_edit::Item;

use crate::error::Error;

#[derive(Debug, Clone, Default)]
pub struct ComponentsConf{}


impl TryFrom<&Item> for ComponentsConf {
    type Error = Error;

    fn try_from(value: &Item) -> Result<Self, Self::Error> {
        todo!()
    }
}