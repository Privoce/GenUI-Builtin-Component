use makepad_widgets::Cx;

pub mod view;
pub mod label;
pub mod button;
pub mod svg;

pub fn register(cx: &mut Cx) {
    view::live_design(cx);
    button::live_design(cx);
}