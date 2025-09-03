use makepad_widgets::*;

use crate::{prop::Position, shader::draw_view::DrawView};


live_design! {
    use link::shaders::*;

    DrawPopup = {{DrawPopup}}{}
}

#[derive(Live, LiveRegister, LiveHook)]
#[repr(C)]
pub struct DrawPopup {
    #[deref]
    pub deref_draw: DrawView,
    #[live]
    pub position: Position,
    #[live(0.6)]
    pub opacity: f32,
    #[live(0.4)]
    pub proportion: f32,
    /// The angle offset of the popup, usually used in tooltips to get the angle center when painting
    #[live]
    pub angle_offset: f32,
}
