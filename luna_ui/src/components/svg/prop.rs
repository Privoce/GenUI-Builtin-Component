use makepad_widgets::*;
use toml_edit::{InlineTable, Item, Value};

use crate::{
    component_state,
    components::{
        live_props::LiveProps,
        traits::{BasicProp, ComponentState, Part, Prop, SlotBasicProp, SlotProp},
        view::{ViewBasicProp, ViewState},
    },
    error::Error,
    prop::{
        manuel::{
            ABS_POS, BACKGROUND_COLOR, BASIC, COLOR, CONTAINER, CURSOR, DISABLED, HEIGHT, HOVER,
            MARGIN, PRESSED, SVG, THEME, WIDTH,
        },
        traits::{FromLiveColor, FromLiveValue, NewFrom},
        ApplySlotMapImpl, ApplyStateMapImpl,
    },
    themes::{Color, Theme, TomlValueTo},
    try_from_toml_item,
    utils::get_from_itable,
};

#[derive(Debug, Clone, Live, LiveHook, LiveRegister)]
#[live_ignore]
pub struct SvgProp {
    #[live(SvgBasicProp::default())]
    pub basic: SvgBasicProp,
    #[live(SvgBasicProp::from_state(Theme::default(), SvgState::Hover))]
    pub hover: SvgBasicProp,
    #[live(SvgBasicProp::from_state(Theme::default(), SvgState::Pressed))]
    pub pressed: SvgBasicProp,
    #[live(SvgBasicProp::from_state(Theme::default(), SvgState::Disabled))]
    pub disabled: SvgBasicProp,
}

impl Prop for SvgProp {
    type State = SvgState;

    type Basic = SvgBasicProp;

    fn get(&self, state: Self::State) -> &Self::Basic {
        match state {
            SvgState::Basic => &self.basic,
            SvgState::Hover => &self.hover,
            SvgState::Pressed => &self.pressed,
            SvgState::Disabled => &self.disabled,
        }
    }

    fn get_mut(&mut self, state: Self::State) -> &mut Self::Basic {
        match state {
            SvgState::Basic => &mut self.basic,
            SvgState::Hover => &mut self.hover,
            SvgState::Pressed => &mut self.pressed,
            SvgState::Disabled => &mut self.disabled,
        }
    }

    fn len() -> usize {
        4 * SvgBasicProp::len()
    }

    fn sync(&mut self, map: &crate::prop::ApplyStateMap<Self::State>) -> ()
    where
        Self::State: Eq + std::hash::Hash + Copy,
    {
        map.sync(
            &mut self.basic,
            SvgState::Basic,
            [
                (SvgState::Hover, &mut self.hover),
                (SvgState::Pressed, &mut self.pressed),
                (SvgState::Disabled, &mut self.disabled),
            ],
        );
    }
}

impl SlotProp for SvgProp {
    type Part = SvgPart;

    fn sync_slot(&mut self, map: &crate::prop::ApplySlotMap<Self::State, Self::Part>) -> () {
        map.sync(
            &mut self.basic,
            SvgState::Basic,
            [
                (SvgState::Hover, &mut self.hover),
                (SvgState::Pressed, &mut self.pressed),
                (SvgState::Disabled, &mut self.disabled),
            ],
            [SvgPart::Container, SvgPart::Svg],
        );
    }
}

impl Default for SvgProp {
    fn default() -> Self {
        Self {
            basic: Default::default(),
            hover: SvgBasicProp::from_state(Theme::default(), SvgState::Hover),
            pressed: SvgBasicProp::from_state(Theme::default(), SvgState::Pressed),
            disabled: SvgBasicProp::from_state(Theme::default(), SvgState::Disabled),
        }
    }
}

try_from_toml_item! {
    SvgProp {
        basic => BASIC, SvgBasicProp::default(), |v| (v, SvgState::Basic).try_into(),
        hover => HOVER, SvgBasicProp::from_state(Theme::default(), SvgState::Hover), |v| (v, SvgState::Hover).try_into(),
        pressed => PRESSED, SvgBasicProp::from_state(Theme::default(), SvgState::Pressed), |v| (v, SvgState::Pressed).try_into(),
        disabled => DISABLED, SvgBasicProp::from_state(Theme::default(), SvgState::Disabled), |v| (v, SvgState::Disabled).try_into()
    }, "[components.svg] should be a table"
}

#[derive(Debug, Clone, Live, LiveHook, LiveRegister, Copy)]
#[live_ignore]
pub struct SvgBasicProp {
    #[live]
    pub svg: SvgPartProp,
    #[live]
    pub container: ViewBasicProp,
}

impl Default for SvgBasicProp {
    fn default() -> Self {
        Self::from_state(Theme::default(), SvgState::Basic)
    }
}

