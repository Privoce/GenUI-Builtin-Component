#[macro_export]
macro_rules! area_ref {
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
macro_rules! getter_setter_ref {
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