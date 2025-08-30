use makepad_widgets::Cx;

pub mod view;
pub mod label;
pub mod button;
pub mod svg;
pub mod image;
pub mod card;
pub mod divider;
pub mod link;

pub fn register(cx: &mut Cx) {
    view::live_design(cx);
    button::live_design(cx);
    label::live_design(cx);
    svg::live_design(cx);
    image::live_design(cx);
    card::live_design(cx);
    divider::live_design(cx);
    link::live_design(cx);
}