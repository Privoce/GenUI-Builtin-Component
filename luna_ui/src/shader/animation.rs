use makepad_widgets::*;

live_design!{
    link luna_animation_prop;

    pub AN_DURATION = 0.25, // default animation duration

    // animation for default draw view
    pub AN_DRAW_VIEW = {
        background_color: #535d6d,
        border_color: #535d6d,
        border_width: 0.0,
        // border_radius: vec4(4.0, 4.0, 4.0, 4.0),
        shadow_color: #535d6d,
        spread_radius: 0.0,
        blur_radius: 0.0,
        background_visible: 1.0,
        rotation: 0.0,
        scale: 1.0,
        shadow_offset: vec2(0.0, 0.0),
    }

    pub AN_DRAW_RADIO = {
        background_color: #535d6d,
        background_visible: 1.0,
        border_color: #535d6d,
        border_width: 1.0,
        size: 16.0,
        // mode: ActiveMode::Round,
        stroke_color: #ffffff,
    }
}