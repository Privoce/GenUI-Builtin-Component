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