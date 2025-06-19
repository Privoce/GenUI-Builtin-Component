/// # Generate Area Function
/// ```
/// impl GBreadCrumbItem {
///     area!{
///         area, draw_item
///     }
///     // pub fn area(&self) -> Area {
///     //     self.draw_item.area()
///     // }
/// }
/// ```
#[macro_export]
macro_rules! area {
    ($($area_fn: ident, $prop: ident),*) => {
        $(
            pub fn $area_fn(&self) -> Area {
                self.$prop.area()
            }
        )*
    };
}
