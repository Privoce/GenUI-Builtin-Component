use makepad_widgets::Cx;

pub mod radio;
pub mod checkbox;
pub mod switch;

pub fn register(cx: &mut Cx) {
    radio::live_design(cx);
    checkbox::live_design(cx);
    switch::live_design(cx);
}