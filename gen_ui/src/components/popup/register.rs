use makepad_widgets::Cx;

pub fn register(cx: &mut Cx) {
    crate::components::popup::live_design(cx);
    crate::components::popup::container::live_design(cx);
}