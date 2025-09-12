use crate::{
    basic_prop_interconvert, component_colors, component_state,
    components::{
        live_props::LiveProps,
        traits::{BasicStyle, ComponentState, Style},
    },
    error::Error,
    get_get_mut,
    prop::{
        manuel::{
            ABS_POS, ACTIVE, BACKGROUND_COLOR, BACKGROUND_VISIBLE, BASIC, BORDER_COLOR,
            BORDER_RADIUS, BORDER_WIDTH, CURSOR, DISABLED, HOVER_ACTIVE, HOVER_BASIC, MARGIN, SIZE,
            STROKE_COLOR, THEME,
        },
        traits::{AbsPos, FromLiveColor, FromLiveValue, NewFrom, ToColor, ToTomlValue},
        ApplyStateMapImpl, Radius,
    },
    prop_interconvert, state_colors,
    themes::{Color, Theme, TomlValueTo},
};
use makepad_widgets::*;

prop_interconvert! {
    SwitchProp {
        basic_prop = SwitchBasicStyle;
        basic => BASIC, SwitchBasicStyle::default(),|v| (v, SwitchState::Basic).try_into(),
        hover_basic => HOVER_BASIC, SwitchBasicStyle::from_state(Theme::default(), SwitchState::HoverBasic),|v| (v, SwitchState::HoverBasic).try_into(),
        hover_active => HOVER_ACTIVE, SwitchBasicStyle::from_state(Theme::default(), SwitchState::HoverActive),|v| (v, SwitchState::HoverActive).try_into(),
        active => ACTIVE, SwitchBasicStyle::from_state(Theme::default(), SwitchState::Active),|v| (v, SwitchState::Active).try_into(),
        disabled => DISABLED, SwitchBasicStyle::from_state(Theme::default(), SwitchState::Disabled),|v| (v, SwitchState::Disabled).try_into()
    }, "[component.checkbox] should be a table"
}

impl Style for SwitchProp {
    type State = SwitchState;

    type Basic = SwitchBasicStyle;

    get_get_mut! {
        SwitchState::Basic => basic,
        SwitchState::HoverBasic => hover_basic,
        SwitchState::HoverActive => hover_active,
        SwitchState::Active => active,
        SwitchState::Disabled => disabled
    }

    fn len() -> usize {
        4 * SwitchBasicStyle::len()
    }

    fn sync(&mut self, map: &crate::prop::ApplyStateMap<Self::State>) -> ()
    where
        Self::State: Eq + std::hash::Hash + Copy,
    {
        map.sync(
            &mut self.basic,
            SwitchState::Basic,
            [
                (SwitchState::HoverBasic, &mut self.hover_basic),
                (SwitchState::HoverActive, &mut self.hover_active),
                (SwitchState::Active, &mut self.active),
                (SwitchState::Disabled, &mut self.disabled),
            ],
        );
    }
}

basic_prop_interconvert! {
    SwitchBasicStyle {
        state = SwitchState;
        {
            background_color => BACKGROUND_COLOR, |v| v.try_into(),
            stroke_color => STROKE_COLOR, |v| v.try_into(),
            border_color => BORDER_COLOR, |v| v.try_into()
        };
        {
            size: f32 => SIZE, 24.0, |v| v.to_f32(),
            background_visible: bool => BACKGROUND_VISIBLE, true, |v| v.to_bool(),
            border_width: f32 => BORDER_WIDTH, 1.0, |v| v.to_f32(),
            margin: Margin => MARGIN, Margin::from_f64(0.0), |v| v.to_margin(margin),
            abs_pos: AbsPos => ABS_POS, None, |v| Ok(v.to_dvec2().map_or(None, |v| Some(v))),
            cursor: MouseCursor => CURSOR, MouseCursor::Hand, |v| v.to_cursor(),
            border_radius: Radius => BORDER_RADIUS, Radius::new(5.4), |v| v.try_into()
        }
    }, "[components.switch.$state] should be an inline table"
}

component_colors! {
    SwitchColors {
        colors = (Color, Color, Color);
        background_color, stroke_color, border_color
    }
}

