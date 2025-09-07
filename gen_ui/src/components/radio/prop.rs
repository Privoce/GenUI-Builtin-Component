use crate::{
    basic_prop_interconvert, component_colors, component_part, component_state,
    components::{
        label::{LabelBasicProp, LabelState},
        live_props::LiveProps,
        traits::{BasicProp, ComponentState, Part, Prop, SlotBasicProp, SlotProp},
        view::{ViewBasicProp, ViewState},
    },
    error::Error,
    from_prop_to_toml, get_get_mut,
    prop::{
        manuel::{
            ABS_POS, ACTIVE, BACKGROUND_COLOR, BACKGROUND_VISIBLE, BASIC, BORDER_COLOR,
            BORDER_WIDTH, CONTAINER, CURSOR, DISABLED, EXTRA, HOVER, MARGIN, MODE, RADIO, SIZE,
            STROKE_COLOR, THEME,
        },
        traits::{AbsPos, FromLiveColor, FromLiveValue, NewFrom, ToColor, ToTomlValue},
        ActiveMode, ApplySlotMapImpl,
    },
    prop_interconvert, state_colors,
    themes::{Color, Theme, TomlValueTo},
    utils::get_from_itable,
};
use makepad_widgets::*;
use toml_edit::Item;

prop_interconvert! {
    RadioProp {
        basic_prop = RadioBasicProp;
        basic => BASIC, RadioBasicProp::default(),|v| (v, RadioState::Basic).try_into(),
        hover => HOVER, RadioBasicProp::from_state(Theme::default(), RadioState::Hover),|v| (v, RadioState::Hover).try_into(),
        active => ACTIVE, RadioBasicProp::from_state(Theme::default(), RadioState::Active),|v| (v, RadioState::Active).try_into(),
        disabled => DISABLED, RadioBasicProp::from_state(Theme::default(), RadioState::Disabled),|v| (v, RadioState::Disabled).try_into()
    }, "[component.radio] should be a table"
}

impl SlotProp for RadioProp {
    type Part = RadioPart;

    fn sync_slot(&mut self, map: &crate::prop::ApplySlotMap<Self::State, Self::Part>) -> () {
        map.sync(
            &mut self.basic,
            RadioState::Basic,
            [
                (RadioState::Hover, &mut self.hover),
                (RadioState::Active, &mut self.active),
                (RadioState::Disabled, &mut self.disabled),
            ],
            [RadioPart::Container, RadioPart::Radio, RadioPart::Extra],
        );
    }
}

impl Prop for RadioProp {
    type State = RadioState;

    type Basic = RadioBasicProp;

    get_get_mut! {
        RadioState::Basic => basic,
        RadioState::Hover => hover,
        RadioState::Active => active,
        RadioState::Disabled => disabled
    }

    fn len() -> usize {
        4 * RadioBasicProp::len()
    }

    fn sync(&mut self, _map: &crate::prop::ApplyStateMap<Self::State>) -> ()
    where
        Self::State: Eq + std::hash::Hash + Copy,
    {
        ()
    }
}

#[derive(Debug, Clone, Live, LiveHook, LiveRegister, Copy)]
#[live_ignore]
pub struct RadioBasicProp {
    #[live(Self::default_container(Theme::default(), RadioState::Basic))]
    pub container: ViewBasicProp,
    #[live(Self::default_radio(Theme::default(), RadioState::Basic))]
    pub radio: RadioPartProp,
    #[live(Self::default_extra(Theme::default(), RadioState::Basic))]
    pub extra: ViewBasicProp,
}

impl Default for RadioBasicProp {
    fn default() -> Self {
        Self::from_state(Theme::default(), RadioState::Basic)
    }
}

from_prop_to_toml! {
    RadioBasicProp {
        container => CONTAINER,
        radio => RADIO,
        extra => EXTRA
    }
}

impl SlotBasicProp for RadioBasicProp {
    type Part = RadioPart;

