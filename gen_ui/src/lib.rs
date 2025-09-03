use std::path::Path;

use makepad_widgets::*;
use crate::themes::conf::Conf;

pub mod components;
pub mod error;
pub mod macros;
pub mod prop;
pub mod shader;
pub mod themes;
pub mod utils;

pub fn live_design<P>(cx: &mut Cx, path: Option<P>) where P: AsRef<Path>{
    cx.link(live_id!(basic_genui_theme), live_id!(genui_theme));
    // cx.set_global(Conf::default());
    let conf = Conf::load(path);
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
    tabbar: bool,
    tabbar_item: bool,
    tag: bool,
    link: bool,
    menu_item: bool,
    collapse: bool,
    sub_menu: bool,
    menu: bool,
}
