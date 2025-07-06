/// # Build Component State Enum
/// ## Usage
/// ```
/// component_state! {
///    MyComponentState {
///        Idle => IDLE,
///        Loading => LOADING,
///        Error => ERROR
///    },
///    _ => MyComponentState::Idle
/// }
/// ```
/// ## Generated Code
/// ```
/// #[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
/// pub enum MyComponentState {
///    Idle,
///    Loading,
///    Error
/// }
/// impl Display for MyComponentState {
///     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
///         f.write_str(match self {
///             MyComponentState::Idle => "IDLE",
///             MyComponentState::Loading => "LOADING",
///             MyComponentState::Error => "ERROR"
///         })
///     }
/// }
/// impl From<String> for MyComponentState {
///    fn from(value: String) -> Self {
///        match value.as_str() {
///             "IDLE" => MyComponentState::Idle,
///             "LOADING" => MyComponentState::Loading,
///             "ERROR" => MyComponentState::Error,
///             _ => MyComponentState::Idle // default case
///        }
///     }
/// }
/// impl Default for MyComponentState {
///     fn default() -> Self {
///         MyComponentState::Idle // default case
///     }
/// }
/// ```
#[macro_export]
macro_rules! component_state {
    ($enum_name: ident {
        $(
            $field: ident => $str: ident
        ),*
    }, _ => $default: expr) => {
        #[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
        pub enum $enum_name {
            $(
                $field,
            )*
        }

        impl std::fmt::Display for $enum_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(match self {
                    $(
                        $enum_name::$field => $str
                    ),*
                })
            }
        }

        impl From<String> for $enum_name {
            fn from(value: String) -> Self {
                match value.as_str() {
                    $(
                        $str => $enum_name::$field,
                    )*
                    _ => $default
                }
            }
        }

        impl Default for $enum_name {
            fn default() -> Self {
                $default
            }
        }
    };
}