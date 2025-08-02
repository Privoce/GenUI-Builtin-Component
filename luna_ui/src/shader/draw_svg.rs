use makepad_widgets::*;

use crate::components::svg::{SvgPartProp, SvgState};

live_design! {
    DrawSvg = {{DrawSvg}} {

    }
}

#[derive(Live, LiveRegister, LiveHook)]
#[repr(C)]
pub struct DrawSvg {
    #[deref]
    pub draw_super: DrawIcon,
    #[live]
    pub hover: f32,
    #[live]
    pub pressed: f32,
}

impl DrawSvg {
    pub fn current_state(&self) -> SvgState {
        if self.pressed == 1.0 {
            SvgState::Pressed
        } else {
            if self.hover == 1.0 {
                SvgState::Hover
            } else {
                SvgState::Basic
            }
        }
    }
    pub fn merge(&mut self, prop: &SvgPartProp) {
       self.color = prop.color;
    }
    pub fn state_basic(&mut self) {
        if self.hover != 0.0 || self.pressed != 0.0 {
            self.hover = 0.0;
            self.pressed = 0.0;
        }
    }
    pub fn state_hover(&mut self) {
        if self.hover != 1.0 {
            self.hover = 1.0;
            self.pressed = 0.0;
        }
    }
    pub fn state_pressed(&mut self) {
        if self.pressed != 1.0 {
            self.pressed = 1.0;
            self.hover = 0.0;
        }
    }
}