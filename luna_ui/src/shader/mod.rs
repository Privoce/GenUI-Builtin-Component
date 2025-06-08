pub mod draw_view;
pub mod animation;

use makepad_widgets::Cx;

pub fn shader_register(cx: &mut Cx) {
    draw_view::live_design(cx);
    animation::live_design(cx);
}