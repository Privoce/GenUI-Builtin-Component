use makepad_widgets::{live_id, Cx, LiveId};

mod components;
pub mod error;
pub mod event;
pub mod shader;
pub mod themes;
pub mod utils;
mod macros;

pub use components::*;

pub fn live_design(cx: &mut Cx, theme: Option<LiveId>) {
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