impl BasicStyle for SwitchBasicStyle {
    type State = SwitchState;
    /// (background_color, stroke_color, border_color)
    type Colors = SwitchColors;

    fn from_state(theme: Theme, state: Self::State) -> Self {
        let SwitchColors {
            background_color,
            stroke_color,
            border_color,
        } = Self::state_colors(theme, state);
        let cursor = if state.is_disabled() {
            MouseCursor::NotAllowed
        } else {
            MouseCursor::Hand
        };
        Self {
            theme,
            size: 24.0,
            background_color: background_color.into(),
            stroke_color: stroke_color.into(),
            border_color: border_color.into(),
            background_visible: true,
            border_width: 1.0,
            margin: Margin::from_f64(0.0),
            abs_pos: None,
            border_radius: Radius::new(5.4),
            cursor,
        }
    }

    state_colors! {
        (bg_level, stroke_level, border_level),
        SwitchState::Basic => (200, 400, 400),
        SwitchState::HoverBasic => (100, 300, 500),
        SwitchState::HoverActive => (500, 300, 500),
        SwitchState::Active => (400, 200, 500),
        SwitchState::Disabled => (100, 200, 300)
    }

    fn len() -> usize {
        11
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
            STROKE_COLOR => {
                let colors = Self::state_colors(self.theme, state);
                self.stroke_color =
                    Vec4::from_live_color(value).unwrap_or(colors.stroke_color.into());
            }
            BORDER_COLOR => {
                let colors = Self::state_colors(self.theme, state);
                self.border_color =
                    Vec4::from_live_color(value).unwrap_or(colors.border_color.into());
            }
            SIZE => {
                self.size = f32::from_live_value(value).unwrap_or(24.0);
            }
            BACKGROUND_VISIBLE => {
                self.background_visible = bool::from_live_value(value).unwrap_or(true);
            }
            BORDER_WIDTH => {
                self.border_width = f32::from_live_value(value).unwrap_or(1.0);
            }
            MARGIN => {
                self.margin = Margin::from_live_value(value).unwrap_or(Margin::from_f64(0.0));
            }
            ABS_POS => {
                self.abs_pos = DVec2::from_live_value(value);
            }
            CURSOR => {
                let cursor = if state.is_disabled() {
                    MouseCursor::NotAllowed
                } else {
                    MouseCursor::Hand
                };
                self.cursor = MouseCursor::from_live_value(value).unwrap_or(cursor);
            }
            _ => {}
        }
    }

    fn sync(&mut self, state: Self::State) -> () {
        let SwitchColors {
            background_color,
            stroke_color,
            border_color,
        } = Self::state_colors(self.theme, state);
        self.background_color = background_color.into();
        self.stroke_color = stroke_color.into();
        self.border_color = border_color.into();
    }

    fn live_props() -> LiveProps {
        vec![
            (live_id!(theme), None.into()),
            (live_id!(size), None.into()),
            (live_id!(background_color), None.into()),
            (live_id!(stroke_color), None.into()),
            (live_id!(border_color), None.into()),
            (live_id!(background_visible), None.into()),
            (live_id!(border_width), None.into()),
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
            (live_id!(abs_pos), None.into()),
            (live_id!(cursor), None.into()),
        ]
    }

    fn walk(&self) -> Walk {
        Walk {
            abs_pos: self.abs_pos,
            margin: self.margin,
            width: Size::Fixed(self.size as f64 * 1.8),
            height: Size::Fixed(self.size as f64),
        }
    }

    fn layout(&self) -> Layout {
        Layout {
            clip_x: false,
            clip_y: false,
            padding: Padding::from_f64(0.0),
            ..Default::default()
        }
    }
}

component_state! {
    SwitchState {
        Basic => BASIC,
        HoverBasic => HOVER_BASIC,
        HoverActive => HOVER_ACTIVE,
        Active => ACTIVE,
        Disabled => DISABLED
    },
    _ => SwitchState::Basic
}

impl ComponentState for SwitchState {
    fn is_disabled(&self) -> bool {
        matches!(self, SwitchState::Disabled)
    }
}
