/// # interconvert_prop_toml! macro
/// This macro generates an implementation of `TryFrom<&toml_edit::Item>` for a
/// specified type. It extracts fields from a TOML table item, providing default
/// values if the fields are not present.
#[macro_export]
macro_rules! interconvert_prop_toml {
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

        impl From<&$ty_name> for &toml_edit::Item {
            fn from(value: &$ty_name) -> Self {
                let mut table = toml_edit::Table::new();
                $(table.insert($key, (&value.$field).into());)*
                toml_edit::Item::Table(table)
            }
        }
    };
}

/// # interconvert_basic_prop_toml! macro
/// This macro generates implementations of `TryFrom` and `From` traits for a
/// specified property struct. It handles conversion from TOML items, values,
/// and inline tables, as well as conversion back to TOML items.
/// ## traits
/// - `TryFrom<(&toml_edit::Item, $state_ty)>`
/// - `TryFrom<(&toml_edit::Value, $state_ty)>`
/// - `TryFrom<(&toml_edit::InlineTable, $state_ty)>`
/// - `From<&$prop_struct> for toml_edit::Item`
/// ## attention
/// - **Theme is not need to be included** : ❌ `theme => THEME, Theme::default(), |v| v.try_into()`
/// ## usage
/// ```rust
/// interconvert_basic_prop_toml!{
///     LabelBasicProp {
///         state = LabelState;
///         colors = color;
///         color => COLOR, |v| v.try_into(),
///         {
///             font_size => FONT_SIZE, 12.0, |v| v.to_f32(),
///             line_spacing => LINE_SPACING, 1.0, |v| v.to_f32(),
///             margin => MARGIN, Margin::from_f64(0.0), |v| v.to_margin(Margin::from_f64(0.0)),
///             padding => PADDING, Padding::from_f64(0.0), |v| v.to_padding(Padding::from_f64(0.0)),
///             flow => FLOW, Flow::RightWrap, |v| v.to_flow(),
///             height => HEIGHT, Size::Fit, |v| v.to_size(),
///             width => WIDTH, Size::Fit, |v| v.to_size()
///         }
///     }, "LabelBasicProp should be a inline table"
/// }
/// ```
#[macro_export]
macro_rules! interconvert_basic_prop_toml {
    ($prop_struct: ty {
        state = $state_ty: ty;
        $(
            colors = $colors: ident;
            $($color: ident => $color_key: ident, $color_try_into: expr),*
        )?,
        {$($field: ident => $key: ident, $field_val: expr, $try_into: expr),*}
    }, $e: expr) => {
        impl TryFrom<(&toml_edit::Item, $state_ty)> for LabelBasicProp {
            type Error = crate::error::Error;

            fn try_from((value, state): (&toml_edit::Item, $state_ty)) -> Result<Self, Self::Error> {
                let inline_table = value.as_inline_table().ok_or(Self::Error::ThemeStyleParse(
                    $e.to_string(),
                ))?;

                (inline_table, state).try_into()
            }
        }

        impl TryFrom<(&toml_edit::Value, $state_ty)> for LabelBasicProp {
            type Error = crate::error::Error;

            fn try_from((value, state): (&toml_edit::Value, $state_ty)) -> Result<Self, Self::Error> {
                let inline_table = value.as_inline_table().ok_or(Self::Error::ThemeStyleParse(
                    $e.to_string(),
                ))?;

                (inline_table, state).try_into()
            }
        }

        impl TryFrom<(&toml_edit::InlineTable, $state_ty)> for LabelBasicProp {
            type Error = crate::error::Error;

            fn try_from((inline_table, state): (&toml_edit::InlineTable, $state_ty)) -> Result<Self, Self::Error> {
                let theme = crate::themes::Theme::default();
                let theme = crate::utils::get_from_itable(inline_table, THEME, || Ok(theme), |v| v.try_into())?;

                $(
                    let $field = $field_val;
                    let $field = crate::utils::get_from_itable(inline_table, $key, || Ok($field), $try_into)?;
                )*
                
                $(
                    let $colors = Self::state_colors(theme, state);
                    $(
                        let $color = crate::utils::get_from_itable(
                            inline_table,
                            $color_key,
                            || Ok($color),
                            $color_try_into,
                        )?.into();
                    )*
                )?

                Ok(Self {
                    theme,
                    $($field,)*
                    $($($color,)*)*
                })
            }
        }

        impl From<&$prop_struct> for toml_edit::Item {
            fn from(value: &$prop_struct) -> Self {
                let mut inline_table = toml_edit::InlineTable::new();
                inline_table.insert(THEME, value.theme.to_toml_value());
                $(
                    inline_table.insert($key, value.$field.to_toml_value());
                )*
                $(
                    $(inline_table.insert($color_key, value.$color.to_color().into());)*
                )?
                toml_edit::Item::Value(toml_edit::Value::InlineTable(inline_table))
            }
        }
    };
}
