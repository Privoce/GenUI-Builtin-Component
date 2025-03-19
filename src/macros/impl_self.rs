/// ## do render if is visible after apply
/// we should do `render()` in :
/// - dev: `after_apply_from_doc()`
/// - release: `after_new_from_doc()`
/// ### where to use
/// this macro should use in `impl ${widget}`
#[macro_export]
macro_rules! render_after_apply {
    ($Name: expr) => {
        fn render_after_apply(&mut self, cx: &mut Cx) {
            if !self.visible {
                return;
            }
            if let Err(e) = self.render(cx) {
                error!("{} render error: {:?}", $Name, e);
            }
        }
    };
}

// /// ## setter for `live` props
// ///
// #[macro_export]
// macro_rules! setter {
//     ($T: ty) => {
//         #[allow(unused_variables)]
//         fn setter<F>(&mut self, cx: &mut Cx, f: F) -> Result<(), Box<dyn std::error::Error>>
//         where
//             F: FnOnce(&mut std::cell::RefMut<'_, $T>) -> Result<(), Box<dyn std::error::Error>>
//         {
//             f(&mut c_ref)
//         }
//     };
// }

#[macro_export]
macro_rules! setter {
    ($T:ty {$(
        $fn_name: ident ($arg: ident: $arg_ty: ty) {$code: expr }
    ),*}) => {
        // crate::setter!($T);
        #[allow(unused_variables)]
        fn setter<F>(&mut self, cx: &mut Cx, f: F) -> Result<(), Box<dyn std::error::Error>>
        where
            F: FnOnce(&mut $T, &mut Cx) -> Result<(), Box<dyn std::error::Error>>
        {
            f(self, cx)
        }

        $(
            pub fn $fn_name(&mut self, cx: &mut Cx, $arg: $arg_ty) -> Result<(), Box<dyn std::error::Error>> {
                return self.setter(cx, $code);
            }
        )*
    };
}

#[macro_export]
macro_rules! getter {
    ($T:ty {$(
        $fn_name: ident ($return_ty: ty) {$code: expr}
    ),*}) => {
        fn getter<T, F>(&self, f: F) -> T
        where
            F: Fn(&$T) -> T,
        {
            f(self)
        }

        $(
            pub fn $fn_name(&self) -> $return_ty{
                self.getter($code)
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
                if !self.event_key{
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

#[macro_export]
macro_rules! event_bool {
    ($($event_fn: ident : $event: path),*) => {
        $(
            pub fn $event_fn(&self, actions: &Actions) -> bool {
                if let $event =
                    actions.find_widget_action(self.widget_uid()).cast()
                {
                    true
                } else {
                    false
                }
            }
        )*
    };
}

/// # Generate Events Option Function (if a widget has multiple events in one action called)
/// See GFileUpload in `src/components/file_upload/mod.rs`
/// ```rust
/// pub fn after_select(&self, actions: &Actions) -> Option<Vec<PathBuf>> {
///     let mut res = None;
///     filter_widget_actions(actions, self.widget_uid()).map(|actions| {
///         actions.iter().for_each(|action| {
///             if let GFileUploadEvent::AfterSelect(e) = action.cast() {
///                 res.replace(e.clone());
///             }
///         })
///     });
///
///     res
/// }
/// ```
#[macro_export]
macro_rules! events_option {
    ($($event_fn: ident : $event: path => $return: ty),*) => {
        $(
            pub fn $event_fn(&self, actions: &Actions) -> Option<$return> {
                let mut res = None;
                filter_widget_actions(actions, self.widget_uid()).map(|actions| {
                    actions.iter().for_each(|action| {
                        if let $event(e) = action.cast() {
                            res.replace(e.clone());
                        }
                    })
                });

                res
            }
        )*
    };
}

/// # Generate Animation Function
/// ```
/// impl GBreadCrumbItemRef {
///     animatie_fn!{
///         animate_hover_on,
///         animate_hover_off,
///         animate_pressed
///     }
///     // pub fn animate_hover_on(&self, cx: &mut Cx) -> () {
///     //     self.borrow_mut().unwrap().animate_hover_on(cx);
///     // }
///     // pub fn animate_hover_off(&self, cx: &mut Cx) -> () {
///     //     self.borrow_mut().unwrap().animate_hover_off(cx);
///     // }
///     // pub fn animate_pressed(&self, cx: &mut Cx) -> () {
///     //     self.borrow_mut().unwrap().animate_pressed(cx);
///     // }
/// }
/// ```
#[macro_export]
macro_rules! animatie_fn{
    ($($an_fn: ident),*) => {
        $(
            pub fn $an_fn(&self, cx: &mut Cx) -> () {
                if let Some(mut c_ref) = self.borrow_mut() {
                    c_ref.$an_fn(cx);
                }
            }
        )*
    };
}

/// # Generate Area Function
/// ```
/// impl GBreadCrumbItem {
///     widget_area!{
///         area, draw_item
///     }
///     // pub fn area(&self) -> Area {
///     //     self.draw_item.area()
///     // }
/// }
/// ```
#[macro_export]
macro_rules! widget_area {
    ($($area_fn: ident, $prop: ident),*) => {
        $(
            pub fn $area_fn(&self) -> Area {
                self.$prop.area()
            }
        )*
    };
}

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
                if self.event_key {
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


#[macro_export]
macro_rules! set_scope_path {
    () => {
        pub fn set_scope_path(&mut self, path: &HeapLiveIdPath) {
            if self.scope_path.is_none() {
                self.scope_path.replace(path.clone());
            }
        }
    };
}

#[macro_export]
macro_rules! play_animation {
    () => {
        pub fn play_animation(&mut self, cx: &mut Cx, state: &[LiveId; 2]) {
            if self.animation_key {
                self.clear_animation(cx);
                self.animator_play(cx, state);
            }
        }
    };
}

#[macro_export]
macro_rules! check_event_scope {
    () => {
        fn check_event_scope(&self) -> Option<&HeapLiveIdPath> {
            self.event_key.then(|| self.scope_path.as_ref()).flatten()
        }
    };
}

