/// ## Example
/// ```rust
/// active_event! {
///     active_hover_in: GButtonEvent::HoverIn |e: FingerHoverEvent| => GButtonHoverParam {e},
///     active_hover_out: GButtonEvent::HoverOut |e: FingerHoverEvent| => GButtonHoverParam {e},
///     active_focus: GButtonEvent::Focus |e: FingerDownEvent| => GButtonFocusParam {e},
///     active_focus_lost: GButtonEvent::FocusLost |e: FingerUpEvent| => GButtonFocusLostParam {e},
///     active_clicked: GButtonEvent::Clicked |e: FingerUpEvent| => GButtonClickedParam {e}
/// }
/// ```
#[macro_export]
macro_rules! active_event{
    ($($event_fn: ident : $event: path |$param: ident : $param_ty: ty| => $return_ty: expr),*) => {
        $(
            pub fn $event_fn (&mut self, cx: &mut Cx, $param: $param_ty){
                if self.event_open {
                    self.scope_path.as_ref().map(|path| {
                        cx.widget_action(
                            self.widget_uid(),
                            path,
                            $event($return_ty),
                        );
                    });
                }
            }
        )*
    };
}

/// ```
/// impl GBreadCrumbItem {
///     event_option!{
///         clicked : GBreadCrumbItemEvent => GBreadCrumbEventItemParam,
///         hover : GBreadCrumbItemEvent => GBreadCrumbEventItemParam
///     }
///     // pub fn clicked(&self, actions: &Actions) -> Option<GBreadCrumbEventItemParam> {
///     //     if let GBreadCrumbItemEvent::Clicked(e) =
///     //         actions.find_widget_action(self.widget_uid()).cast()
///     //     {
///     //         Some(e)
///     //     } else {
///     //         None
///     //     }
///     // }
///     // pub fn hover(&self, actions: &Actions) -> Option<GBreadCrumbEventItemParam> {
///     //     if let GBreadCrumbItemEvent::Hover(e) = actions.find_widget_action(self.widget_uid()).cast()
///     //     {
///     //         Some(e)
///     //     } else {
///     //         None
///     //     }
///     // }
/// }
/// ```
#[macro_export]
macro_rules! event_option {
    ($($event_fn: ident : $event: path => $return: ty),*) => {
        $(
            pub fn $event_fn(&self, actions: &Actions) -> Option<$return> {
                if !self.event_open{
                    return None;
                }

                if let $event(e) =
                    actions.find_widget_action(self.widget_uid()).cast()
                {
                    Some(e)
                } else {
                    None
                }
            }
        )*
    };
}

/// # Generate Ref Event Function
///```rust
/// impl GBreadCrumbItemRef {
///
///     ref_event_option!{
///         clicked => GBreadCrumbEventItemParam,
///         hover => GBreadCrumbEventItemParam
///     }
///     // pub fn clicked(&self, actions: &Actions) -> Option<GBreadCrumbEventItemParam> {
///     //     if let Some(c_ref) = self.borrow() {
///     //         return c_ref.clicked(actions);
///     //     }
///     //     None
///     // }
///     // pub fn hover(&self, actions: &Actions) -> Option<GBreadCrumbEventItemParam> {
///     //     if let Some(c_ref) = self.borrow() {
///     //         return c_ref.hover(actions);
///     //     }
///     //     None
///     // }
/// }
/// ```
#[macro_export]
macro_rules! event_option_ref {
    ($($event_fn: ident => $return: ty),*) => {
        $(
            pub fn $event_fn(&self, actions: &Actions) -> Option<$return> {
                if let Some(c_ref) = self.borrow() {
                    return c_ref.$event_fn(actions);
                }
                None
            }
        )*
    };
}

#[macro_export]
macro_rules! hit_finger_down {
    ($self:ident, $cx:ident, $focus_area:expr, $e:expr) => {
        if $self.grab_key_focus {
            $cx.set_key_focus($focus_area);
        }
        $self.play_animation($cx, id!(hover.pressed));
        $self.active_finger_down($cx, $e);
    };
}

#[macro_export]
macro_rules! hit_hover_in {
    ($self:ident, $cx:ident, $e:expr) => {
        $self.play_animation($cx, id!(hover.on));
        $self.active_hover_in($cx, $e);
    };
}

#[macro_export]
macro_rules! hit_hover_out {
    ($self:ident, $cx:ident, $e:expr) => {
        $self.play_animation($cx, id!(hover.off));
        $self.active_hover_out($cx, $e);
    };
}

#[macro_export]
macro_rules! hit_finger_up {
    ($self:ident, $cx:ident, $e:expr) => {
        $self.play_animation($cx, id!(hover.off));
        $self.active_finger_up($cx, $e);
    };
}
