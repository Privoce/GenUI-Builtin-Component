use makepad_widgets::Cx;

pub mod basic;
pub mod form;
pub mod data;
pub mod home;
pub mod test_home;
pub mod cbox;
pub mod nav;

pub fn register(cx: &mut Cx) {
    basic::register(cx);
    form::register(cx);
    data::register(cx);
    nav::register(cx);
    cbox::live_design(cx);
    home::live_design(cx);
    test_home::live_design(cx);
}