    fn set_from_str_slot(
        &mut self,
        key: &str,
        value: &crate::prop::Applys,
        state: Self::State,
        part: Self::Part,
    ) -> () {
        match part {
            RadioPart::Container => self
                .container
                .set_from_str(key, &value.into(), state.into()),
            RadioPart::Radio => self.radio.set_from_str(key, &value.into(), state),
            RadioPart::Extra => self.extra.set_from_str(key, &value.into(), state.into()),
        }
    }

    fn sync_slot(&mut self, state: Self::State, part: Self::Part) -> () {
        match part {
            RadioPart::Container => self.container.sync(state.into()),
            RadioPart::Radio => self.radio.sync(state),
            RadioPart::Extra => self.extra.sync(state.into()),
        }
    }
}

impl BasicProp for RadioBasicProp {
    type State = RadioState;

    type Colors = RadioColors;

    fn from_state(theme: crate::themes::Theme, state: Self::State) -> Self {
        Self {
            container: Self::default_container(theme, state),
            radio: Self::default_radio(theme, state),
            extra: Self::default_extra(theme, state),
        }
    }

    fn state_colors(theme: crate::themes::Theme, state: Self::State) -> Self::Colors {
        RadioPartProp::state_colors(theme, state)
    }

    fn len() -> usize {
        RadioPartProp::len() + ViewBasicProp::len() + LabelBasicProp::len()
    }

    fn set_from_str(
        &mut self,
        _key: &str,
        _value: &makepad_widgets::LiveValue,
        _state: Self::State,
    ) -> () {
        ()
    }

    fn sync(&mut self, state: Self::State) -> () {
        self.container.sync(state.into());
        self.radio.sync(state);
        self.extra.sync(state.into());
    }

    fn live_props() -> LiveProps {
        vec![
            (live_id!(container), ViewBasicProp::live_props().into()),
            (live_id!(radio), RadioPartProp::live_props().into()),
            (live_id!(extra), ViewBasicProp::live_props().into()),
        ]
    }

    fn walk(&self) -> makepad_widgets::Walk {
        self.container.walk()
    }
    fn layout(&self) -> Layout {
        self.container.layout()
    }
}

impl TryFrom<(&Item, RadioState)> for RadioBasicProp {
    type Error = Error;

    fn try_from((value, state): (&Item, RadioState)) -> Result<Self, Self::Error> {
        let inline_table = value.as_inline_table().ok_or(Error::ThemeStyleParse(
            "[component.radio.$part] should be an inline table".to_string(),
        ))?;
        let container = get_from_itable(
            inline_table,
            CONTAINER,
            || Ok(Self::default_container(Theme::default(), state)),
            |v| (v, state.into()).try_into(),
        )?;
        let radio = get_from_itable(
            inline_table,
            RADIO,
            || Ok(Self::default_radio(Theme::default(), state)),
            |v| (v, state).try_into(),
        )?;
        let extra = get_from_itable(
            inline_table,
            EXTRA,
            || Ok(Self::default_extra(Theme::default(), state)),
            |v| (v, state.into()).try_into(),
        )?;

        Ok(Self {
            container,
            radio,
            extra,
        })
    }
}

impl RadioBasicProp {
    pub fn default_container(theme: Theme, state: RadioState) -> ViewBasicProp {
        let mut container = ViewBasicProp::from_state(theme, state.into());
        container.set_height(Size::Fit);
        container.set_width(Size::Fit);
        container.set_flow(Flow::Right);
        container.set_background_visible(false);
        container.set_align(Align::from_f64(0.5));
        container.set_cursor(MouseCursor::Hand);
        container
    }
    pub fn default_extra(theme: Theme, state: RadioState) -> ViewBasicProp {
        let mut extra = Self::default_container(theme, state);
        extra.set_padding(Padding::from_f64(0.0));
        extra
    }

    pub fn default_radio(theme: Theme, state: RadioState) -> RadioPartProp {
        RadioPartProp::from_state(theme, state)
    }
}

