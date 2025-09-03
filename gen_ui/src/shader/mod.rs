pub mod animation;
pub mod draw_checkbox;
pub mod draw_radio;
pub mod draw_view;
pub mod draw_switch;
pub mod draw_svg;
pub mod draw_image;
pub mod draw_popup;
pub mod draw_link;

use makepad_widgets::Cx;

pub fn shader_register(cx: &mut Cx) {
    draw_view::live_design(cx);
    draw_radio::live_design(cx);
    draw_checkbox::live_design(cx);
    draw_switch::live_design(cx);
    draw_svg::live_design(cx);
    draw_image::live_design(cx);
    draw_popup::live_design(cx);
    draw_link::live_design(cx);
    animation::live_design(cx);
}
