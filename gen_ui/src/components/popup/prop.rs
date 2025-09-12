use makepad_widgets::*;

use crate::{
    basic_prop_interconvert, component_colors, component_state,
    components::{
        live_props::LiveProps,
        traits::{BasicStyle, ComponentState, Style},
        view::ViewBasicStyle,
    },
    error::Error,
    from_inherit_to_view_basic_prop,
    prop::{
        manuel::{
            ABS_POS, ALIGN, BACKGROUND_COLOR, BACKGROUND_VISIBLE, BASIC, BLUR_RADIUS, BORDER_COLOR,
            BORDER_RADIUS, BORDER_WIDTH, CLIP_X, CLIP_Y, CURSOR, FLOW, HEIGHT, MARGIN, PADDING,
            ROTATION, SCALE, SHADOW_COLOR, SHADOW_OFFSET, SPACING, SPREAD_RADIUS, THEME, WIDTH,
        },
        traits::{AbsPos, FromLiveColor, FromLiveValue, NewFrom, ToColor, ToTomlValue},
        ApplyStateMapImpl, Radius,
    },
    prop_interconvert, state_colors,
    themes::{Color, Theme, TomlValueTo},
};

prop_interconvert! {
    PopupProp {
        basic_prop = PopupBasicStyle;
        basic => BASIC, PopupBasicStyle::default(),|v| (v, PopupState::Basic).try_into()
    }, "[component.popup] should be a table"
}

impl Style for PopupProp {
    type State = PopupState;

    type Basic = PopupBasicStyle;

    fn get(&self, state: Self::State) -> &Self::Basic {
        match state {
            PopupState::Basic => &self.basic,
        }
    }

    fn get_mut(&mut self, state: Self::State) -> &mut Self::Basic {
        match state {
            PopupState::Basic => &mut self.basic,
        }
    }

    fn len() -> usize {
        PopupBasicStyle::len()
    }

    fn sync(&mut self, map: &crate::prop::ApplyStateMap<Self::State>) -> ()
    where
        Self::State: Eq + std::hash::Hash + Copy,
    {
        map.sync(&mut self.basic, PopupState::Basic, []);
    }
}

basic_prop_interconvert! {
    PopupBasicStyle {
        state = PopupState;
        {
            background_color => BACKGROUND_COLOR, |v| v.try_into(),
            border_color => BORDER_COLOR, |v| v.try_into(),
            shadow_color => SHADOW_COLOR, |v| v.try_into()
        };
        {
            border_width: f32 => BORDER_WIDTH, 0.0, |v| v.to_f32(),
            border_radius: Radius => BORDER_RADIUS, Radius::new(8.0), |v| v.try_into(),
            spread_radius: f32 => SPREAD_RADIUS, 0.0, |v| v.to_f32(),
            blur_radius: f32 => BLUR_RADIUS, 0.0, |v| v.to_f32(),
            shadow_offset: Vec2 => SHADOW_OFFSET, vec2(0.0, 0.0), |v| v.to_vec2(shadow_offset),
            background_visible: bool => BACKGROUND_VISIBLE, true, |v| v.to_bool(),
            rotation: f32 => ROTATION, 0.0, |v| v.to_f32(),
            scale: f32 => SCALE, 1.0, |v| v.to_f32(),
            padding: Padding => PADDING, Padding::from_f64(12.0), |v| v.to_padding(padding),
            margin: Margin => MARGIN, Margin::from_f64(0.0), |v| v.to_margin(margin),
            clip_x: bool => CLIP_X, false, |v| v.to_bool(),
            clip_y: bool => CLIP_Y, false, |v| v.to_bool(),
            align: Align => ALIGN, Align::default(), |v| v.to_align(Align::default()),
            cursor: MouseCursor => CURSOR, MouseCursor::default(), |v| v.to_cursor(),
            flow: Flow => FLOW, Flow::Down, |v| v.to_flow(),
            spacing: f64 => SPACING, 6.0, |v| v.to_f64(),
            height: Size => HEIGHT, Size::Fill, |v| v.to_size(),
            width: Size => WIDTH, Size::Fill, |v| v.to_size(),
            abs_pos: AbsPos => ABS_POS, None, |v| Ok(v.to_dvec2().map_or(None, |v| Some(v)))
        }
    }, "PopupBasicStyle should be a inline table"
}

component_colors! {
    PopupColors {
        colors = (Color, Color, Color);
        background_color, border_color, shadow_color
    }
}

impl BasicStyle for PopupBasicStyle {
    type State = PopupState;

    type Colors = PopupColors;

    fn len() -> usize {
        22
    }

