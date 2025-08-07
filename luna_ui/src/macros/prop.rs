#[macro_export]
macro_rules! set_scope_path {
    () => {
        fn set_scope_path(&mut self, path: &HeapLiveIdPath) {
            if self.scope_path.is_none() {
                self.scope_path.replace(path.clone());
            }
        }
    };
}

#[macro_export]
macro_rules! setter {
    ($T:ty {$(
        $fn_name: ident ($arg: ident: $arg_ty: ty) {$code: expr }
    ),*}) => {
        // crate::setter!($T);
        #[allow(unused_variables)]
        fn setter<F>(&mut self, cx: &mut Cx, f: F) -> Result<(), crate::error::Error>
        where
            F: FnOnce(&mut $T, &mut Cx) -> Result<(), crate::error::Error>
        {
            f(self, cx)
        }

        $(
            pub fn $fn_name(&mut self, cx: &mut Cx, $arg: $arg_ty) -> Result<(), crate::error::Error> {
                return self.setter(cx, $code);
            }
        )*
    };
}

#[macro_export]
macro_rules! getter_setter_prop {
    ($(
        $fn_getter: ident, $fn_setter: ident : $prop: ident -> $v_ty: ty
    ),*) => {
        $(
            pub fn $fn_setter(&mut self, v: $v_ty) -> () {
                self.$prop = v;
            }
            pub fn $fn_getter(&self) -> $v_ty {
                self.$prop
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

#[macro_export]
macro_rules! set_index {
    () => {
        fn set_index(&mut self, index: usize) {
            self.index = index;
        }
    };
}

#[macro_export]
macro_rules! lifecycle {
    () => {
        fn lifecycle(&self) -> LifeCycle {
            self.lifecycle
        }
    };
}

#[macro_export]
macro_rules! visible {
    () => {
        fn visible(&self) -> bool {
            self.visible
        }
    };
}

/// ## generate state_colors function
/// ### usage
/// ```
/// state_colors!{
///     (bg_level, stroke_level, border_level),
///     RadioState::Basic => (200, 200, 400),
///     RadioState::Hover => (200, 200, 400),
///     RadioState::Active => (500, 200, 500),
///     RadioState::Disabled => (100, 100, 300)
/// }
/// ```
/// ### generate code
/// ```
/// fn state_colors(theme: Theme, state: Self::State) -> Self::Colors {
///     let (bg_level, stroke_level, border_level) = match state {
///         RadioState::Basic => (200, 200, 400),
///         RadioState::Hover => (200, 200, 400),
///         RadioState::Active => (500, 200, 500),
///         RadioState::Disabled => (100, 100, 300),
///     };
///
///     match theme {
///         Theme::Dark => (
///             Theme::Dark.color(bg_level),
///             Theme::Dark.color(stroke_level),
///             Theme::Dark.color(border_level),
///         ),
///         Theme::Primary => (
///             Theme::Primary.color(bg_level),
///             Theme::Primary.color(stroke_level),
///             Theme::Primary.color(border_level),
///         ),
///         ...
///     }
/// }
/// ```
#[macro_export]
macro_rules! state_colors {
    (
        ($($level: ident),*),
        $($state: path => ($($level_number: expr),*)),*
    ) => {
        fn state_colors(theme: Theme, state: Self::State) -> Self::Colors {
            let ($($level),*) = match state {
                $(
                    $state => (
                        $($level_number),*
                    ),
                )*

            };

            match theme {
                Theme::Dark => (
                    $(Theme::Dark.color($level),)*
                ),
                Theme::Primary => (
                    $(Theme::Primary.color($level),)*
                ),
                Theme::Error => (
                    $(Theme::Error.color($level),)*
                ),
                Theme::Warning => (
                    $(Theme::Warning.color($level),)*
                ),
                Theme::Success => (
                    $(Theme::Success.color($level),)*
                ),
                Theme::Info => (
                    $(Theme::Info.color($level),)*
                ),
            }
        }
    };
}

/// ## generate `get` and `get_mut` fn in `Prop` trait
/// ### usage
/// ```
/// get_get_mut!{
///     RadioState::Basic => basic,
///     RadioState::Hover => hover,
///     RadioState::Active => active,
///     RadioState::Disabled => disabled
/// }
/// ```
/// ### generate code
/// ```
/// fn get(&self, state: Self::State) -> &Self::Basic {
///     match state {
///         RadioState::Basic => &self.basic,
///         RadioState::Hover => &self.hover,
///         RadioState::Active => &self.active,
///         RadioState::Disabled => &self.disabled,
///     }
/// }
///
/// fn get_mut(&mut self, state: Self::State) -> &mut Self::Basic {
///     match state {
///         RadioState::Basic => &mut self.basic,
///         RadioState::Hover => &mut self.hover,
///         RadioState::Active => &mut self.active,
///         RadioState::Disabled => &mut self.disabled,
///     }
/// }
/// ```
#[macro_export]
macro_rules! get_get_mut {
    ($(
        $state_path: path => $state_prop: tt
    ),*) => {
        fn get(&self, state: Self::State) -> &Self::Basic {
            match state {
                $(
                    $state_path => &self.$state_prop,
                )*
            }
        }

        fn get_mut(&mut self, state: Self::State) -> &mut Self::Basic {
            match state {
                $(
                    $state_path => &mut self.$state_prop,
                )*
            }
        }
    };
}
