#[macro_export]
macro_rules! animation_open_then_redraw {
    ($self:ident, $cx:ident, $event: ident) => {
        if $self.animation_open {
            if $self.animator_handle_event($cx, $event).must_redraw() {
                $self.redraw($cx);
            }
        }
    };
}

#[macro_export]
macro_rules! play_animation {
    () => {
        fn play_animation(&mut self, cx: &mut Cx, state: &[LiveId; 2]) -> () {
            if self.animation_open {
                self.clear_animation(cx);
                self.animator_play(cx, state);
            }
        }
    };
}

/// ## This macro is used to set properties of nodes in an animation set.
/// ### Usage
/// ```rust
// set_animation!{
///     nodes: draw_button = {
///         basic_index => {
///             background_color => basic_prop.background_color
///
///         },
///         hover_index => {
///             background_color => hover_prop.background_color
///         }
///     }
/// }
/// ```
/// ### Generate
/// ```
/// if let Some(index) = basic_index {
///     if let Some(v_index) = nodes.child_by_path(
///         index,
///         &[
///             live_id!(apply).as_field(),
///             live_id!(draw_button).as_field(),
///             live_id!(background_color).as_field(),
///         ],
///     ) {
///         nodes[v_index].value = basic_prop.background_color.to_live_value();
///     }
/// }
/// ```
#[macro_export]
macro_rules! set_animation {
    ($nodes: ident : $draw: tt = {
        $(
            $basic_index: expr => {
                $(
                    $field: tt => $prop_field: expr
                ),*
            }
        ),*
    }) => {
        $(
            if let Some(index) = $basic_index {
                $(
                    if let Some(v_index) = $nodes.child_by_path(
                        index,
                        &[
                            live_id!(apply).as_field(),
                            live_id!($draw).as_field(),
                            live_id!($field).as_field(),
                        ],
                    ) {
                        $nodes[v_index].value = $prop_field.to_live_value();
                    }
                )*
            }
        )*
    };
}
