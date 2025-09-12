/// # prop_interconvert! macro
/// This macro generates an implementation of `TryFrom<&toml_edit::Item>` for a
/// specified type. It extracts fields from a TOML table item, providing default
/// values if the fields are not present.
/// ## usage
/// ```rust
/// prop_interconvert! {
///     LabelProp {
///         basic_prop = LabelBasicStyle;
///         basic => BASIC, LabelBasicStyle::default(), |v| (v, LabelState::Basic).try_into(),
///         disabled => DISABLED, LabelBasicStyle::from_state(Theme::default(), LabelState::Disabled), |v| (v, LabelState::Disabled).try_into()
///     }, "[component.label] should be a table"
/// }
/// ```
#[macro_export]
macro_rules! prop_interconvert {
    ($ty_name: ident {
        basic_prop = $basic_prop: ident;
        $(
            $field: ident => $key: ident, $default: expr, $try_into: expr
        ),*
    }, $e: expr) => {
        #[derive(Debug, Clone, Live, LiveHook, LiveRegister, Copy)]
        #[live_ignore]
        pub struct $ty_name {
            $(
                #[live($default)]
                pub $field: $basic_prop,
            )*
        }


        impl Default for $ty_name {
            fn default() -> Self {
                Self {$($field: $default,)*}
            }
        }

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

        impl From<&$ty_name> for toml_edit::Item {
            fn from(value: &$ty_name) -> Self {
                let mut table = toml_edit::Table::new();
                $(table.insert($key, (&value.$field).into());)*
                toml_edit::Item::Table(table)
            }
        }
    };
}

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
macro_rules! from_prop_to_toml {
    ($prop_struct: ident {
        $(
            $field: ident => $key: ident
        ),*
    }) => {
        impl From<&$prop_struct> for toml_edit::Value {
            fn from(value: &$prop_struct) -> Self {
                let mut itable = toml_edit::InlineTable::new();
                $(itable.insert($key, (&value.$field).into());)*
                toml_edit::Value::InlineTable(itable)
            }
        }
    };
}

/// # basic_prop_interconvert! macro
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
/// basic_prop_interconvert! {
///     LabelBasicStyle {
///         state = LabelState;
///         {color => COLOR, |v| v.try_into()};
///         {
///             font_size: f32 => FONT_SIZE, 12.0, |v| v.to_f32(),
///             line_spacing: f32 => LINE_SPACING, 1.0, |v| v.to_f32(),
///             margin: Margin => MARGIN, Margin::from_f64(0.0), |v| v.to_margin(Margin::from_f64(0.0)),
///             padding: Padding => PADDING, Padding::from_f64(0.0), |v| v.to_padding(Padding::from_f64(0.0)),
///             flow: Flow => FLOW, Flow::RightWrap, |v| v.to_flow(),
///             height: Size => HEIGHT, Size::Fit, |v| v.to_size(),
///             width: Size => WIDTH, Size::Fit, |v| v.to_size()
///         }
///     }, "LabelBasicStyle should be a inline table"
/// }
/// ```
#[macro_export]
macro_rules! basic_prop_interconvert {
    ($prop_struct: ident {
        state = $state_ty: ident;
        $({$($color: ident => $color_key: ident, $color_try_into: expr),*})?;
        {$($field: ident : $field_ty: ident => $key: ident, $field_val: expr, $try_into: expr),*}
    }, $e: expr) => {
        #[derive(Debug, Clone, Live, LiveHook, LiveRegister, Copy)]
        #[live_ignore]
        pub struct $prop_struct {
            #[live]
            pub theme: Theme,
            $(
                $(
                    #[live]
                    pub $color: Vec4,
                )*
            )?
            $(
                #[live($field_val)]
                pub $field: $field_ty,
            )*
        }

        impl Default for $prop_struct {
            fn default() -> Self {
                Self::from_state(Theme::default(), $state_ty::default())
            }
        }

        impl TryFrom<(&toml_edit::Item, $state_ty)> for $prop_struct {
            type Error = crate::error::Error;

            fn try_from((value, state): (&toml_edit::Item, $state_ty)) -> Result<Self, Self::Error> {
                let inline_table = value.as_inline_table().ok_or(Self::Error::ThemeStyleParse(
                    $e.to_string(),
                ))?;

                (inline_table, state).try_into()
            }
        }

        impl TryFrom<(&toml_edit::Value, $state_ty)> for $prop_struct {
            type Error = crate::error::Error;

            fn try_from((value, state): (&toml_edit::Value, $state_ty)) -> Result<Self, Self::Error> {
                let inline_table = value.as_inline_table().ok_or(Self::Error::ThemeStyleParse(
                    $e.to_string(),
                ))?;

                (inline_table, state).try_into()
            }
        }
        
        #[allow(unused_variables)]
        impl TryFrom<(&toml_edit::InlineTable, $state_ty)> for $prop_struct {
            type Error = crate::error::Error;

            fn try_from((inline_table, state): (&toml_edit::InlineTable, $state_ty)) -> Result<Self, Self::Error> {
                let theme = crate::themes::Theme::default();
                let theme = crate::utils::get_from_itable(inline_table, THEME, || Ok(theme), |v| v.try_into())?;

                $(
                    let $field = $field_val;
                    let $field = crate::utils::get_from_itable(inline_table, $key, || Ok($field), $try_into)?;
                )*

                $(
                    let color = Self::state_colors(theme, state);
                    $(
                        let $color = crate::utils::get_from_itable(
                            inline_table,
                            $color_key,
                            || Ok(color.$color),
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

        impl From<&$prop_struct> for toml_edit::Value {
            fn from(value: &$prop_struct) -> Self {
                let mut inline_table = toml_edit::InlineTable::new();
                inline_table.insert(THEME, value.theme.to_toml_value());
                $(
                    inline_table.insert($key, value.$field.to_toml_value());
                )*
                $(
                    $(inline_table.insert($color_key, value.$color.to_color().into());)*
                )?
                toml_edit::Value::InlineTable(inline_table)
            }
        }
    };
}

/// # component_colors! macro
/// This macro generates a struct with specified color fields and implements
/// the `From` trait for converting from a tuple of colors to the struct.
/// ## usage
/// ```rust
/// component_colors! {
///     ButtonColors {
///         colors = (Color, Color, Color);
///         background_color, border_color, shadow_color
///     }
/// }
/// ```
#[macro_export]
macro_rules! component_colors {
    ($color: ident {
        colors = ($($color_from: ty),*);
        $($field: ident),*
    }) => {
        #[derive(Debug, Clone, Copy, Default)]
        pub struct $color {
            $(pub $field: crate::themes::Color),*
        }

        impl From<($($color_from),*)> for $color {
            fn from(( $($field),* ): ($($color_from),*) ) -> Self {
                Self {
                    $($field),*
                }
            }
        }
    };
}

#[macro_export]
macro_rules! component_color {
    ($color: ident {
        colors = ($($color_from: ty)*);
        $($field: ident)?
    }) => {
        #[derive(Debug, Clone, Copy)]
        pub struct $color {
            $(pub $field: crate::themes::Color),*
        }

        impl From<crate::themes::Color> for $color {
            fn from(color: crate::themes::Color ) -> Self {
                Self {
                    $($field: color)?
                }
            }
        }
    };
}
