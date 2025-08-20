use makepad_widgets::*;
use themes::Conf;

pub mod components;
pub mod error;
pub mod macros;
pub mod prop;
pub mod shader;
pub mod themes;
pub mod utils;

pub fn live_design(cx: &mut Cx) {
    cx.link(live_id!(basic_luna_theme), live_id!(luna_theme));
    // cx.set_global(Conf::default());
    let conf = Conf::load::<&str>(Some("/Users/shengyifei/projects/gen_ui/components/luna_ui"));
    let conf = if let Err(e) = &conf {
        eprintln!("Error loading theme configuration: {}", e);
        conf.unwrap_or_default()
    } else {
        conf.unwrap()
    };
    cx.set_global(conf);
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
pub struct ComponentAnInit {
    button: bool,
    view: bool,
    card: bool,
    radio: bool,
    checkbox: bool,
    svg: bool,
    image: bool,
    tabbar_item: bool,
    tag: bool,
    link: bool,
    menu_item: bool,
    collapse: bool,
}
