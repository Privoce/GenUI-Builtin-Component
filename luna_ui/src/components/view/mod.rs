mod prop;

use makepad_widgets::*;
pub use prop::*;

live_design! {
    pub LViewBase = {{LView}} {}
}

#[derive(Live, LiveRegisterWidget, WidgetRef, WidgetSet)]
pub struct LView {
    #[live]
    pub prop: ViewProp,
    #[live]
    pub draw_view: DrawView

}