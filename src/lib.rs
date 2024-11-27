use makepad_widgets::Cx;

mod components;
pub mod error;
pub mod event;
pub mod shader;
pub mod themes;
pub mod utils;

pub use components::*;


pub fn live_design(cx: &mut Cx) {
    // components ---------------------------------------------
    crate::components::register::register(cx);
    // shader -------------------------------------------------
    crate::shader::register::register(cx);
    // export all the components
    crate::components::live_design(cx);
}
