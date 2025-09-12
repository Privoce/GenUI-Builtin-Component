use crate::{components::ViewBasicStyle, prop::traits::ToFloat};
use makepad_widgets::*;

live_design! {
    use link::shaders::*;
    DrawView = {{DrawView}}{
        uniform border_inset: vec4(0.0, 0.0, 0.0, 0.0),
        varying rect_size2: vec2,
        varying rect_size3: vec2,
        varying rect_pos2: vec2,
        varying rect_shift: vec2,
        varying sdf_rect_pos: vec2,
        varying sdf_rect_size: vec2,

        // [getter] -------------------------------------------------------------------------------
        fn get_background_color(self) -> vec4 { return self.background_color; }
        fn get_border_color(self) -> vec4 { return self.border_color; }
        fn get_shadow_color(self) -> vec4 { return self.shadow_color; }

        // [vertex shader] ------------------------------------------------------------------------
        fn vertex(self) -> vec4 {
            // - [get minimum shadow offset] ------------------------------------------------------
            let min_offset = min(self.shadow_offset, vec2(0));
            // - [shadow spread and blur calculation] ---------------------------------------------

            let total_shadow_size = self.spread_radius + self.blur_radius;

            self.rect_size2 = self.rect_size + 2.0 * vec2(total_shadow_size);
            self.rect_size3 = self.rect_size2 + abs(self.shadow_offset);
            self.rect_pos2 = self.rect_pos - vec2(total_shadow_size) + min_offset;
            self.rect_shift = -min_offset;

            let border_width = self.border_width;
            self.sdf_rect_size = self.rect_size2 - vec2(total_shadow_size * 2.0 + border_width * 2.0);
            self.sdf_rect_pos = -min_offset + vec2(border_width + total_shadow_size);

            return self.clip_and_transform_vertex(self.rect_pos2, self.rect_size3)
        }
        // ----------------------------------------------------------------------------------------

        // [pixel shader] -------------------------------------------------------------------------
        fn pixel(self) -> vec4 {
            let sdf = Sdf2d::viewport(self.pos * self.rect_size3);
            // - [draw shadow and blur] -----------------------------------------------------------
            if sdf.shape > -1.0 {
                if self.spread_radius > 0.0 || self.blur_radius > 0.0 {
                    let shadow_offset = self.shadow_offset + self.rect_shift;
                    let total_shadow_size = self.spread_radius + self.blur_radius;
                    let shadow_lower = vec2(total_shadow_size) + shadow_offset;
                    let shadow_upper = self.rect_size + vec2(self.spread_radius * 2.0) + shadow_offset;
                    if self.border_radius.x != 0.0 || self.border_radius.y != 0.0 ||
                        self.border_radius.z != 0.0 || self.border_radius.w != 0.0 {
                        let max_border_radius = max(
                            max(
                                max(self.border_radius.x, self.border_radius.y),
                                max(self.border_radius.z, self.border_radius.w)
                            ), 1.0
                        );
                        let v = GaussShadow::rounded_box_shadow(
                            shadow_lower,
                            shadow_upper,
                            self.pos * self.rect_size3,
                            self.blur_radius,
                            max_border_radius
                        );
                        let shadow_color = vec4(self.get_shadow_color().rgb, self.get_shadow_color().a * v);
                        sdf.clear(shadow_color);
                    } else {
                        let v = GaussShadow::box_shadow(
                            shadow_lower,
                            shadow_upper,
                            self.pos * self.rect_size3,
                            self.blur_radius
                        );
                        let shadow_color = vec4(self.get_shadow_color().rgb, self.get_shadow_color().a * v);
                        sdf.clear(shadow_color);
                    }
                }
            }

            // - [basic sdf for draw a view] ------------------------------------------------------
            let border_width = self.border_width;
            let total_shadow_size = self.spread_radius + self.blur_radius;

            // 使用calculated位置而不是原始rect_size
            sdf.box_all(
                self.sdf_rect_pos.x,
                self.sdf_rect_pos.y,
                self.sdf_rect_size.x,
                self.sdf_rect_size.y,
                max(self.border_radius.x, 1.0),
                max(self.border_radius.y, 1.0),
                max(self.border_radius.z, 1.0),
                max(self.border_radius.w, 1.0)
            );

            // - [background color if visible] ----------------------------------------------------
            if self.background_visible == 1.0 {
                sdf.fill_keep(self.get_background_color());
            }

            // - [border with and color if width bigger than 0] -----------------------------------
            if border_width > 0.0 {
                sdf.stroke(self.get_border_color(), border_width);
            }

            return sdf.result;
        }
    }
}

#[derive(Live, LiveRegister, LiveHook)]
#[repr(C)]
pub struct DrawView {
    #[deref]
    pub draw_super: DrawQuad,
    #[live]
    pub background_color: Vec4,
    #[live]
    pub border_color: Vec4,
    #[live]
    pub border_width: f32,
    #[live]
    pub border_radius: Vec4,
    #[live]
    pub shadow_color: Vec4,
    #[live]
    pub spread_radius: f32,
    #[live]
    pub blur_radius: f32,
    #[live]
    pub background_visible: f32,
    #[live]
    pub rotation: f32,
    #[live]
    pub scale: f32,
    #[live]
    pub shadow_offset: Vec2,
}

impl DrawView {
    pub fn merge(&mut self, prop: &ViewBasicStyle) {
        self.background_color = prop.background_color;
        self.border_color = prop.border_color;
        self.border_width = prop.border_width;
        self.border_radius = prop.border_radius.into();
        self.shadow_color = prop.shadow_color.into();
        self.spread_radius = prop.spread_radius;
        self.blur_radius = prop.blur_radius;
        self.shadow_offset = prop.shadow_offset;
        self.background_visible = prop.background_visible.to_f32();
        self.rotation = prop.rotation;
        self.scale = prop.scale;
    }
}
