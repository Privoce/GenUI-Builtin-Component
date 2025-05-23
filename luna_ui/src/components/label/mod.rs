use makepad_widgets::{widget::WidgetDesignAction, *};

use crate::{shader::DrawLText, themes::Theme};

live_design! {
    link luna_basic;

    pub LLabelBase = {{LLabel}} {}
}

#[derive(Live, LiveHook, Widget)]
pub struct LLabel {
    #[live]
    pub theme: Theme,
    // --- draw ------------------
    #[redraw]
    #[live]
    pub draw_text: DrawLText,
}

impl Widget for LLabel {}