impl SlotBasicProp for SvgBasicProp {
    type Part = SvgPart;

    fn set_from_str_slot(
        &mut self,
        key: &str,
        value: &LiveValue,
        state: Self::State,
        part: Self::Part,
    ) -> () {
        match part {
            SvgPart::Container => self.container.set_from_str(key, value, state.into()),
            SvgPart::Svg => self.svg.set_from_str(key, value, state),
        }
    }

    fn sync_slot(&mut self, state: Self::State, part: Self::Part) -> () {
        match part {
            SvgPart::Container => self.container.sync(state.into()),
            SvgPart::Svg => self.svg.sync(state),
        }
    }
}

impl BasicProp for SvgBasicProp {
    type State = SvgState;

    type Colors = Color;

    fn from_state(theme: Theme, state: Self::State) -> Self {
        Self {
            svg: Self::default_svg(theme, state),
            container: Self::default_container(theme, state),
        }
    }

    fn state_colors(theme: Theme, state: Self::State) -> Self::Colors {
        SvgPartProp::state_colors(theme, state)
    }

    fn len() -> usize {
        SvgPartProp::len() + ViewBasicProp::len()
    }

    fn set_from_str(&mut self, _key: &str, _value: &LiveValue, _state: Self::State) -> () {
        ()
    }

    fn sync(&mut self, state: Self::State) -> () {
        self.svg.sync(state);
        self.container.sync(state.into());
    }

    fn live_props() -> LiveProps {
        vec![
            (live_id!(svg), SvgPartProp::live_props().into()),
            (live_id!(container), ViewBasicProp::live_props().into()),
        ]
    }

    fn walk(&self) -> Walk {
        self.container.walk()
    }

    fn layout(&self) -> Layout {
        self.container.layout()
    }
}

impl TryFrom<(&Item, SvgState)> for SvgBasicProp {
    type Error = Error;

    fn try_from((value, state): (&Item, SvgState)) -> Result<Self, Self::Error> {
        let inline_table = value.as_inline_table().ok_or(Error::ThemeStyleParse(
            "[component.svg.$part] should be an inline table".to_string(),
        ))?;

        (inline_table, state).try_into()
    }
}

impl TryFrom<(&Value, SvgState)> for SvgBasicProp {
    type Error = Error;

    fn try_from((value, state): (&Value, SvgState)) -> Result<Self, Self::Error> {
        let inline_table = value.as_inline_table().ok_or(Error::ThemeStyleParse(
            "[component.svg.$part] should be an inline table".to_string(),
        ))?;

        (inline_table, state).try_into()
    }
}

impl TryFrom<(&InlineTable, SvgState)> for SvgBasicProp {
    type Error = Error;

    fn try_from((inline_table, state): (&InlineTable, SvgState)) -> Result<Self, Self::Error> {
        let svg = get_from_itable(
            inline_table,
            SVG,
            || Ok(Self::default_svg(Theme::default(), state)),
            |v| (v, state).try_into(),
        )?;

        let container = get_from_itable(
            inline_table,
            CONTAINER,
            || Ok(Self::default_container(Theme::default(), state)),
            |v| (v, state.into()).try_into(),
        )?;

        Ok(Self { container, svg })
    }
}

impl SvgBasicProp {
    pub fn default_container(theme: Theme, state: SvgState) -> ViewBasicProp {
        let mut container = ViewBasicProp::from_state(theme, state.into());
        container.set_clip_x(true);
        container.set_clip_y(true);
        container.set_align(Align::from_f64(0.5));
        container.set_width(Size::Fit);
        container.set_height(Size::Fit);
        container.set_background_visible(false);
        container
    }
    pub fn default_svg(theme: Theme, state: SvgState) -> SvgPartProp {
        SvgPartProp::from_state(theme, state)
    }
}

#[derive(Debug, Clone, Live, LiveHook, LiveRegister, Copy)]
#[live_ignore]
pub struct SvgPartProp {
    #[live]
    pub theme: Theme,
    #[live]
    pub color: Vec4,
    #[live(Margin::from_f64(0.0))]
    pub margin: Margin,
    #[live(MouseCursor::default())]
    pub cursor: MouseCursor,
    #[live(Size::Fixed(16.0))]
    pub height: Size,
    #[live(Size::Fixed(16.0))]
    pub width: Size,
    #[live(None)]
    pub abs_pos: Option<DVec2>,
}

impl Default for SvgPartProp {
    fn default() -> Self {
        Self::from_state(Theme::default(), SvgState::default())
    }
}

impl BasicProp for SvgPartProp {
    type State = SvgState;
    /// color
    type Colors = Color;

    fn from_state(theme: crate::themes::Theme, state: Self::State) -> Self {
        let color = Self::state_colors(theme, state).into();
        Self {
            theme,
            color,
            margin: Margin::from_f64(0.0),
            cursor: MouseCursor::default(),
            height: Size::Fixed(16.0),
            width: Size::Fixed(16.0),
            abs_pos: None,
        }
    }

