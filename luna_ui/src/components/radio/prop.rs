use crate::{
    component_state,
    components::{
        label::{LabelBasicProp, LabelState},
        traits::{BasicProp, ComponentState, Part, Prop, SlotBasicProp, SlotProp},
        view::{ViewBasicProp, ViewState},
    },
    error::Error,
    prop::{
        manuel::{
            ABS_POS, ACTIVE, BACKGROUND_COLOR, BACKGROUND_VISIBLE, BASIC, BORDER_COLOR,
            BORDER_WIDTH, CONTAINER, CURSOR, DISABLED, HOVER, MARGIN, MODE, SIZE, STROKE_COLOR,
            THEME,
        },
        traits::{FromLiveColor, FromLiveValue, NewFrom},
        ActiveMode, ApplySlotMapImpl,
    },
    themes::{Color, Theme, TomlValueTo},
    try_from_toml_item,
    utils::get_from_itable,
};
use makepad_widgets::*;
use toml_edit::{Item, Value};

#[derive(Debug, Clone, Live, LiveHook, LiveRegister)]
#[live_ignore]
pub struct RadioProp {
    #[live(RadioBasicProp::default())]
    pub basic: RadioBasicProp,
    #[live(RadioBasicProp::from_state(Theme::default(), RadioState::Hover))]
    pub hover: RadioBasicProp,
    #[live(RadioBasicProp::from_state(Theme::default(), RadioState::Active))]
    pub active: RadioBasicProp,
    #[live(RadioBasicProp::from_state(Theme::default(), RadioState::Disabled))]
    pub disabled: RadioBasicProp,
}

impl Default for RadioProp {
    fn default() -> Self {
        Self {
            basic: RadioBasicProp::default(),
            hover: RadioBasicProp::from_state(Theme::default(), RadioState::Hover),
            active: RadioBasicProp::from_state(Theme::default(), RadioState::Active),
            disabled: RadioBasicProp::from_state(Theme::default(), RadioState::Disabled),
        }
    }
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
            [RadioPart::Container, RadioPart::Radio, RadioPart::Label],
        );
    }
}

impl Prop for RadioProp {
    type State = RadioState;

    type Basic = RadioBasicProp;

    fn get(&self, state: Self::State) -> &Self::Basic {
        match state {
            RadioState::Basic => &self.basic,
            RadioState::Hover => &self.hover,
            RadioState::Active => &self.active,
            RadioState::Disabled => &self.disabled,
        }
    }

    fn get_mut(&mut self, state: Self::State) -> &mut Self::Basic {
        match state {
            RadioState::Basic => &mut self.basic,
            RadioState::Hover => &mut self.hover,
            RadioState::Active => &mut self.active,
            RadioState::Disabled => &mut self.disabled,
        }
    }

    fn len() -> usize {
        4 * RadioBasicProp::len()
    }

    fn sync(&mut self, map: &crate::prop::ApplyStateMap<Self::State>) -> ()
    where
        Self::State: Eq + std::hash::Hash + Copy,
    {
        ()
    }
}

try_from_toml_item! {
    RadioProp {
        basic => BASIC, RadioBasicProp::default(),|v| (v, RadioState::Basic).try_into(),
        hover => HOVER, RadioBasicProp::from_state(Theme::default(), RadioState::Hover),|v| (v, RadioState::Hover).try_into(),
        active => ACTIVE, RadioBasicProp::from_state(Theme::default(), RadioState::Active),|v| (v, RadioState::Active).try_into(),
        disabled => DISABLED, RadioBasicProp::from_state(Theme::default(), RadioState::Disabled),|v| (v, RadioState::Disabled).try_into()
    }, "[component.radio] should be a table"
}

#[derive(Debug, Clone, Live, LiveHook, LiveRegister)]
#[live_ignore]
pub struct RadioBasicProp {
    #[live(Self::default_container(Theme::default(), RadioState::Basic))]
    pub container: ViewBasicProp,
    #[live(Self::default_radio(Theme::default(), RadioState::Basic))]
    pub radio: RadioPartProp,
    #[live(Self::default_label(Theme::default(), RadioState::Basic))]
    pub label: LabelBasicProp,
}

impl Default for RadioBasicProp {
    fn default() -> Self {
        Self::from_state(Theme::default(), RadioState::Basic)
    }
}

impl SlotBasicProp for RadioBasicProp {
    type Part = RadioPart;

    fn set_from_str_slot(
        &mut self,
        key: &str,
        value: &LiveValue,
        state: Self::State,
        part: Self::Part,
    ) -> () {
        match part {
            RadioPart::Container => self.container.set_from_str(key, value, state.into()),
            RadioPart::Radio => self.radio.set_from_str(key, value, state),
            RadioPart::Label => self.label.set_from_str(key, value, state.into()),
        }
    }

