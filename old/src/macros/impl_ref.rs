/// ## generate getter and setter functions in `${widget}Ref`
/// This macros should be use in `${widget}Ref` impl block.
/// After `${widget}` impl use `setter!` and `getter!` to generate setter and getter functions.
/// ### example
/// ```rust
/// impl AWidgetRef{
///     ref_getter_setter!{
///        get_a, set_a -> f32,
///     }
/// }
/// ```
/// ### output
/// ```rust
/// impl AWidgetRef{
///     pub fn set_a(&self, cx: &mut Cx, value: f32) -> Result<(), Box<dyn std::error::Error>>{
///         if let Some(mut c_ref) = self.borrow_mut() {
///             c_ref.set_a(cx, value);
///         }
///         Ok(())
///     }
///     pub fn get_a(&self) -> f32{
///         if let Some(c_ref) = self.borrow() {
///             c_ref.get_a()
///         } else {
///             Default::default()
///         }
///     }
/// }
/// ```
#[macro_export]
macro_rules! ref_getter_setter {
    ($(
        $fn_get: ident, $fn_set: ident -> $value_ty: ty
    ),*) => {
        $(
            pub fn $fn_set(&self, cx: &mut Cx, value: $value_ty) -> Result<(), Box<dyn std::error::Error>>{
                if let Some(mut c_ref) = self.borrow_mut() {
                    c_ref.$fn_set(cx, value)?;
                }
                Ok(())
            }
            pub fn $fn_get(&self) -> $value_ty
            {
                if let Some(c_ref) = self.borrow() {
                    c_ref.$fn_get()
                } else {
                    Default::default()
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
macro_rules! ref_event_option {
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

///```rust
/// impl GBreadCrumbItemRef {
///     ref_event_bool!{
///         clicked
///     }
/// }
/// ```
#[macro_export]
macro_rules! ref_event_bool {
    ($($event_fn: ident),*) => {
        $(
            pub fn $event_fn(&self, actions: &Actions) -> bool {
                if let Some(c_ref) = self.borrow() {
                    return c_ref.$event_fn(actions);
                }
                false
            }
        )*
    };
}

#[macro_export]
macro_rules! ref_play_animation {
    ($($an_fn: ident : $state: expr),*) => {
        $(
            pub fn $an_fn(&self, cx: &mut Cx) -> () {
                if let Some(mut c_ref) = self.borrow_mut() {
                    c_ref.play_animation(cx, $state);
                }
            }
        )*
    };
}


/// # Generate Area Function
#[macro_export]
macro_rules! ref_area {
    () => {
        pub fn area(&self) -> Area {
            if let Some(c_ref) = self.borrow() {
                return c_ref.area();
            }
            Area::Empty
        }
    };
}

#[macro_export]
macro_rules! ref_area_ext {
    ($($area_fn: ident),*) => {
        $(
            pub fn $area_fn(&self) -> Area {
                if let Some(c_ref) = self.borrow() {
                    return c_ref.$area_fn();
                }
                Area::Empty
            }
        )*
    };
}

#[macro_export]
macro_rules! ref_redraw {
    () => {
        pub fn redraw(&self, cx: &mut Cx) -> () {
            if let Some(c_ref) = self.borrow() {
                c_ref.redraw(cx);
            }
        }
    };
}

#[macro_export]
macro_rules! ref_redraw_mut {
    () => {
        pub fn redraw(&mut self, cx: &mut Cx) -> () {
            if let Some(mut c_ref) = self.borrow_mut() {
                c_ref.redraw(cx);
            }
        }
    };
}

#[macro_export]
macro_rules! ref_animate_state {
    () => {
        pub fn animate_state(&self) -> GLabelState {
            if let Some(c_ref) = self.borrow() {
                return c_ref.animate_state();
            }
            GLabelState::None
        }
    };
}

#[macro_export]
macro_rules! ref_render {
    () => {
        pub fn render(&self, cx: &mut Cx) -> Result<(), Box<dyn std::error::Error>> {
            if let Some(mut c_ref) = self.borrow_mut() {
                c_ref.render(cx)?;
            }
            Ok(())
        }
    };
}

#[macro_export]
macro_rules! ref_actives {
    ($($event_fn: ident : $e_ty: ty),*) => {
       $(
            pub fn $event_fn(&self, cx: &mut Cx, e: $e_ty) -> () {
                self.borrow_mut().map(|mut c_ref| c_ref.$event_fn(cx, e));
            }
       )*
    };
}
