use makepad_widgets::*;

pub mod components;
pub mod shader;
pub mod themes;
pub mod styles;
pub mod error;

pub fn live_design(cx: &mut Cx) {
    cx.link(live_id!(basic_luna_theme), live_id!(luna_theme));
    // [components] ------------------------------------------------------
    components::live_design(cx);
    components::components_register(cx);
    // [themes] ----------------------------------------------------------
    themes::sheet::live_design(cx);
}
