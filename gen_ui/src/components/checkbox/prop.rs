use crate::{
    basic_prop_interconvert, component_colors, component_part, component_state, components::{
        label::{LabelBasicStyle, LabelState},
        live_props::LiveProps,
        traits::{BasicStyle, ComponentState, Part, Style, SlotBasicStyle, SlotStyle},
        view::{ViewBasicStyle, ViewState},
    }, error::Error, from_prop_to_toml, get_get_mut, prop::{
        manuel::{
            ABS_POS, ACTIVE, BACKGROUND_COLOR, BACKGROUND_VISIBLE, BASIC, BORDER_COLOR,
            BORDER_WIDTH, CHECKBOX, CONTAINER, CURSOR, DISABLED, EXTRA, HOVER, MARGIN, MODE, SIZE,
            STROKE_COLOR, THEME,
        },
        traits::{AbsPos, FromLiveColor, FromLiveValue, NewFrom, ToColor, ToTomlValue},
        ActiveMode, ApplySlotMapImpl,
    }, prop_interconvert, state_colors, themes::{Color, Theme, TomlValueTo}, utils::get_from_itable
};
use makepad_widgets::*;
use toml_edit::{Item};

prop_interconvert! {
    CheckboxProp {
        basic_prop = CheckboxBasicStyle;
        basic => BASIC, CheckboxBasicStyle::default(),|v| (v, CheckboxState::Basic).try_into(),
        hover => HOVER, CheckboxBasicStyle::from_state(Theme::default(), CheckboxState::Hover),|v| (v, CheckboxState::Hover).try_into(),
        active => ACTIVE, CheckboxBasicStyle::from_state(Theme::default(), CheckboxState::Active),|v| (v, CheckboxState::Active).try_into(),
        disabled => DISABLED, CheckboxBasicStyle::from_state(Theme::default(), CheckboxState::Disabled),|v| (v, CheckboxState::Disabled).try_into()
    }, "[component.checkbox] should be a table"
}


impl SlotStyle for CheckboxProp {
    type Part = CheckboxPart;

    fn sync_slot(&mut self, map: &crate::prop::ApplySlotMap<Self::State, Self::Part>) -> () {
        map.sync(
            &mut self.basic,
            CheckboxState::Basic,
            [
                (CheckboxState::Hover, &mut self.hover),
                (CheckboxState::Active, &mut self.active),
                (CheckboxState::Disabled, &mut self.disabled),
            ],
            [
                CheckboxPart::Container,
                CheckboxPart::Checkbox,
                CheckboxPart::Extra,
            ],
        );
    }
}

impl Style for CheckboxProp {
    type State = CheckboxState;

    type Basic = CheckboxBasicStyle;

    get_get_mut! {
        CheckboxState::Basic => basic,
        CheckboxState::Hover => hover,
        CheckboxState::Active => active,
        CheckboxState::Disabled => disabled
    }

    fn len() -> usize {
        4 * CheckboxBasicStyle::len()
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
pub struct CheckboxBasicStyle {
    #[live(Self::default_container(Theme::default(), CheckboxState::Basic))]
    pub container: ViewBasicStyle,
    #[live(Self::default_checkbox(Theme::default(), CheckboxState::Basic))]
    pub checkbox: CheckboxPartProp,
    #[live(Self::default_extra(Theme::default(), CheckboxState::Basic))]
    pub extra: ViewBasicStyle,
}

impl Default for CheckboxBasicStyle {
    fn default() -> Self {
        Self::from_state(Theme::default(), CheckboxState::Basic)
    }
}

from_prop_to_toml!{
    CheckboxBasicStyle {
        container => CONTAINER,
        checkbox => CHECKBOX,
        extra => EXTRA
    }
}

impl SlotBasicStyle for CheckboxBasicStyle {
    type Part = CheckboxPart;

    fn set_from_str_slot(
        &mut self,
        key: &str,
        value: &crate::prop::Applys,
        state: Self::State,
        part: Self::Part,
    ) -> () {
        match part {
            CheckboxPart::Container => {
                self.container
                    .set_from_str(key, &value.into(), state.into())
            }
            CheckboxPart::Checkbox => self.checkbox.set_from_str(key, &value.into(), state),
            CheckboxPart::Extra => self.extra.set_from_str(key, &value.into(), state.into()),
        }
    }

    fn sync_slot(&mut self, state: Self::State, part: Self::Part) -> () {
        match part {
            CheckboxPart::Container => self.container.sync(state.into()),
            CheckboxPart::Checkbox => self.checkbox.sync(state),
            CheckboxPart::Extra => self.extra.sync(state.into()),
        }
    }
}

impl BasicStyle for CheckboxBasicStyle {
    type State = CheckboxState;

    type Colors = CheckboxColors;

    fn from_state(theme: crate::themes::Theme, state: Self::State) -> Self {
        Self {
            container: Self::default_container(theme, state),
            checkbox: Self::default_checkbox(theme, state),
            extra: Self::default_extra(theme, state),
        }
    }

    fn state_colors(theme: crate::themes::Theme, state: Self::State) -> Self::Colors {
        CheckboxPartProp::state_colors(theme, state)
    }

    fn len() -> usize {
        CheckboxPartProp::len() + ViewBasicStyle::len() + LabelBasicStyle::len()
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
        self.checkbox.sync(state);
        self.extra.sync(state.into());
    }

    fn live_props() -> LiveProps {
        vec![
            (live_id!(container), ViewBasicStyle::live_props().into()),
            (live_id!(checkbox), CheckboxPartProp::live_props().into()),
            (live_id!(extra), ViewBasicStyle::live_props().into()),
        ]
    }

    fn walk(&self) -> makepad_widgets::Walk {
        self.container.walk()
    }
    fn layout(&self) -> Layout {
        self.container.layout()
    }
}

impl TryFrom<(&Item, CheckboxState)> for CheckboxBasicStyle {
    type Error = Error;

    fn try_from((value, state): (&Item, CheckboxState)) -> Result<Self, Self::Error> {
        let inline_table = value.as_inline_table().ok_or(Error::ThemeStyleParse(
            "[component.checkbox.$part] should be an inline table".to_string(),
        ))?;
        let container = get_from_itable(
            inline_table,
            CONTAINER,
            || Ok(Self::default_container(Theme::default(), state)),
            |v| (v, state.into()).try_into(),
        )?;
        let checkbox = get_from_itable(
            inline_table,
            CHECKBOX,
            || Ok(Self::default_checkbox(Theme::default(), state)),
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
            checkbox,
            extra,
        })
    }
}

impl CheckboxBasicStyle {
    pub fn default_container(theme: Theme, state: CheckboxState) -> ViewBasicStyle {
        let mut container = ViewBasicStyle::from_state(theme, state.into());
        container.set_height(Size::Fit);
        container.set_width(Size::Fit);
        container.set_flow(Flow::Right);
        container.set_background_visible(false);
        container.set_align(Align::from_f64(0.5));
        container.set_cursor(MouseCursor::Hand);
        container
    }
    pub fn default_extra(theme: Theme, state: CheckboxState) -> ViewBasicStyle {
        let mut extra = Self::default_container(theme, state);
        extra.set_padding(Padding::from_f64(0.0));
        extra
    }

