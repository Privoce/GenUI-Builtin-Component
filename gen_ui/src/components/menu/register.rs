use makepad_widgets::Cx;

pub fn register(cx: &mut Cx) {
    crate::components::menu::item::live_design(cx);
    crate::components::menu::sub::live_design(cx);
    crate::components::menu::live_design(cx);
}