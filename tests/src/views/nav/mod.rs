use makepad_widgets::Cx;

pub mod popover;
pub mod tooltip;
pub mod drawer;
pub mod dialog;
pub mod menu;
pub mod router;
pub mod tabbar;


pub fn register(cx: &mut Cx) {
    popover::live_design(cx);
    tooltip::live_design(cx);
    drawer::live_design(cx);
    dialog::live_design(cx);
    menu::live_design(cx);
    router::live_design(cx);
    tabbar::live_design(cx);
}