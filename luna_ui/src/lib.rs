use makepad_widgets::*;
use themes::Conf;

pub mod components;
pub mod error;
pub mod macros;
pub mod shader;
pub mod prop;
pub mod themes;
pub mod utils;

pub fn live_design(cx: &mut Cx) {
    cx.link(live_id!(basic_luna_theme), live_id!(luna_theme));
    cx.set_global(Conf::default());
    cx.set_global(ComponentAnInit::default());
    // [shader] ----------------------------------------------------------
    shader::shader_register(cx);
    // [themes] ----------------------------------------------------------
    themes::sheet::live_design(cx);
    // [components] ------------------------------------------------------
    
    components::components_register(cx);
    components::live_design(cx);
}

/// # Component Animation init
/// define what components should be animated on init
#[derive(Default, Debug, Clone)]
pub struct ComponentAnInit{
    button: bool,
    view: bool,
    card: bool,
}