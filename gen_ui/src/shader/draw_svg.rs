use makepad_widgets::*;

use crate::components::SvgPartProp;

live_design! {
    DrawSvg = {{DrawSvg}} {

    }
}

#[derive(Live, LiveRegister, LiveHook)]
#[repr(C)]
pub struct DrawSvg {
    #[deref]
    pub draw_super: DrawIcon,
}

impl DrawSvg {
    pub fn merge(&mut self, prop: &SvgPartProp) {
        self.color = prop.color;
    }
}
