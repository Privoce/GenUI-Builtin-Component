use makepad_widgets::*;

use crate::{
    basic_prop_interconvert, component_colors, component_state,
    components::{
        live_props::LiveProps,
        traits::{BasicStyle, ComponentState, Style},
        view::ViewBasicStyle,
    },
    error::Error,
    get_get_mut,
    prop::{
        manuel::{
            ABS_POS, BACKGROUND_COLOR, BACKGROUND_VISIBLE, BASIC, BLUR_RADIUS, BORDER_RADIUS,
            CURSOR, HEIGHT, MARGIN, SHADOW_COLOR, SHADOW_OFFSET, SPREAD_RADIUS, THEME, WIDTH,
        },
        traits::{AbsPos, FromLiveColor, FromLiveValue, NewFrom, ToColor, ToTomlValue},
        ApplyStateMapImpl, Radius,
    },
    prop_interconvert, state_colors,
    themes::{Color, Theme, TomlValueTo},
};

prop_interconvert! {
    DividerProp {
        basic_prop = DividerBasicStyle;
        basic => BASIC, DividerBasicStyle::default(), |v| (v, DividerState::Basic).try_into()
    }, "[component.divider] should be a table"
}

impl Style for DividerProp {
    type State = DividerState;

    type Basic = DividerBasicStyle;

    get_get_mut! {
        DividerState::Basic => basic
    }

    fn len() -> usize {
        1 * DividerBasicStyle::len()
    }

    fn sync(&mut self, map: &crate::prop::ApplyStateMap<Self::State>) -> ()
    where
        Self::State: Eq + std::hash::Hash + Copy,
    {
        map.sync(&mut self.basic, DividerState::Basic, []);
    }
}

basic_prop_interconvert! {
    DividerBasicStyle {
        state = DividerState;
        {
            background_color => BACKGROUND_COLOR, |v| v.try_into(),
            shadow_color => SHADOW_COLOR, |v| v.try_into()
        };
        {
            border_radius: Radius => BORDER_RADIUS, Radius::new(1.0), |v| v.try_into(),
            spread_radius: f32 => SPREAD_RADIUS, 0.0, |v| v.to_f32(),
            blur_radius: f32 => BLUR_RADIUS, 0.0, |v| v.to_f32(),
            shadow_offset: Vec2 => SHADOW_OFFSET, vec2(0.0, 0.0), |v| v.to_vec2(shadow_offset),
            background_visible: bool => BACKGROUND_VISIBLE, true, |v| v.to_bool(),
            margin: Margin => MARGIN, Margin::from_f64(0.0), |v| v.to_margin(margin),
            cursor: MouseCursor => CURSOR, MouseCursor::Default, |v| v.to_cursor(),
            height: Size => HEIGHT, Size::Fixed(1.2), |v| v.to_size(),
            width: Size => WIDTH, Size::Fill, |v| v.to_size(),
            abs_pos: AbsPos => ABS_POS, None, |v| Ok(v.to_dvec2().map_or(None, |v| Some(v)))
        }
    }, "[components.divider.$state] should be an inline table"
}

component_colors! {
    DividerColors {
        colors = (Color, Color);
        background_color, shadow_color
    }
}

impl BasicStyle for DividerBasicStyle {
    type State = DividerState;
    /// (background_color, shadow_color)
    type Colors = DividerColors;

    fn from_state(theme: crate::themes::Theme, state: Self::State) -> Self {
        let DividerColors {
            background_color,
            shadow_color,
        } = Self::state_colors(theme, state);
        Self {
            theme,
            background_color: background_color.into(),
            border_radius: Radius::new(1.0),
            shadow_color: shadow_color.into(),
            spread_radius: 0.0,
            blur_radius: 0.0,
            shadow_offset: vec2(0.0, 0.0),
            background_visible: true,
            margin: Margin::from_f64(0.0),
            cursor: MouseCursor::default(),
            height: Size::Fixed(1.2),
            width: Size::Fill,
            abs_pos: None,
        }
    }

    state_colors! {
        (bg_level, shadow_level),
        DividerState::Basic => (300, 400)
    }

    fn len() -> usize {
        12
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
            BORDER_RADIUS => {
                self.border_radius = Radius::from_live_value(value).unwrap_or(Radius::new(1.0));
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
            MARGIN => {
                self.margin = Margin::from_live_value(value).unwrap_or(Margin::from_f64(0.0));
            }
            HEIGHT => {
                self.height = Size::from_live_value(value).unwrap_or(Size::Fixed(1.2));
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
        let DividerColors {
            background_color,
            shadow_color,
        } = Self::state_colors(self.theme, state);
        self.background_color = background_color.into();
        self.shadow_color = shadow_color.into();
    }

    fn live_props() -> LiveProps {
        vec![
            (live_id!(theme), None.into()),
            (live_id!(background_color), None.into()),
            (live_id!(border_radius), None.into()),
            (live_id!(shadow_color), None.into()),
            (live_id!(spread_radius), None.into()),
            (live_id!(blur_radius), None.into()),
            (live_id!(shadow_offset), None.into()),
            (live_id!(background_visible), None.into()),
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
            (live_id!(cursor), None.into()),
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
            clip_x: false,
            clip_y: false,
            ..Default::default()
        }
    }
}

impl From<&DividerBasicStyle> for ViewBasicStyle {
    fn from(value: &DividerBasicStyle) -> Self {
        let DividerBasicStyle {
            theme,
            background_color,
            border_radius,
            shadow_color,
            spread_radius,
            blur_radius,
            shadow_offset,
            background_visible,
            margin,
            cursor,
            height,
            width,
            abs_pos,
        } = value;

        ViewBasicStyle {
            theme: *theme,
            background_color: *background_color,
            border_radius: *border_radius,
            shadow_color: *shadow_color,
            spread_radius: *spread_radius,
            blur_radius: *blur_radius,
            shadow_offset: *shadow_offset,
            background_visible: *background_visible,
            margin: *margin,
            cursor: *cursor,
            height: *height,
            width: *width,
            abs_pos: *abs_pos,
            border_color: Default::default(),
            border_width: 0.0,
            rotation: 0.0,
            scale: 1.0,
            padding: Padding::default(),
            clip_x: false,
            clip_y: false,
            align: Align::default(),
            flow: Flow::default(),
            spacing: 0.0,
        }
    }
}

component_state! {
    DividerState {
        Basic => BASIC
    }, _ => DividerState::Basic
}

impl ComponentState for DividerState {
    fn is_disabled(&self) -> bool {
        false
    }
}
