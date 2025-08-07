use crate::{
    component_part, component_state, components::{
        label::{LabelBasicProp, LabelState},
        live_props::LiveProps,
        traits::{BasicProp, ComponentState, Part, Prop, SlotBasicProp, SlotProp},
        view::{ViewBasicProp, ViewState},
    }, error::Error, get_get_mut, prop::{
        manuel::{
            ABS_POS, ACTIVE, BACKGROUND_COLOR, BACKGROUND_VISIBLE, BASIC, BORDER_COLOR,
            BORDER_WIDTH, CHECKBOX, CONTAINER, CURSOR, DISABLED, EXTRA, HOVER, MARGIN, MODE, SIZE,
            STROKE_COLOR, THEME,
        },
        traits::{FromLiveColor, FromLiveValue, NewFrom},
        ActiveMode, ApplySlotMapImpl,
    }, state_colors, themes::{Color, Theme, TomlValueTo}, try_from_toml_item, utils::get_from_itable
};
use makepad_widgets::*;
use toml_edit::{Item, Value};

#[derive(Debug, Clone, Live, LiveHook, LiveRegister)]
#[live_ignore]
pub struct CheckboxProp {
    #[live(CheckboxBasicProp::default())]
    pub basic: CheckboxBasicProp,
    #[live(CheckboxBasicProp::from_state(Theme::default(), CheckboxState::Hover))]
    pub hover: CheckboxBasicProp,
    #[live(CheckboxBasicProp::from_state(Theme::default(), CheckboxState::Active))]
    pub active: CheckboxBasicProp,
    #[live(CheckboxBasicProp::from_state(Theme::default(), CheckboxState::Disabled))]
    pub disabled: CheckboxBasicProp,
}

impl Default for CheckboxProp {
    fn default() -> Self {
        Self {
            basic: CheckboxBasicProp::default(),
            hover: CheckboxBasicProp::from_state(Theme::default(), CheckboxState::Hover),
            active: CheckboxBasicProp::from_state(Theme::default(), CheckboxState::Active),
            disabled: CheckboxBasicProp::from_state(Theme::default(), CheckboxState::Disabled),
        }
    }
}

impl SlotProp for CheckboxProp {
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

impl Prop for CheckboxProp {
    type State = CheckboxState;

    type Basic = CheckboxBasicProp;

    get_get_mut! {
        CheckboxState::Basic => basic,
        CheckboxState::Hover => hover,
        CheckboxState::Active => active,
        CheckboxState::Disabled => disabled
    }

    fn len() -> usize {
        4 * CheckboxBasicProp::len()
    }

    fn sync(&mut self, _map: &crate::prop::ApplyStateMap<Self::State>) -> ()
    where
        Self::State: Eq + std::hash::Hash + Copy,
    {
        ()
    }
}

try_from_toml_item! {
    CheckboxProp {
        basic => BASIC, CheckboxBasicProp::default(),|v| (v, CheckboxState::Basic).try_into(),
        hover => HOVER, CheckboxBasicProp::from_state(Theme::default(), CheckboxState::Hover),|v| (v, CheckboxState::Hover).try_into(),
        active => ACTIVE, CheckboxBasicProp::from_state(Theme::default(), CheckboxState::Active),|v| (v, CheckboxState::Active).try_into(),
        disabled => DISABLED, CheckboxBasicProp::from_state(Theme::default(), CheckboxState::Disabled),|v| (v, CheckboxState::Disabled).try_into()
    }, "[component.checkbox] should be a table"
}

#[derive(Debug, Clone, Live, LiveHook, LiveRegister)]
#[live_ignore]
pub struct CheckboxBasicProp {
    #[live(Self::default_container(Theme::default(), CheckboxState::Basic))]
    pub container: ViewBasicProp,
    #[live(Self::default_checkbox(Theme::default(), CheckboxState::Basic))]
    pub checkbox: CheckboxPartProp,
    #[live(Self::default_extra(Theme::default(), CheckboxState::Basic))]
    pub extra: ViewBasicProp,
}

impl Default for CheckboxBasicProp {
    fn default() -> Self {
        Self::from_state(Theme::default(), CheckboxState::Basic)
    }
}

impl SlotBasicProp for CheckboxBasicProp {
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

impl BasicProp for CheckboxBasicProp {
    type State = CheckboxState;

    type Colors = (Color, Color, Color);

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
        CheckboxPartProp::len() + ViewBasicProp::len() + LabelBasicProp::len()
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
            (live_id!(container), ViewBasicProp::live_props().into()),
            (live_id!(checkbox), CheckboxPartProp::live_props().into()),
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

impl TryFrom<(&Item, CheckboxState)> for CheckboxBasicProp {
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

impl CheckboxBasicProp {
    pub fn default_container(theme: Theme, state: CheckboxState) -> ViewBasicProp {
        let mut container = ViewBasicProp::from_state(theme, state.into());
        container.set_height(Size::Fit);
        container.set_width(Size::Fit);
        container.set_flow(Flow::Right);
        container.set_background_visible(false);
        container.set_align(Align::from_f64(0.5));
        container.set_cursor(MouseCursor::Hand);
        container
    }
    pub fn default_extra(theme: Theme, state: CheckboxState) -> ViewBasicProp {
        Self::default_container(theme, state)
    }

    pub fn default_checkbox(theme: Theme, state: CheckboxState) -> CheckboxPartProp {
        CheckboxPartProp::from_state(theme, state)
    }
}

#[derive(Debug, Clone, Live, LiveHook, LiveRegister)]
#[live_ignore]
pub struct CheckboxPartProp {
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

impl TryFrom<(&Value, CheckboxState)> for CheckboxPartProp {
    type Error = Error;

    fn try_from((value, state): (&Value, CheckboxState)) -> Result<Self, Self::Error> {
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

impl BasicProp for CheckboxPartProp {
    type State = CheckboxState;
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

impl Default for CheckboxPartProp {
    fn default() -> Self {
        Self::from_state(Theme::default(), CheckboxState::Basic)
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