    fn state_colors(theme: crate::themes::Theme, state: Self::State) -> Self::Colors {
        let color_level = match state {
            SvgState::Basic => 500,
            SvgState::Hover => 400,
            SvgState::Pressed => 600,
            SvgState::Disabled => 300,
        };

        match theme {
            Theme::Dark => Theme::Dark.color(color_level),
            Theme::Primary => Theme::Primary.color(color_level),
            Theme::Error => Theme::Error.color(color_level),
            Theme::Warning => Theme::Warning.color(color_level),
            Theme::Success => Theme::Success.color(color_level),
            Theme::Info => Theme::Info.color(color_level),
        }
    }

    fn len() -> usize {
        6
    }

    fn set_from_str(&mut self, key: &str, value: &LiveValue, state: Self::State) -> () {
        match key {
            THEME => {
                self.theme = Theme::from_live_value(value).unwrap_or(Theme::default());
                self.sync(state);
            }
            COLOR => {
                let color = Self::state_colors(self.theme, state);
                self.color = Vec4::from_live_color(value).unwrap_or(color.into());
            }
            MARGIN => {
                self.margin = Margin::from_live_value(value).unwrap_or(Margin::from_f64(0.0));
            }
            HEIGHT => {
                self.height = Size::from_live_value(value).unwrap_or(Size::Fixed(16.0));
            }
            WIDTH => {
                self.width = Size::from_live_value(value).unwrap_or(Size::Fixed(16.0));
            }
            ABS_POS => {
                self.abs_pos = DVec2::from_live_value(value);
            }
            _ => {}
        }
    }

    fn sync(&mut self, state: Self::State) -> () {
        let color = Self::state_colors(self.theme, state);
        self.color = color.into();
    }

    fn live_props() -> LiveProps {
        vec![
            (live_id!(theme), None.into()),
            (live_id!(color), None.into()),
            (
                live_id!(margin),
                Some(vec![
                    live_id!(top),
                    live_id!(bottom),
                    live_id!(left),
                    live_id!(right),
                ]).into(),
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

impl TryFrom<(&Value, SvgState)> for SvgPartProp {
    type Error = Error;

    fn try_from((value, state): (&Value, SvgState)) -> Result<Self, Self::Error> {
        let inline_table = value.as_inline_table().ok_or(Error::ThemeStyleParse(
            "[components.svg.svg] should be an inline table".to_string(),
        ))?;

        let theme = Theme::default();
        let theme = get_from_itable(inline_table, THEME, || Ok(theme), |v| v.try_into())?;
        let color = Self::state_colors(theme, state);
        let color = get_from_itable(
            inline_table,
            BACKGROUND_COLOR,
            || Ok(color),
            |v| v.try_into(),
        )?
        .into();
        let margin = Margin::from_f64(0.0);
        let margin = get_from_itable(inline_table, MARGIN, || Ok(margin), |v| v.to_margin(margin))?;
        let cursor = get_from_itable(
            inline_table,
            CURSOR,
            || Ok(MouseCursor::Default),
            |v| v.to_cursor(),
        )?;
        let height = get_from_itable(
            inline_table,
            HEIGHT,
            || Ok(Size::Fixed(16.0)),
            |v| v.to_size(),
        )?;
        let width = get_from_itable(
            inline_table,
            WIDTH,
            || Ok(Size::Fixed(16.0)),
            |v| v.to_size(),
        )?;
        let abs_pos = get_from_itable(
            inline_table,
            ABS_POS,
            || Ok(None),
            |v| v.to_dvec2().map(Some),
        )?;
        Ok(Self {
            theme,
            color,
            margin,
            cursor,
            height,
            width,
            abs_pos,
        })
    }
}

component_state! {
    SvgState {
        Basic => BASIC,
        Hover => HOVER,
        Pressed => PRESSED,
        Disabled => DISABLED
    }, _ => SvgState::Basic
}

impl ComponentState for SvgState {
    fn is_disabled(&self) -> bool {
        matches!(self, SvgState::Disabled)
    }
}

impl From<SvgState> for ViewState {
    fn from(value: SvgState) -> Self {
        match value {
            SvgState::Basic => ViewState::Basic,
            SvgState::Hover => ViewState::Hover,
            SvgState::Pressed => ViewState::Pressed,
            SvgState::Disabled => ViewState::Disabled,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SvgPart {
    Container,
    Svg,
}

impl Part for SvgPart {
    type State = SvgState;
    fn to_live_id(&self) -> LiveId {
        match self {
            SvgPart::Container => live_id!(container),
            SvgPart::Svg => live_id!(svg),
        }
    }
}
