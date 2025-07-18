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
            BORDER_WIDTH, CHECKBOX, CONTAINER, CURSOR, DISABLED, EXTRA, HOVER, MARGIN, MODE,
            SIZE, STROKE_COLOR, THEME,
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
pub struct SwitchProp {
    #[live(SwitchBasicProp::default())]
    pub basic: SwitchBasicProp,
    #[live(SwitchBasicProp::from_state(Theme::default(), SwitchState::Hover))]
    pub hover: SwitchBasicProp,
    #[live(SwitchBasicProp::from_state(Theme::default(), SwitchState::Active))]
    pub active: SwitchBasicProp,
    #[live(SwitchBasicProp::from_state(Theme::default(), SwitchState::Disabled))]
    pub disabled: SwitchBasicProp,
}

impl Default for SwitchProp {
    fn default() -> Self {
        Self {
            basic: SwitchBasicProp::default(),
            hover: SwitchBasicProp::from_state(Theme::default(), SwitchState::Hover),
            active: SwitchBasicProp::from_state(Theme::default(), SwitchState::Active),
            disabled: SwitchBasicProp::from_state(Theme::default(), SwitchState::Disabled),
        }
    }
}

impl SlotProp for SwitchProp {
    type Part = SwitchPart;

    fn sync_slot(&mut self, map: &crate::prop::ApplySlotMap<Self::State, Self::Part>) -> () {
        map.sync(
            &mut self.basic,
            SwitchState::Basic,
            [
                (SwitchState::Hover, &mut self.hover),
                (SwitchState::Active, &mut self.active),
                (SwitchState::Disabled, &mut self.disabled),
            ],
            [
                SwitchPart::Container,
                SwitchPart::Switch,
                SwitchPart::Extra,
            ],
        );
    }
}

impl Prop for SwitchProp {
    type State = SwitchState;

    type Basic = SwitchBasicProp;

    fn get(&self, state: Self::State) -> &Self::Basic {
        match state {
            SwitchState::Basic => &self.basic,
            SwitchState::Hover => &self.hover,
            SwitchState::Active => &self.active,
            SwitchState::Disabled => &self.disabled,
        }
    }

    fn get_mut(&mut self, state: Self::State) -> &mut Self::Basic {
        match state {
            SwitchState::Basic => &mut self.basic,
            SwitchState::Hover => &mut self.hover,
            SwitchState::Active => &mut self.active,
            SwitchState::Disabled => &mut self.disabled,
        }
    }

    fn len() -> usize {
        4 * SwitchBasicProp::len()
    }

    fn sync(&mut self, _map: &crate::prop::ApplyStateMap<Self::State>) -> ()
    where
        Self::State: Eq + std::hash::Hash + Copy,
    {
        ()
    }
}

try_from_toml_item! {
    SwitchProp {
        basic => BASIC, SwitchBasicProp::default(),|v| (v, SwitchState::Basic).try_into(),
        hover => HOVER, SwitchBasicProp::from_state(Theme::default(), SwitchState::Hover),|v| (v, SwitchState::Hover).try_into(),
        active => ACTIVE, SwitchBasicProp::from_state(Theme::default(), SwitchState::Active),|v| (v, SwitchState::Active).try_into(),
        disabled => DISABLED, SwitchBasicProp::from_state(Theme::default(), SwitchState::Disabled),|v| (v, SwitchState::Disabled).try_into()
    }, "[component.checkbox] should be a table"
}

#[derive(Debug, Clone, Live, LiveHook, LiveRegister)]
#[live_ignore]
pub struct SwitchBasicProp {
    #[live(Self::default_container(Theme::default(), SwitchState::Basic))]
    pub container: ViewBasicProp,
    #[live(Self::default_checkbox(Theme::default(), SwitchState::Basic))]
    pub checkbox: SwitchPartProp,
    #[live(Self::default_extra(Theme::default(), SwitchState::Basic))]
    pub extra: ViewBasicProp,
}

impl Default for SwitchBasicProp {
    fn default() -> Self {
        Self::from_state(Theme::default(), SwitchState::Basic)
    }
}

impl SlotBasicProp for SwitchBasicProp {
    type Part = SwitchPart;

    fn set_from_str_slot(
        &mut self,
        key: &str,
        value: &LiveValue,
        state: Self::State,
        part: Self::Part,
    ) -> () {
        match part {
            SwitchPart::Container => self.container.set_from_str(key, value, state.into()),
            SwitchPart::Switch => self.checkbox.set_from_str(key, value, state),
            SwitchPart::Extra => self.extra.set_from_str(key, value, state.into()),
        }
    }

