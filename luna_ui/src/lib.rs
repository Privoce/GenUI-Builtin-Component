use makepad_widgets::*;
use themes::Conf;

pub mod components;
pub mod error;
pub mod macros;
pub mod shader;
pub mod styles;
pub mod themes;
pub mod utils;

pub fn live_design(cx: &mut Cx) {
    cx.link(live_id!(basic_luna_theme), live_id!(luna_theme));
    cx.set_global(Conf::default());
    // [shader] ----------------------------------------------------------
    shader::shader_register(cx);
    // [themes] ----------------------------------------------------------
    themes::sheet::live_design(cx);
    // [components] ------------------------------------------------------
    
    components::components_register(cx);
    components::live_design(cx);
}
