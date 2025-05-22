use std::sync::LazyLock;

use makepad_widgets::{live_id, Cx, LiveId};

mod components;
pub mod error;
pub mod event;
mod macros;
pub mod shader;
pub mod themes;
pub mod utils;

pub use components::*;

use themes::handler::ThemeHandler;



pub fn live_design(cx: &mut Cx, theme: Option<LiveId>) {
    cx.set_global(ThemeHandler::load());
    // shader -------------------------------------------------
    crate::shader::register::register(cx);
    // style --------------------------------------------------
    crate::themes::register::register(cx);
    // components ---------------------------------------------
    crate::components::register::register(cx);
    // export all the components
    crate::components::live_design(cx);
    // link the theme
    if let Some(theme) = theme {
        cx.link(live_id!(gen_theme), theme);
    }
}