    pub fn default_checkbox(theme: Theme, state: CheckboxState) -> CheckboxPartProp {
        CheckboxPartProp::from_state(theme, state)
    }
}

basic_prop_interconvert! {
    CheckboxPartProp {
        state = CheckboxState;
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
    }, "[component.checkbox.$part] should be a inline table"
}

component_colors!{
    CheckboxColors {
        colors = (Color, Color, Color);
        background_color, stroke_color, border_color
    }
}

impl BasicStyle for CheckboxPartProp {
    type State = CheckboxState;
    /// (background_color, stroke_color, border_color)
    type Colors = CheckboxColors;

    fn from_state(theme: Theme, state: Self::State) -> Self {
        let CheckboxColors { background_color, stroke_color, border_color } = Self::state_colors(theme, state);
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
        CheckboxState::Basic => (200, 200, 400),
        CheckboxState::Hover => (200, 200, 400),
        CheckboxState::Active => (500, 200, 500),
        CheckboxState::Disabled => (100, 100, 300)
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
                self.stroke_color = Vec4::from_live_color(value).unwrap_or(colors.stroke_color.into());
            }
            BORDER_COLOR => {
                let colors = Self::state_colors(self.theme, state);
                self.border_color = Vec4::from_live_color(value).unwrap_or(colors.border_color.into());
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
        let CheckboxColors { background_color, stroke_color, border_color } = Self::state_colors(self.theme, state);
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
    CheckboxState {
        Basic => BASIC,
        Hover => HOVER,
        Active => ACTIVE,
        Disabled => DISABLED
    },
    _ => CheckboxState::Basic
}

impl ComponentState for CheckboxState {
    fn is_disabled(&self) -> bool {
        matches!(self, CheckboxState::Disabled)
    }
}

impl From<CheckboxState> for LabelState {
    fn from(value: CheckboxState) -> Self {
        match value {
            CheckboxState::Basic | CheckboxState::Hover | CheckboxState::Active => {
                LabelState::Basic
            }

            CheckboxState::Disabled => LabelState::Disabled,
        }
    }
}

impl From<CheckboxState> for ViewState {
    fn from(value: CheckboxState) -> Self {
        match value {
            CheckboxState::Basic => ViewState::Basic,
            CheckboxState::Hover => ViewState::Hover,
            CheckboxState::Active => ViewState::Pressed,
            CheckboxState::Disabled => ViewState::Disabled,
        }
    }
}
impl From<ViewState> for CheckboxState {
    fn from(value: ViewState) -> Self {
        match value {
            ViewState::Basic => CheckboxState::Basic,
            ViewState::Hover => CheckboxState::Hover,
            ViewState::Pressed => CheckboxState::Active,
            ViewState::Disabled => CheckboxState::Disabled,
        }
    }
}


component_part! {
    CheckboxPart {
        Container => container => CONTAINER,
        Checkbox => checkbox => CHECKBOX,
        Extra => extra => EXTRA
    }, CheckboxState
}
