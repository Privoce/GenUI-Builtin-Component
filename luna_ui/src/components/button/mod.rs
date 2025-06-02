mod prop;

pub use prop::*;

use makepad_widgets::*;

use crate::{pure_after_apply,  themes::Theme};

live_design!{
    link luna_basic;

    pub LButtonBase = {{LButton}}
}

#[derive(Live, Widget)]
pub struct LButton {
    #[live]
    pub prop: ButtonProp,
    // --- visible -------------------
    #[live]
    pub visible: bool,
    #[walk]
    pub walk: Walk,
    
}

impl LiveHook for LButton {
    pure_after_apply!();
}