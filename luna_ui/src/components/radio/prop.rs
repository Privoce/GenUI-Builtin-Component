use crate::{
    component_state,
    components::{
        label::{LabelBasicProp, LabelState},
        traits::{BasicProp, ComponentState, Prop},
        view::{ViewBasicProp, ViewState},
    },
    error::Error,
    prop::{
        manuel::{ABS_POS, ACTIVE, BACKGROUND_COLOR, BACKGROUND_VISIBLE, BASIC, BORDER_COLOR, BORDER_WIDTH, CONTAINER, DISABLED, HOVER, MARGIN, MODE, SIZE, STROKE_COLOR, THEME}, traits::NewFrom, ActiveMode
    },
    themes::{Color, Theme, TomlValueTo},
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

impl Prop for RadioProp {
    type State = RadioState;

    type Basic = RadioBasicProp;

    fn get(&self, state: Self::State) -> &Self::Basic {
        todo!()
    }

    fn get_mut(&mut self, state: Self::State) -> &mut Self::Basic {
        todo!()
    }

    fn len() -> usize {
        todo!()
    }

    fn sync(&mut self, map: &crate::prop::ApplyStateMap<Self::State>) -> ()
    where
        Self::State: Eq + std::hash::Hash + Copy,
    {
        todo!()
    }
}

impl TryFrom<&Item> for RadioProp {
    type Error = Error;

    fn try_from(value: &Item) -> Result<Self, Self::Error> {
        todo!()
    }
}

#[derive(Debug, Clone, Live, LiveHook, LiveRegister)]
#[live_ignore]
pub struct RadioBasicProp {
    #[live]
    pub container: ViewBasicProp,
    #[live]
    pub radio: RadioPartProp,
    #[live]
    pub label: LabelBasicProp,
}

impl Default for RadioBasicProp {
    fn default() -> Self {
        Self::from_state(Theme::default(), RadioState::Basic)
    }
}

impl BasicProp for RadioBasicProp {
    type State = RadioState;

    type Colors = (Color, Color, Color);

    fn from_state(theme: crate::themes::Theme, state: Self::State) -> Self {
        todo!()
    }

    fn state_colors(theme: crate::themes::Theme, state: Self::State) -> Self::Colors {
        todo!()
    }

    fn len() -> usize {
        todo!()
    }

    fn set_from_str(
        &mut self,
        key: &str,
        value: &makepad_widgets::LiveValue,
        state: Self::State,
    ) -> () {
        todo!()
    }

    fn sync(&mut self, state: Self::State) -> () {
        todo!()
    }

    fn live_props() -> Vec<(
        makepad_widgets::LiveId,
        Option<Vec<makepad_widgets::LiveId>>,
    )> {
        todo!()
    }

    fn walk(&self) -> makepad_widgets::Walk {
        todo!()
    }
}

impl TryFrom<(&Value, RadioState)> for RadioBasicProp {
    type Error = Error;

    fn try_from((value, state): (&Value, RadioState)) -> Result<Self, Self::Error> {
        let inline_table = value.as_inline_table().ok_or(Error::ThemeStyleParse(
            "[component.radio.$part] should be an inline table".to_string(),
        ))?;
        let container = get_from_itable(
            inline_table,
            CONTAINER,
            || Ok(ViewBasicProp::default()),
            |v| (v, state.into()).try_into(),
        )?;
        let radio = get_from_itable(
            inline_table,
            ACTIVE,
            || Ok(RadioPartProp::default()),
            |v| (v, state).try_into(),
        )?;
        let label = get_from_itable(
            inline_table,
            BASIC,
            || Ok(LabelBasicProp::default()),
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
    pub fn default_container(state: RadioState) -> ViewBasicProp {
        let mut container = ViewBasicProp::from_state(Theme::default(), state.into());
        
        container
    }
    pub fn default_label(state: RadioState) -> LabelBasicProp {
        LabelBasicProp::from_state(Theme::default(), state.into())
    }
}

#[derive(Debug, Clone, Live, LiveHook, LiveRegister)]
#[live_ignore]
pub struct RadioPartProp {
    #[live]
    pub theme: Theme,
    #[live]
    pub size: f32,
    #[live]
    pub background_color: Vec4,
    #[live]
    pub border_color: Vec4,
    #[live]
    pub stroke_color: Vec4,
    #[live(1.0)]
    pub background_visible: f32,
    #[live(1.0)]
    pub border_width: f32,
    #[live(ActiveMode::Round)]
    pub mode: ActiveMode,
    #[live]
    pub margin: Margin,
    #[live]
    pub abs_pos: Option<DVec2>,
}

impl TryFrom<(&Value, RadioState)> for RadioPartProp {
    type Error = Error;

    fn try_from((value,state): (&Value, RadioState)) -> Result<Self, Self::Error> {
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
            || Ok(1.0),
            |item| item.to_f32(),
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
        })
    }
}

impl BasicProp for RadioPartProp {
    type State = RadioState;
    /// (background_color, stroke_color, border_color)
    type Colors = (Color, Color, Color);

    fn from_state(theme: Theme, state: Self::State) -> Self {
        todo!()
    }

    fn state_colors(theme: Theme, state: Self::State) -> Self::Colors {
        let (bg_level, stroke_level, border_level) = match state {
            RadioState::Basic => (500, 500, 400),
            RadioState::Hover => (400, 400, 300),
            RadioState::Active => (600, 600, 500),
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
        8
    }

    fn set_from_str(&mut self, key: &str, value: &LiveValue, state: Self::State) -> () {
        todo!()
    }

    fn sync(&mut self, state: Self::State) -> () {
        todo!()
    }

    fn live_props() -> Vec<(LiveId, Option<Vec<LiveId>>)> {
        todo!()
    }

    fn walk(&self) -> Walk {
        Walk {
            abs_pos: self.abs_pos,
            margin: self.margin,
            width: Size::Fixed(self.size as f64),
            height: Size::Fixed(self.size as f64),
        }
    }
}

impl Default for  RadioPartProp {
    fn default() -> Self {
        Self { theme: Default::default(), size: Default::default(), background_color: Default::default(), border_color: Default::default(), stroke_color: Default::default(), background_visible: Default::default(), border_width: Default::default(), mode: Default::default(), margin: Default::default(), abs_pos: Default::default() }
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