    fn sync_slot(&mut self, state: Self::State, part: Self::Part) -> () {
        match part {
            RadioPart::Container => self.container.sync(state.into()),
            RadioPart::Radio => self.radio.sync(state),
            RadioPart::Label => self.label.sync(state.into()),
        }
    }
}

impl BasicProp for RadioBasicProp {
    type State = RadioState;

    type Colors = (Color, Color, Color);

    fn from_state(theme: crate::themes::Theme, state: Self::State) -> Self {
        Self {
            container: Self::default_container(theme, state),
            radio: Self::default_radio(theme, state),
            label: Self::default_label(theme, state),
        }
    }

    fn state_colors(theme: crate::themes::Theme, state: Self::State) -> Self::Colors {
        RadioPartProp::state_colors(theme, state)
    }

    fn len() -> usize {
        3 * (RadioPartProp::len() + ViewBasicProp::len() + LabelBasicProp::len())
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
        self.label.sync(state.into());
    }

    fn live_props() -> Vec<(
        makepad_widgets::LiveId,
        Option<Vec<makepad_widgets::LiveId>>,
    )> {
        vec![]
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
            ACTIVE,
            || Ok(Self::default_radio(Theme::default(), state)),
            |v| (v, state).try_into(),
        )?;
        let label = get_from_itable(
            inline_table,
            BASIC,
            || Ok(Self::default_label(Theme::default(), state)),
            |v| (v, state.into()).try_into(),
        )?;

        Ok(Self {
            container,
            radio,
            label,
        })
    }
}

impl RadioBasicProp {
    pub fn default_container(theme: Theme, state: RadioState) -> ViewBasicProp {
        let mut container = ViewBasicProp::from_state(theme, state.into());
        container.set_height(Size::Fit);
        container.set_width(Size::Fixed(200.0));
        container.set_flow(Flow::Right);
        container.set_background_visible(false);
        container
    }
    pub fn default_label(theme: Theme, state: RadioState) -> LabelBasicProp {
        LabelBasicProp::from_state(theme, state.into())
    }

    pub fn default_radio(theme: Theme, state: RadioState) -> RadioPartProp {
        RadioPartProp::from_state(theme, state)
    }
}

#[derive(Debug, Clone, Live, LiveHook, LiveRegister)]
#[live_ignore]
pub struct RadioPartProp {
    #[live(Theme::default())]
    pub theme: Theme,
    #[live(16.0)]
    pub size: f32,
    #[live]
    pub background_color: Vec4,
    #[live]
    pub border_color: Vec4,
    #[live]
    pub stroke_color: Vec4,
    #[live(true)]
    pub background_visible: bool,
    #[live(1.0)]
    pub border_width: f32,
    #[live(ActiveMode::Round)]
    pub mode: ActiveMode,
    #[live]
    pub margin: Margin,
    #[live(None)]
    pub abs_pos: Option<DVec2>,
    #[live(MouseCursor::Hand)]
    pub cursor: MouseCursor,
}

impl TryFrom<(&Value, RadioState)> for RadioPartProp {
    type Error = Error;

    fn try_from((value, state): (&Value, RadioState)) -> Result<Self, Self::Error> {
        let inline_table = value.as_inline_table().ok_or(Error::ThemeStyleParse(
            "[component.radio.radio] should be an inline table".to_string(),
        ))?;

        let theme = Theme::default();
        let theme = get_from_itable(inline_table, THEME, || Ok(theme), |v| v.try_into())?;
        let (background_color, stroke_color, border_color) = Self::state_colors(theme, state);
        let background_color = get_from_itable(
            inline_table,
            BACKGROUND_COLOR,
            || Ok(background_color),
            |v| v.try_into(),
        )?
        .into();
        let stroke_color = get_from_itable(
            inline_table,
            STROKE_COLOR,
            || Ok(stroke_color),
            |v| v.try_into(),
        )?
        .into();
        let border_color = get_from_itable(
            inline_table,
            BORDER_COLOR,
            || Ok(border_color),
            |v| v.try_into(),
        )?
        .into();
        let size = get_from_itable(inline_table, SIZE, || Ok(16.0), |item| item.to_f32())?;
        let background_visible = get_from_itable(
            inline_table,
            BACKGROUND_VISIBLE,
            || Ok(true),
            |item| item.to_bool(),
        )?;
        let border_width =
            get_from_itable(inline_table, BORDER_WIDTH, || Ok(1.0), |item| item.to_f32())?;
        let mode = get_from_itable(
            inline_table,
            MODE,
            || Ok(ActiveMode::Round),
            |item| item.try_into(),
        )?;
        let margin = Margin::from_f64(6.0);
        let margin = get_from_itable(inline_table, MARGIN, || Ok(margin), |v| v.to_margin(margin))?;
        let abs_pos = get_from_itable(
            inline_table,
            ABS_POS,
            || Ok(None),
            |v| v.to_dvec2().map(Some),
        )?;
        let cursor = if state.is_disabled() {
            MouseCursor::NotAllowed
        } else {
            MouseCursor::Hand
        };
        let cursor = get_from_itable(inline_table, "cursor", || Ok(cursor), |v| v.to_cursor())?;

        Ok(Self {
            theme,
            size,
            background_color,
            stroke_color,
            border_color,
            background_visible,
            border_width,
            mode,
            margin,
            abs_pos,
            cursor,
        })
    }
}

