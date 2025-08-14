use makepad_widgets::Cx;

pub mod basic;
pub mod home;
pub mod cbox;

pub fn register(cx: &mut Cx) {
    basic::register(cx);
    cbox::live_design(cx);
    home::live_design(cx);
}