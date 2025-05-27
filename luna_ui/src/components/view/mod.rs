mod prop;

use makepad_widgets::*;

live_design! {
    pub LViewBase = {{LView}} {}
}

#[derive(Live, LiveRegisterWidget, WidgetRef, WidgetSet)]
pub struct LView {
    #[live]
    pub draw_view: DrawView

}