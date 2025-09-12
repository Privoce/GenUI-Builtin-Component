use makepad_widgets::*;

use crate::{
    component_colors, component_state,
    components::{
        live_props::LiveProps,
        traits::{BasicStyle, ComponentState, Style},
    },
    error::Error,
    get_get_mut, getter_setter_prop, inherits_view_basic_prop,
    prop::{
        manuel::{
            ABS_POS, ALIGN, BACKGROUND_COLOR, BACKGROUND_VISIBLE, BASIC, BLUR_RADIUS, BORDER_COLOR,
            BORDER_RADIUS, BORDER_WIDTH, CLIP_X, CLIP_Y, CURSOR, DISABLED, FLOW, HEIGHT, HOVER,
            MARGIN, PADDING, PRESSED, ROTATION, SCALE, SHADOW_COLOR, SHADOW_OFFSET, SPACING,
            SPREAD_RADIUS, THEME, WIDTH,
        },
        traits::AbsPos,
        traits::{FromLiveColor, FromLiveValue, NewFrom, ToTomlValue, ToColor},
        ApplyStateMapImpl, Radius,
    },
    prop_interconvert, state_colors,
    themes::{Color, Theme, TomlValueTo},
};

prop_interconvert! {
    ViewStyle {
        basic_prop = ViewBasicStyle;
        basic => BASIC, ViewBasicStyle::default(),|v| (v, ViewState::Basic).try_into(),
        hover => HOVER, ViewBasicStyle::from_state(Theme::default(), ViewState::Hover), |v| (v, ViewState::Hover).try_into(),
        pressed => PRESSED, ViewBasicStyle::from_state(Theme::default(), ViewState::Pressed), |v| (v, ViewState::Pressed).try_into(),
        disabled => DISABLED, ViewBasicStyle::from_state(Theme::default(), ViewState::Disabled), |v| (v, ViewState::Disabled).try_into()
    }, "[component.view] should be a table"
}

impl Style for ViewStyle {
    type State = ViewState;
    type Basic = ViewBasicStyle;

    get_get_mut! {
        ViewState::Basic => basic,
        ViewState::Hover => hover,
        ViewState::Pressed => pressed,
        ViewState::Disabled => disabled
    }

    fn len() -> usize {
        ViewBasicStyle::len() * 3
    }

    fn sync(&mut self, map: &crate::prop::ApplyStateMap<Self::State>) -> ()
    where
        Self::State: Eq + std::hash::Hash + Copy,
    {
        map.sync(
            &mut self.basic,
            ViewState::Basic,
            [
                (ViewState::Hover, &mut self.hover),
                (ViewState::Pressed, &mut self.pressed),
                (ViewState::Disabled, &mut self.disabled),
            ],
        );
    }
}

component_colors! {
    ViewColors {
        colors = (Color, Color, Color);
        background_color, border_color, shadow_color
    }
}

inherits_view_basic_prop! {
    ViewBasicStyle {
        border_width: 0.0,
        border_radius: Radius::new(4.0),
        spread_radius: 0.0,
        blur_radius: 0.0,
        shadow_offset: vec2(0.0, 0.0),
        background_visible: false,
        rotation: 0.0,
        scale: 1.0,
        padding: Padding::from_f64(12.0),
        margin: Margin::from_f64(0.0),
        clip_x: false,
        clip_y: false,
        align: Align::default(),
        cursor: MouseCursor::default(),
        flow: Flow::Down,
        spacing: 8.0,
        height: Size::Fill,
        width: Size::Fill,
        abs_pos: None,
    }, ViewState, "view",
    {
        ViewState::Basic => (500, 500, 400),
        ViewState::Hover => (400, 400, 300),
        ViewState::Pressed => (600, 600, 500),
        ViewState::Disabled => (300, 300, 200)
    }
}

impl ViewBasicStyle {
    getter_setter_prop! {
        get_theme, set_theme: theme -> Theme,
        get_background_color, set_background_color: background_color -> Vec4,
        get_border_color, set_border_color: border_color -> Vec4,
        get_border_width, set_border_width: border_width -> f32,
        get_border_radius, set_border_radius: border_radius -> Radius,
        get_shadow_color, set_shadow_color: shadow_color -> Vec4,
        get_spread_radius, set_spread_radius: spread_radius -> f32,
        get_blur_radius, set_blur_radius: blur_radius -> f32,
        get_shadow_offset, set_shadow_offset: shadow_offset -> Vec2,
        get_background_visible, set_background_visible: background_visible -> bool,
        get_rotation, set_rotation: rotation -> f32,
        get_scale, set_scale: scale -> f32,
        get_padding, set_padding: padding -> Padding,
        get_margin, set_margin: margin -> Margin,
        get_clip_x, set_clip_x: clip_x -> bool,
        get_clip_y, set_clip_y: clip_y -> bool,
        get_align, set_align: align -> Align,
        get_cursor, set_cursor: cursor -> MouseCursor,
        get_flow, set_flow: flow -> Flow,
        get_spacing, set_spacing: spacing -> f64,
        get_height, set_height: height -> Size,
        get_width, set_width: width -> Size,
        get_abs_pos, set_abs_pos: abs_pos -> Option<DVec2>
    }
}

component_state! {
    ViewState {
        Basic => BASIC,
        Hover => HOVER,
        Pressed => PRESSED,
        Disabled => DISABLED
    }, _ => ViewState::Basic
}

impl ViewState {
    pub fn id(&self) -> &[LiveId; 2] {
        match self {
            ViewState::Basic => id!(hover.off),
            ViewState::Hover => id!(hover.on),
            ViewState::Pressed => id!(hover.pressed),
            ViewState::Disabled => id!(hover.off),
        }
    }
}

impl ComponentState for ViewState {
    fn is_disabled(&self) -> bool {
        matches!(self, ViewState::Disabled)
    }
}
