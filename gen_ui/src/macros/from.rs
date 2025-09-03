/// # try_from_toml_item! macro
/// This macro generates an implementation of `TryFrom<&toml_edit::Item>` for a
/// specified type. It extracts fields from a TOML table item, providing default
/// values if the fields are not present.
#[macro_export]
macro_rules! try_from_toml_item {
    ($ty_name: ty {
        $(
            $field: ident => $key: ident, $default: expr, $try_into: expr
        ),*
    }, $e: expr) => {
        impl TryFrom<&toml_edit::Item> for $ty_name {
            type Error = crate::error::Error;
            fn try_from(value: &toml_edit::Item) -> Result<Self, Self::Error> {
                let table = value.as_table().ok_or(Error::ThemeStyleParse(
                    $e.to_string(),
                ))?;
                $(
                    let $field = crate::utils::get_from_table(
                        table,
                        $key,
                        || Ok($default),
                        $try_into,
                    )?;
                )*
                Ok(Self {
                    $(
                        $field,
                    )*
                })
            }
        }
    };
}

#[macro_export]
macro_rules! from_prop_to_toml_ {
    () => {
        
    };
}