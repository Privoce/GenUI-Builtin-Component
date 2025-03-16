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
            if let Err(e) = self.render() {
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
            F: FnOnce(&mut $T) -> Result<(), Box<dyn std::error::Error>>
        {
            f(self)
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
