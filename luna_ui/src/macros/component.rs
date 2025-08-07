/// # Component enum
/// This macro generates an enum `LComponent` that can hold multiple component types.
/// ## Usage
/// ```rust
/// component! {
///    Label => GLabel,
/// }
/// ```
/// ## Code Generation
/// ```
/// enum LComponent<'c> {
///     Label(&'c mut GLabel),
///     View(&'c mut GView),
///     ...
/// }
///
/// impl <'c> From<&'c mut GLabel> for LComponent<'c> {
///     fn from(component: &'c mut GLabel) -> Self {
///         LComponent::Label(component)
///     }
/// }
/// ...
/// ```
#[macro_export]
macro_rules! component {
    ($(
        $field: ident => $component: ty
    ),*) => {
        pub enum LComponent<'c> {
            $(
                $field(&'c mut $component)
            ),*
        }

        impl<'c> LComponent<'c> {
            pub fn visible(&self) -> bool {
                match self {
                    $(LComponent::$field(c) => c.visible),*
                }
            }

            pub fn walk(&mut self, cx: &mut Cx) -> Walk {
                match self {
                    $(LComponent::$field(c) => c.walk(cx)),*
                }
            }

            pub fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
                match self {
                    $(LComponent::$field(c) => c.draw_walk(cx, scope, walk)),*
                }
            }

            pub fn switch_state_with_animation(&mut self, cx: &mut Cx, state: String) {
                match self {
                    $(LComponent::$field(c) => {
                        c.switch_state_with_animation(cx, state.into());
                    }),*
                }
            }
        }

        $(
            impl<'c> From<&'c mut $component> for LComponent<'c> {
                fn from(component: &'c mut $component) -> Self {
                    LComponent::$field(component)
                }
            }
        )*
    };
}

/// ## Component Part
/// This macro generates an enum for component parts, which can be used to identify different parts of a component.
/// ## Usage
/// ```rust
/// component_part!{
///     TabbarItemPart {
///         Icon => icon => ICON,
///         Text => text => TEXT,
///         Container => container => CONTAINER
///     }, TabbarItemState
/// }
/// ```
#[macro_export]
macro_rules! component_part {
    ($part: ident {
        $(
            $field: ident => $live_id: tt => $slot_str: ident
        ),*
    }, $state: ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum $part {
            $(
                $field
            ),*
        }

        impl Part for $part {
            type State = $state;
            fn to_live_id(&self) -> LiveId {
                match self {
                    $(
                        $part::$field => live_id!($live_id),
                    )*
                }
            }
        }

        impl std::str::FromStr for $part {
            type Err = crate::error::Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $(
                        $slot_str => Ok($part::$field),
                    )*
                    _ => Err(crate::error::Error::InvalidPart {
                        from: s.to_string(),
                        to: stringify!($part).to_string(),
                    }),
                }
            }
        }

        impl std::fmt::Display for $part {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(match self {
                    $(
                        $part::$field => $slot_str,
                    )*
                })
            }
        }
    };
}