use crate::prop::traits::ToTomlValue;

impl ToTomlValue for String {
    fn to_toml_value(&self) -> toml_edit::Value {
        toml_edit::Value::String(toml_edit::Formatted::new(self.to_string()))
    }
}