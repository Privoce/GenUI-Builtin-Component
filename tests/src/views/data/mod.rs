use makepad_widgets::Cx;

pub mod tag;


pub fn register(cx: &mut Cx) {
    tag::live_design(cx);
}