    fn set_from_str(&mut self, key: &str, value: &LiveValue, state: Self::State) -> () {
        match key {
            THEME => {
                self.theme = Theme::from_live_value(value).unwrap_or(Theme::default());
                self.sync(state);
            }
            BACKGROUND_COLOR => {
                let colors = Self::state_colors(self.theme, state);
                self.background_color =
                    Vec4::from_live_color(value).unwrap_or(colors.background_color.into());
            }
            BORDER_COLOR => {
                let colors = Self::state_colors(self.theme, state);
                self.border_color =
                    Vec4::from_live_color(value).unwrap_or(colors.border_color.into());
            }
            BORDER_WIDTH => {
                self.border_width = f32::from_live_value(value).unwrap_or(0.0);
            }
            BORDER_RADIUS => {
                self.border_radius = Radius::from_live_value(value).unwrap_or(Radius::new(8.0));
            }
            SHADOW_COLOR => {
                let colors = Self::state_colors(self.theme, state);
                self.shadow_color =
                    Vec4::from_live_color(value).unwrap_or(colors.shadow_color.into());
            }
            SPREAD_RADIUS => {
                self.spread_radius = f32::from_live_value(value).unwrap_or(0.0);
            }
            BLUR_RADIUS => {
                self.blur_radius = f32::from_live_value(value).unwrap_or(0.0);
            }
            SHADOW_OFFSET => {
                self.shadow_offset = Vec2::from_live_value(value).unwrap_or(vec2(0.0, 0.0));
            }
            BACKGROUND_VISIBLE => {
                self.background_visible = bool::from_live_value(value).unwrap_or(true);
            }
            ROTATION => {
                self.rotation = f32::from_live_value(value).unwrap_or(0.0);
            }
            SCALE => {
                self.scale = f32::from_live_value(value).unwrap_or(1.0);
            }
            PADDING => {
                self.padding = Padding::from_live_value(value).unwrap_or(Padding::from_f64(12.0));
            }
            MARGIN => {
                self.margin = Margin::from_live_value(value).unwrap_or(Margin::from_f64(0.0));
            }
            CLIP_X => {
                self.clip_x = bool::from_live_value(value).unwrap_or(false);
            }
            CLIP_Y => {
                self.clip_y = bool::from_live_value(value).unwrap_or(false);
            }
            ALIGN => {
                self.align = Align::from_live_value(value).unwrap_or(Align::default());
            }
            CURSOR => {
                let cursor = if state.is_disabled() {
                    MouseCursor::NotAllowed
                } else {
                    MouseCursor::Default
                };
                self.cursor = MouseCursor::from_live_value(value).unwrap_or(cursor);
            }
            FLOW => {
                self.flow = Flow::from_live_value(value).unwrap_or(Flow::Down);
            }
            SPACING => {
                self.spacing = f64::from_live_value(value).unwrap_or(6.0);
            }
            HEIGHT => {
                self.height = Size::from_live_value(value).unwrap_or(Size::Fill);
            }
            WIDTH => {
                self.width = Size::from_live_value(value).unwrap_or(Size::Fill);
            }
            ABS_POS => {
                self.abs_pos = DVec2::from_live_value(value);
            }
            _ => {}
        }
    }

    fn sync(&mut self, state: Self::State) -> () {
        let PopupColors {
            background_color,
            border_color,
            shadow_color,
        } = Self::state_colors(self.theme, state);
        self.background_color = background_color.into();
        self.border_color = border_color.into();
        self.shadow_color = shadow_color.into();
    }

    fn from_state(theme: Theme, state: Self::State) -> Self {
        let PopupColors {
            background_color,
            border_color,
            shadow_color,
        } = Self::state_colors(theme, state);

        let cursor = if state.is_disabled() {
            MouseCursor::NotAllowed
        } else {
            MouseCursor::default()
        };

        Self {
            theme,
            background_color: background_color.into(),
            border_color: border_color.into(),
            border_width: 0.0,
            border_radius: Radius::new(8.0),
            shadow_color: shadow_color.into(),
            spread_radius: 0.0,
            blur_radius: 0.0,
            shadow_offset: vec2(0.0, 0.0),
            background_visible: true,
            rotation: 0.0,
            scale: 1.0,
            padding: Padding::from_f64(12.0),
            margin: Margin::from_f64(0.0),
            clip_x: false,
            clip_y: false,
            align: Align::default(),
            cursor,
            flow: Flow::Down,
            spacing: 6.0,
            height: Size::Fill,
            width: Size::Fill,
            abs_pos: None,
        }
    }

    state_colors! {
        (bg_level, border_level, shadow_level),
        PopupState::Basic => (500, 500, 400)
    }

    fn live_props() -> LiveProps {
        vec![
            (live_id!(theme), None.into()),
            (live_id!(background_color), None.into()),
            (live_id!(border_color), None.into()),
            (live_id!(border_width), None.into()),
            (
                live_id!(border_radius),
                Some(vec![
                    live_id!(top),
                    live_id!(bottom),
                    live_id!(left),
                    live_id!(right),
                ])
                .into(),
            ),
            (live_id!(shadow_color), None.into()),
            (live_id!(spread_radius), None.into()),
            (live_id!(blur_radius), None.into()),
            (live_id!(shadow_offset), None.into()),
            (live_id!(background_visible), None.into()),
            (live_id!(rotation), None.into()),
            (live_id!(scale), None.into()),
            (
                live_id!(padding),
                Some(vec![
                    live_id!(top),
                    live_id!(bottom),
                    live_id!(left),
                    live_id!(right),
                ])
                .into(),
            ),
            (
                live_id!(margin),
                Some(vec![
                    live_id!(top),
                    live_id!(bottom),
                    live_id!(left),
                    live_id!(right),
                ])
                .into(),
            ),
            (live_id!(clip_x), None.into()),
            (live_id!(clip_y), None.into()),
            (live_id!(align), Some(vec![live_id!(x), live_id!(y)]).into()),
            (live_id!(cursor), None.into()),
            (live_id!(flow), None.into()),
            (live_id!(spacing), None.into()),
            (live_id!(height), None.into()),
            (live_id!(width), None.into()),
            (live_id!(abs_pos), None.into()),
        ]
    }

    fn walk(&self) -> Walk {
        Walk {
            abs_pos: self.abs_pos,
            margin: self.margin,
            width: self.width,
            height: self.height,
        }
    }

    fn layout(&self) -> Layout {
        Layout {
            clip_x: self.clip_x,
            clip_y: self.clip_y,
            padding: self.padding,
            align: self.align,
            flow: self.flow,
            spacing: self.spacing,
            ..Default::default()
        }
    }
}

from_inherit_to_view_basic_prop!(PopupBasicStyle);

component_state! {
    PopupState {
        Basic => BASIC
    }, _ => PopupState::Basic
}

impl ComponentState for PopupState {
    fn is_disabled(&self) -> bool {
        false
    }
}