basic_prop_interconvert! {
    RadioPartProp {
        state = RadioState;
        {
            background_color => BACKGROUND_COLOR, |v| v.try_into(),
            stroke_color => STROKE_COLOR, |v| v.try_into(),
            border_color => BORDER_COLOR, |v| v.try_into()
        };
        {
            size: f32 => SIZE, 20.0, |v| v.to_f32(),
            background_visible: bool => BACKGROUND_VISIBLE, true, |v| v.to_bool(),
            border_width: f32 => BORDER_WIDTH, 1.0, |v| v.to_f32(),
            mode: ActiveMode => MODE, ActiveMode::Round, |v| v.try_into(),
            margin: Margin => MARGIN, Margin::from_f64(0.0), |v| v.to_margin(margin),
            abs_pos: AbsPos => ABS_POS, None, |v| Ok(v.to_dvec2().map_or(None, |v| Some(v))),
            cursor: MouseCursor => CURSOR, MouseCursor::Hand, |v| v.to_cursor()
        }
    }, "[component.radio.radio] should be an inline table"
}

component_colors! {
    RadioColors {
        colors = (Color, Color, Color);
        background_color, stroke_color, border_color
    }
}

impl BasicProp for RadioPartProp {
    type State = RadioState;
    /// (background_color, stroke_color, border_color)
    type Colors = RadioColors;

    fn from_state(theme: Theme, state: Self::State) -> Self {
        let RadioColors {
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
            size: 20.0,
            background_color: background_color.into(),
            stroke_color: stroke_color.into(),
            border_color: border_color.into(),
            background_visible: true,
            border_width: 1.0,
            mode: ActiveMode::Round,
            margin: Margin::from_f64(0.0),
            abs_pos: None,
            cursor,
        }
    }

    state_colors! {
        (bg_level, stroke_level, border_level),
        RadioState::Basic => (200, 200, 400),
        RadioState::Hover => (200, 200, 400),
        RadioState::Active => (200, 500, 500),
        RadioState::Disabled => (100, 100, 300)
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
                self.size = f32::from_live_value(value).unwrap_or(20.0);
            }
            BACKGROUND_VISIBLE => {
                self.background_visible = bool::from_live_value(value).unwrap_or(true);
            }
            BORDER_WIDTH => {
                self.border_width = f32::from_live_value(value).unwrap_or(1.0);
            }
            MODE => {
                self.mode = ActiveMode::from_live_value(value).unwrap_or(ActiveMode::Round);
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
        let RadioColors {
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
            (live_id!(mode), None.into()),
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
            width: Size::Fixed(self.size as f64),
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
    RadioState {
        Basic => BASIC,
        Hover => HOVER,
        Active => ACTIVE,
        Disabled => DISABLED
    },
    _ => RadioState::Basic
}

impl ComponentState for RadioState {
    fn is_disabled(&self) -> bool {
        matches!(self, RadioState::Disabled)
    }
}

impl From<RadioState> for LabelState {
    fn from(value: RadioState) -> Self {
        match value {
            RadioState::Basic | RadioState::Hover | RadioState::Active => LabelState::Basic,
            RadioState::Disabled => LabelState::Disabled,
        }
    }
}

impl From<RadioState> for ViewState {
    fn from(value: RadioState) -> Self {
        match value {
            RadioState::Basic => ViewState::Basic,
            RadioState::Hover => ViewState::Hover,
            RadioState::Active => ViewState::Pressed,
            RadioState::Disabled => ViewState::Disabled,
        }
    }
}
impl From<ViewState> for RadioState {
    fn from(value: ViewState) -> Self {
        match value {
            ViewState::Basic => RadioState::Basic,
            ViewState::Hover => RadioState::Hover,
            ViewState::Pressed => RadioState::Active,
            ViewState::Disabled => RadioState::Disabled,
        }
    }
}

component_part! {
    RadioPart {
        Container => container => CONTAINER,
        Radio => radio => RADIO,
        Extra => extra => EXTRA
    }, RadioState
}
