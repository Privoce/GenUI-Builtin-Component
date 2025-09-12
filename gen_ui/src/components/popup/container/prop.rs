use makepad_widgets::*;

use crate::{
    basic_prop_interconvert, component_color,
    components::{
        live_props::LiveProps,
        popup::PopupState,
        traits::{BasicStyle, ComponentState, Style},
        view::ViewBasicStyle,
    },
    error::Error,
    prop::{
        manuel::{
            ABS_POS, ALIGN, BACKGROUND_COLOR, BACKGROUND_VISIBLE, BASIC, CLIP_X, CLIP_Y, CURSOR,
            FLOW, HEIGHT, MARGIN, PADDING, SPACING, THEME, WIDTH,
        },
        traits::{AbsPos, FromLiveColor, FromLiveValue, NewFrom, ToColor, ToTomlValue},
        ApplyStateMapImpl,
    },
    prop_interconvert, state_color,
    themes::{Theme, TomlValueTo},
};

prop_interconvert! {
    PopupContainerProp {
        basic_prop = PopupContainerBasicStyle;
        basic => BASIC, PopupContainerBasicStyle::default(),|v| (v, PopupState::Basic).try_into()
    }, "[component.popup] should be a table"
}

impl Style for PopupContainerProp {
    type State = PopupState;

    type Basic = PopupContainerBasicStyle;

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
        PopupContainerBasicStyle::len()
    }

    fn sync(&mut self, map: &crate::prop::ApplyStateMap<Self::State>) -> ()
    where
        Self::State: Eq + std::hash::Hash + Copy,
    {
        map.sync(&mut self.basic, PopupState::Basic, []);
    }
}

basic_prop_interconvert! {
    PopupContainerBasicStyle {
        state = PopupState;
        {background_color => BACKGROUND_COLOR, |v| v.try_into()};
        {
            background_visible: bool => BACKGROUND_VISIBLE, true, |v| v.to_bool(),
            padding: Padding => PADDING, Padding::from_f64(0.0), |v| v.to_padding(padding),
            margin: Margin => MARGIN, Margin::from_f64(0.0), |v| v.to_margin(margin),
            clip_x: bool => CLIP_X, false, |v| v.to_bool(),
            clip_y: bool => CLIP_Y, false, |v| v.to_bool(),
            align: Align => ALIGN, Align::default(), |v| v.to_align(align),
            cursor: MouseCursor => CURSOR, MouseCursor::default(), |v| v.to_cursor(),
            flow: Flow => FLOW, Flow::Down, |v| v.to_flow(),
            spacing: f64 => SPACING, 6.0, |v| v.to_f64(),
            height: Size => HEIGHT, Size::Fill, |v| v.to_size(),
            width: Size => WIDTH, Size::Fill, |v| v.to_size(),
            abs_pos: AbsPos => ABS_POS, None, |v| Ok(v.to_dvec2().map_or(None, |v| Some(v)))
        }
    }, "[component.popup_container] should be a table"
}

component_color! {
    PopupContainerColors {
        colors = (Color);
        background_color
    }
}

impl BasicStyle for PopupContainerBasicStyle {
    type State = PopupState;

    type Colors = PopupContainerColors;

    fn len() -> usize {
        14
    }

    fn set_from_str(&mut self, key: &str, value: &LiveValue, state: Self::State) -> () {
        match key {
            THEME => {
                self.theme = Theme::from_live_value(value).unwrap_or(Theme::default());
                self.sync(state);
            }
            BACKGROUND_COLOR => {
                let PopupContainerColors { background_color } =
                    Self::state_colors(self.theme, state);
                self.background_color =
                    Vec4::from_live_color(value).unwrap_or(background_color.into());
            }
            BACKGROUND_VISIBLE => {
                self.background_visible = bool::from_live_value(value).unwrap_or(true);
            }
            PADDING => {
                self.padding = Padding::from_live_value(value).unwrap_or(Padding::from_f64(0.0));
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
        let PopupContainerColors { background_color } = Self::state_colors(self.theme, state);
        self.background_color = background_color.into();
    }

    fn from_state(theme: Theme, state: Self::State) -> Self {
        let PopupContainerColors { background_color } = Self::state_colors(theme, state);

        let cursor = if state.is_disabled() {
            MouseCursor::NotAllowed
        } else {
            MouseCursor::default()
        };

        Self {
            theme,
            background_color: background_color.into(),
            background_visible: true,
            padding: Padding::from_f64(0.0),
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

    state_color! {
        (bg_level),
        PopupState::Basic => (300)
    }

    fn live_props() -> LiveProps {
        vec![
            (live_id!(theme), None.into()),
            (live_id!(background_color), None.into()),
            (live_id!(background_visible), None.into()),
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

impl From<&PopupContainerBasicStyle> for ViewBasicStyle {
    fn from(value: &PopupContainerBasicStyle) -> Self {
        let PopupContainerBasicStyle {
            theme,
            background_color,
            background_visible,
            padding,
            margin,
            clip_x,
            clip_y,
            align,
            cursor,
            flow,
            spacing,
            height,
            width,
            abs_pos,
        } = *value;

        ViewBasicStyle {
            theme,
            background_color,
            background_visible,
            padding,
            margin,
            clip_x,
            clip_y,
            align,
            cursor,
            flow,
            spacing,
            height,
            width,
            abs_pos,
            ..Default::default()
        }
    }
}

// component_state! {
//     PopupState {
//         Basic => BASIC
//     }, _ => PopupState::Basic
// }

// impl ComponentState for PopupState {
//     fn is_disabled(&self) -> bool {
//         false
//     }
// }
