use makepad_widgets::Cx;

pub mod tag;
pub mod collapse;

pub fn register(cx: &mut Cx) {
    tag::live_design(cx);
    collapse::live_design(cx);
}