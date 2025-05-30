use makepad_widgets::*;

use crate::components::view::{ViewBasicProp, ViewProp};

live_design!{
    use link::shaders::*;
    DrawView = {{DrawView}}{
        uniform border_inset: vec4(0.0, 0.0, 0.0, 0.0)
        // This is the main background color of the view
        // pressed level: 1
        // hover level: 2
        // so if you pressed the view, it will be the pressed color
        fn get_background_color(self) -> vec4 {
            return mix(
                mix(
                    self.prop.basic.background_color,
                    self.prop.hover.background_color,
                    self.hover
                ),
                self.prop.pressed.background_color,
                self.pressed
            );
        }

        fn get_border_color(self) -> vec4 {
            return mix(
                mix(
                    self.prop.basic.border_color,
                    self.prop.hover.border_color,
                    self.hover
                ),
                self.prop.pressed.border_color,
                self.pressed
            );
        }

        fn get_border_width(self) -> f32 {
            return mix(
                mix(
                    self.prop.basic.border_width,
                    self.prop.hover.border_width,
                    self.hover
                ),
                self.prop.pressed.border_width,
                self.pressed
            );
        }

        fn get_border_radius

        fn get_background_visible(self) -> f32 {
            return mix(
                mix(
                    self.prop.basic.background_visible,
                    self.prop.hover.background_visible,
                    self.hover
                ),
                self.prop.pressed.background_visible,
                self.pressed
            );
        }

        fn pixel(self) -> vec4 {
            let sdf = Sdf2d::viewport(self.pos * self.rect_size);
            sdf.box_all(
                    self.border_inset.x + self.border_width,
                    self.border_inset.y + self.border_width,
                    self.rect_size.x - (self.border_inset.x + self.border_inset.z + self.border_width * 2.0),
                    self.rect_size.y - (self.border_inset.y + self.border_inset.w + self.border_width * 2.0),
                    self.border_radius.x,
                    self.border_radius.y,
                    self.border_radius.z,
                    self.border_radius.w
                )
            sdf.fill_keep(self.get_color())
            if self.border_width > 0.0 {
                sdf.stroke(self.get_border_color(), self.border_width)
            }
            return sdf.result;
        }
    }
}

#[derive(Live, LiveRegister, LiveHook)]
#[repr(C)]
pub struct DrawView{
    #[deref] pub draw_super: DrawQuad,
    #[live] pub prop: ViewProp,
    #[live] pub hover: f32,
    #[live] pub pressed: f32,
}