    fn sync_slot(&mut self, state: Self::State, part: Self::Part) -> () {
        match part {
            SwitchPart::Container => self.container.sync(state.into()),
            SwitchPart::Switch => self.checkbox.sync(state),
            SwitchPart::Extra => self.extra.sync(state.into()),
        }
    }
}

impl BasicProp for SwitchBasicProp {
    type State = SwitchState;

    type Colors = (Color, Color, Color);

    fn from_state(theme: crate::themes::Theme, state: Self::State) -> Self {
        Self {
            container: Self::default_container(theme, state),
            checkbox: Self::default_checkbox(theme, state),
            extra: Self::default_extra(theme, state),
        }
    }

    fn state_colors(theme: crate::themes::Theme, state: Self::State) -> Self::Colors {
        SwitchPartProp::state_colors(theme, state)
    }

    fn len() -> usize {
        3 * (SwitchPartProp::len() + ViewBasicProp::len() + LabelBasicProp::len())
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

impl TryFrom<(&Item, SwitchState)> for SwitchBasicProp {
    type Error = Error;

    fn try_from((value, state): (&Item, SwitchState)) -> Result<Self, Self::Error> {
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

impl SwitchBasicProp {
    pub fn default_container(theme: Theme, state: SwitchState) -> ViewBasicProp {
        let mut container = ViewBasicProp::from_state(theme, state.into());
        container.set_height(Size::Fit);
        container.set_width(Size::Fit);
        container.set_flow(Flow::Right);
        container.set_background_visible(false);
        container.set_align(Align::from_f64(0.5));
        container.set_cursor(MouseCursor::Hand);
        container
    }
    pub fn default_extra(theme: Theme, state: SwitchState) -> ViewBasicProp {
        Self::default_container(theme, state)
    }

    pub fn default_checkbox(theme: Theme, state: SwitchState) -> SwitchPartProp {
        SwitchPartProp::from_state(theme, state)
    }
}

#[derive(Debug, Clone, Live, LiveHook, LiveRegister)]
#[live_ignore]
pub struct SwitchPartProp {
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
    #[live(Margin::from_f64(0.0))]
    pub margin: Margin,
    #[live(None)]
    pub abs_pos: Option<DVec2>,
    #[live(MouseCursor::Hand)]
    pub cursor: MouseCursor,
}

impl TryFrom<(&Value, SwitchState)> for SwitchPartProp {
    type Error = Error;

    fn try_from((value, state): (&Value, SwitchState)) -> Result<Self, Self::Error> {
        let inline_table = value.as_inline_table().ok_or(Error::ThemeStyleParse(
            "[component.checkbox.checkbox] should be an inline table".to_string(),
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
        let margin = Margin::from_f64(0.0);
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

impl BasicProp for SwitchPartProp {
    type State = SwitchState;
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
            margin: Margin::from_f64(0.0),
            abs_pos: None,
            cursor,
        }
    }

    fn state_colors(theme: Theme, state: Self::State) -> Self::Colors {
        let (bg_level, stroke_level, border_level) = match state {
            SwitchState::Basic => (200, 200, 400),
            SwitchState::Hover => (200, 200, 400),
            SwitchState::Active => (500, 200, 500),
            SwitchState::Disabled => (100, 100, 300),
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

impl Default for SwitchPartProp {
    fn default() -> Self {
        Self::from_state(Theme::default(), SwitchState::Basic)
    }
}

component_state! {
    SwitchState {
        Basic => BASIC,
        Hover => HOVER,
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

impl From<SwitchState> for LabelState {
    fn from(value: SwitchState) -> Self {
        match value {
            SwitchState::Basic | SwitchState::Hover | SwitchState::Active => {
                LabelState::Basic
            }

            SwitchState::Disabled => LabelState::Disabled,
        }
    }
}

impl From<SwitchState> for ViewState {
    fn from(value: SwitchState) -> Self {
        match value {
            SwitchState::Basic => ViewState::Basic,
            SwitchState::Hover => ViewState::Hover,
            SwitchState::Active => ViewState::Pressed,
            SwitchState::Disabled => ViewState::Disabled,
        }
    }
}
impl From<ViewState> for SwitchState {
    fn from(value: ViewState) -> Self {
        match value {
            ViewState::Basic => SwitchState::Basic,
            ViewState::Hover => SwitchState::Hover,
            ViewState::Pressed => SwitchState::Active,
            ViewState::Disabled => SwitchState::Disabled,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SwitchPart {
    Container,
    Switch,
    Extra,
}

impl Part for SwitchPart {
    type State = SwitchState;
    fn to_live_id(&self) -> LiveId {
        match self {
            SwitchPart::Container => live_id!(container),
            SwitchPart::Switch => live_id!(checkbox),
            SwitchPart::Extra => live_id!(extra),
        }
    }
}
