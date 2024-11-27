use makepad_widgets::{live_id, Cx, LiveId};

mod components;
pub mod error;
pub mod event;
pub mod shader;
pub mod themes;
pub mod utils;

pub use components::*;


pub fn live_design(cx: &mut Cx, theme: Option<LiveId>) {
    let theme = theme.unwrap_or(live_id!(crate::themes::style));
    cx.link(live_id!(theme), theme);
    // style --------------------------------------------------
    crate::themes::register::register(cx);
    // components ---------------------------------------------
    crate::components::register::register(cx);
    // shader -------------------------------------------------
    crate::shader::register::register(cx);
    // export all the components
    crate::components::live_design(cx);
}