impl BasicProp for RadioPartProp {
    type State = RadioState;
    /// (background_color, stroke_color, border_color)
    type Colors = (Color, Color, Color);

    fn from_state(theme: Theme, state: Self::State) -> Self {
        let (backgroud_color, stroke_color, border_color) = Self::state_colors(theme, state);
        let cursor = if state.is_disabled() {
            MouseCursor::NotAllowed
        } else {
            MouseCursor::Hand
        };
        Self {
            theme,
            size: 16.0,
            background_color: backgroud_color.into(),
            stroke_color: stroke_color.into(),
            border_color: border_color.into(),
            background_visible: true,
            border_width: 1.0,
            mode: ActiveMode::Round,
            margin: Margin::from_f64(6.0),
            abs_pos: None,
            cursor,
        }
    }

    fn state_colors(theme: Theme, state: Self::State) -> Self::Colors {
        let (bg_level, stroke_level, border_level) = match state {
            RadioState::Basic => (200, 200, 400),
            RadioState::Hover => (200, 200, 400),
            RadioState::Active => (500, 200, 500),
            RadioState::Disabled => (300, 300, 200),
        };

        match theme {
            Theme::Dark => (
                Theme::Dark.color(bg_level),
                Theme::Dark.color(stroke_level),
                Theme::Dark.color(border_level),
            ),
            Theme::Primary => (
                Theme::Primary.color(bg_level),
                Theme::Primary.color(stroke_level),
                Theme::Primary.color(border_level),
            ),
            Theme::Error => (
                Theme::Error.color(bg_level),
                Theme::Error.color(stroke_level),
                Theme::Error.color(border_level),
            ),
            Theme::Warning => (
                Theme::Warning.color(bg_level),
                Theme::Warning.color(stroke_level),
                Theme::Warning.color(border_level),
            ),
            Theme::Success => (
                Theme::Success.color(bg_level),
                Theme::Success.color(stroke_level),
                Theme::Success.color(border_level),
            ),
            Theme::Info => (
                Theme::Info.color(bg_level),
                Theme::Info.color(stroke_level),
                Theme::Info.color(border_level),
            ),
        }
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
                let (background_color, _, _) = Self::state_colors(self.theme, state);
                self.background_color =
                    Vec4::from_live_color(value).unwrap_or(background_color.into());
            }
            STROKE_COLOR => {
                let (_, stroke_color, _) = Self::state_colors(self.theme, state);
                self.stroke_color = Vec4::from_live_color(value).unwrap_or(stroke_color.into());
            }
            BORDER_COLOR => {
                let (_, _, border_color) = Self::state_colors(self.theme, state);
                self.border_color = Vec4::from_live_color(value).unwrap_or(border_color.into());
            }
            SIZE => {
                self.size = f32::from_live_value(value).unwrap_or(16.0);
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
                self.margin = Margin::from_live_value(value).unwrap_or(Margin::from_f64(6.0));
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
        let (background_color, stroke_color, border_color) = Self::state_colors(self.theme, state);
        self.background_color = background_color.into();
        self.stroke_color = stroke_color.into();
        self.border_color = border_color.into();
    }

    fn live_props() -> Vec<(LiveId, Option<Vec<LiveId>>)> {
        vec![
            (live_id!(theme), None),
            (live_id!(size), None),
            (live_id!(background_color), None),
            (live_id!(stroke_color), None),
            (live_id!(border_color), None),
            (live_id!(background_visible), None),
            (live_id!(border_width), None),
            (live_id!(mode), None),
            (
                live_id!(margin),
                Some(vec![
                    live_id!(top),
                    live_id!(bottom),
                    live_id!(left),
                    live_id!(right),
                ]),
            ),
            (live_id!(abs_pos), None),
            (live_id!(cursor), None),
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

impl Default for RadioPartProp {
    fn default() -> Self {
        Self::from_state(Theme::default(), RadioState::Basic)
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RadioPart {
    Container,
    Radio,
    Label,
}

impl Part for RadioPart {
    type State = RadioState;
    fn to_live_id(&self) -> LiveId {
        match self {
            RadioPart::Container => live_id!(container),
            RadioPart::Radio => live_id!(radio),
            RadioPart::Label => live_id!(label),
        }
    }
}
