pub mod animation;
pub mod draw_checkbox;
pub mod draw_radio;
pub mod draw_view;

use makepad_widgets::Cx;

pub fn shader_register(cx: &mut Cx) {
    draw_view::live_design(cx);
    draw_radio::live_design(cx);
    draw_checkbox::live_design(cx);
    animation::live_design(cx);
}
