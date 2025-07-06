/// # Component enum
/// This macro generates an enum `LComponent` that can hold multiple component types.
/// ## Usage
/// ```rust
/// component! {
///    Label => LLabel,
/// }
/// ```
/// ## Code Generation
/// ```
/// enum LComponent<'c> {
///     Label(&'c mut LLabel),
///     View(&'c mut LView),
///     ...
/// }
///
/// impl <'c> From<&'c mut LLabel> for LComponent<'c> {
///     fn from(component: &'c mut LLabel) -> Self {
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