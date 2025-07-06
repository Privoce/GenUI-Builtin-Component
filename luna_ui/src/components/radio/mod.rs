mod event;
mod prop;

pub use event::*;
pub use prop::*;

use makepad_widgets::*;

use crate::shader::{draw_radio::DrawRadio, draw_view::DrawView};

live_design!{
    link luna_basic;
    use link::luna_animation_prop::*;

    pub GRadio = <GRadio> {

    }
}

#[derive(Live, WidgetRef, WidgetSet, LiveRegisterWidget)]
pub struct GRadio {
    #[live]
    pub draw_radio: DrawRadio,
    #[live]
    pub draw_text: DrawText,
    #[live]
    pub draw_container: DrawView,
}