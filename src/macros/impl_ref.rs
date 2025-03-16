#[macro_export]
macro_rules! ref_getter_setter {
    ($(
        $fn_get: ident, $fn_set: ident -> $value_ty: ty
    ),*) => {
        $(
            pub fn $fn_set(&self, cx: &mut Cx, value: $value_ty) -> Result<(), Box<dyn std::error::Error>>{
                if let Some(mut c_ref) = self.borrow_mut() {
                    c_ref.$fn_set(cx, value);
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
