use toml_edit::Item;

use crate::error::Error;

#[derive(Clone, Debug, Default)]
pub struct ContainerConf {

}


impl TryFrom<&Item> for ContainerConf {
    type Error = Error;

    fn try_from(value: &Item) -> Result<Self, Self::Error> {
        todo!()
    }
}