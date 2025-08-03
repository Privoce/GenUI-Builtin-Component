use makepad_widgets::*;

live_design! {
    use link::shaders::*;

    DrawImg = {{DrawImg}}{
        texture image: texture2d

        fn get_color_scale_pan(self, scale: vec2, pan: vec2) -> vec4 {
            return sample2d(self.image, self.pos * scale + pan).xyzw;
        }

        fn get_color(self) -> vec4 {
            return self.get_color_scale_pan(self.image_scale, self.image_pan)
        }

        fn pixel(self) -> vec4 {
            let color = mix(self.get_color(), #3, self.load);
            return Pal::premul(vec4(color.xyz, color.w * self.opacity))
        }
    }
}

#[derive(Live, LiveHook, LiveRegister)]
#[repr(C)]
pub struct DrawImg {
    #[deref]
    draw_super: DrawQuad,
    #[live(1.0)]
    pub opacity: f32,
    #[live(vec2(1.0, 1.0))]
    pub image_scale: Vec2,
    #[live(vec2(0.0, 0.0))]
    pub image_pan: Vec2,
    #[live]
    pub load: f32,
}
