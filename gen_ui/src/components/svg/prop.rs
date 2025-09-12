use makepad_widgets::*;
use toml_edit::{InlineTable, Item, Value};

use crate::{
    basic_prop_interconvert, component_color, component_part, component_state,
    components::{
        live_props::LiveProps,
        traits::{BasicStyle, ComponentState, Part, Style, SlotBasicStyle, SlotStyle},
        view::ViewState,
        ViewBasicStyle,
    },
    error::Error,
    from_inherit_to_view_basic_prop, from_prop_to_toml, get_get_mut, inherits_view_basic_prop,
    prop::{
        manuel::{
            ABS_POS, ALIGN, BACKGROUND_COLOR, BACKGROUND_VISIBLE, BASIC, BLUR_RADIUS, BORDER_COLOR,
            BORDER_RADIUS, BORDER_WIDTH, CLIP_X, CLIP_Y, COLOR, CONTAINER, CURSOR, DISABLED, FLOW,
            HEIGHT, HOVER, MARGIN, PADDING, PRESSED, ROTATION, SCALE, SHADOW_COLOR, SHADOW_OFFSET,
            SPACING, SPREAD_RADIUS, SVG, THEME, WIDTH,
        },
        traits::{AbsPos, FromLiveColor, FromLiveValue, NewFrom, ToColor, ToTomlValue},
        ApplySlotMapImpl, Radius,
    },
    prop_interconvert, state_color, state_colors,
    themes::{Theme, TomlValueTo},
    utils::get_from_itable,
};

prop_interconvert! {
    SvgStyle {
        basic_prop = SvgBasicStyle;
        basic => BASIC, SvgBasicStyle::default(), |v| (v, SvgState::Basic).try_into(),
        hover => HOVER, SvgBasicStyle::from_state(Theme::default(), SvgState::Hover), |v| (v, SvgState::Hover).try_into(),
        pressed => PRESSED, SvgBasicStyle::from_state(Theme::default(), SvgState::Pressed), |v| (v, SvgState::Pressed).try_into(),
        disabled => DISABLED, SvgBasicStyle::from_state(Theme::default(), SvgState::Disabled), |v| (v, SvgState::Disabled).try_into()
    }, "[components.svg] should be a table"
}

impl Style for SvgStyle {
    type State = SvgState;

    type Basic = SvgBasicStyle;

    get_get_mut! {
        SvgState::Basic => basic,
        SvgState::Hover => hover,
        SvgState::Pressed => pressed,
        SvgState::Disabled => disabled
    }

    fn len() -> usize {
        4 * SvgBasicStyle::len()
    }

    fn sync(&mut self, _map: &crate::prop::ApplyStateMap<Self::State>) -> ()
    where
        Self::State: Eq + std::hash::Hash + Copy,
    {
        ()
    }
}

impl SlotStyle for SvgStyle {
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

#[derive(Debug, Clone, Live, LiveHook, LiveRegister, Copy)]
#[live_ignore]
pub struct SvgBasicStyle {
    #[live(SvgPartProp::default())]
    pub svg: SvgPartProp,
    #[live(SvgBasicStyle::default_container(Theme::default(), SvgState::Basic))]
    pub container: SvgContainerProp,
}

impl Default for SvgBasicStyle {
    fn default() -> Self {
        Self::from_state(Theme::default(), SvgState::Basic)
    }
}

from_prop_to_toml! {
    SvgBasicStyle {
        svg => SVG,
        container => CONTAINER
    }
}

impl SlotBasicStyle for SvgBasicStyle {
    type Part = SvgPart;

    fn set_from_str_slot(
        &mut self,
        key: &str,
        value: &crate::prop::Applys,
        state: Self::State,
        part: Self::Part,
    ) -> () {
        match part {
            SvgPart::Container => {
                self.container.set_from_str(key, &value.into(), state);
            }
            SvgPart::Svg => self.svg.set_from_str(key, &value.into(), state),
        }
    }

    fn sync_slot(&mut self, state: Self::State, part: Self::Part) -> () {
        match part {
            SvgPart::Container => self.container.sync(state),
            SvgPart::Svg => self.svg.sync(state),
        }
    }
}

impl BasicStyle for SvgBasicStyle {
    type State = SvgState;

    type Colors = SvgColors;

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
        SvgPartProp::len() + SvgContainerProp::len()
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
            (live_id!(container), SvgContainerProp::live_props().into()),
        ]
    }

    fn walk(&self) -> Walk {
        self.container.walk()
    }

    fn layout(&self) -> Layout {
        self.container.layout()
    }
}

impl TryFrom<(&Item, SvgState)> for SvgBasicStyle {
    type Error = Error;

    fn try_from((value, state): (&Item, SvgState)) -> Result<Self, Self::Error> {
        let inline_table = value.as_inline_table().ok_or(Error::ThemeStyleParse(
            "[component.svg.$part] should be an inline table".to_string(),
        ))?;

        (inline_table, state).try_into()
    }
}

impl TryFrom<(&Value, SvgState)> for SvgBasicStyle {
    type Error = Error;

    fn try_from((value, state): (&Value, SvgState)) -> Result<Self, Self::Error> {
        let inline_table = value.as_inline_table().ok_or(Error::ThemeStyleParse(
            "[component.svg.$part] should be an inline table".to_string(),
        ))?;

        (inline_table, state).try_into()
    }
}

impl TryFrom<(&InlineTable, SvgState)> for SvgBasicStyle {
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

impl SvgBasicStyle {
    pub fn default_container(theme: Theme, state: SvgState) -> SvgContainerProp {
        SvgContainerProp::from_state(theme, state.into())
    }
    pub fn default_svg(theme: Theme, state: SvgState) -> SvgPartProp {
        SvgPartProp::from_state(theme, state)
    }
}

basic_prop_interconvert! {
    SvgPartProp {
        state = SvgState;
        {
            color => COLOR, |v| v.try_into()
        };
        {
            margin: Margin => MARGIN, Margin::from_f64(0.0), |v| v.to_margin(margin),
            cursor: MouseCursor => CURSOR, MouseCursor::Default, |v| v.to_cursor(),
            height: Size => HEIGHT, Size::Fit, |v| v.to_size(),
            width: Size => WIDTH, Size::Fixed(16.0), |v| v.to_size(),
            abs_pos: AbsPos => ABS_POS, None, |v| Ok(v.to_dvec2().map_or(None, |v| Some(v)))
        }
    }, "[components.svg.svg] should be an inline table"
}

component_color! {
    SvgColors {
        colors = (Color);
        color
    }
}

impl BasicStyle for SvgPartProp {
    type State = SvgState;
    /// color
    type Colors = SvgColors;

    fn from_state(theme: crate::themes::Theme, state: Self::State) -> Self {
        let SvgColors { color } = Self::state_colors(theme, state);
        Self {
            theme,
            color: color.into(),
            margin: Margin::from_f64(0.0),
            cursor: MouseCursor::default(),
            height: Size::Fit,
            width: Size::Fixed(16.0),
            abs_pos: None,
        }
    }

    state_color! {
        (color_level),
        SvgState::Basic => (200),
        SvgState::Hover => (100),
        SvgState::Pressed => (300),
        SvgState::Disabled => (100)
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
                let SvgColors { color } = Self::state_colors(self.theme, state);
                self.color = Vec4::from_live_color(value).unwrap_or(color.into());
            }
            MARGIN => {
                self.margin = Margin::from_live_value(value).unwrap_or(Margin::from_f64(0.0));
            }
            HEIGHT => {
                self.height = Size::from_live_value(value).unwrap_or(Size::Fit);
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
        let SvgColors { color } = Self::state_colors(self.theme, state);
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

// impl TryFrom<(&Value, SvgState)> for SvgPartProp {
//     type Error = Error;

//     fn try_from((value, state): (&Value, SvgState)) -> Result<Self, Self::Error> {
//         let inline_table = value.as_inline_table().ok_or(Error::ThemeStyleParse(
//             "[components.svg.svg] should be an inline table".to_string(),
//         ))?;

//         let theme = Theme::default();
//         let theme = get_from_itable(inline_table, THEME, || Ok(theme), |v| v.try_into())?;
//         let color = Self::state_colors(theme, state);
//         let color = get_from_itable(
//             inline_table,
//             BACKGROUND_COLOR,
//             || Ok(color),
//             |v| v.try_into(),
//         )?
//         .into();
//         let margin = Margin::from_f64(0.0);
//         let margin = get_from_itable(inline_table, MARGIN, || Ok(margin), |v| v.to_margin(margin))?;
//         let cursor = get_from_itable(
//             inline_table,
//             CURSOR,
//             || Ok(MouseCursor::Default),
//             |v| v.to_cursor(),
//         )?;
//         let height = get_from_itable(inline_table, HEIGHT, || Ok(Size::Fit), |v| v.to_size())?;
//         let width = get_from_itable(
//             inline_table,
//             WIDTH,
//             || Ok(Size::Fixed(16.0)),
//             |v| v.to_size(),
//         )?;
//         let abs_pos = get_from_itable(
//             inline_table,
//             ABS_POS,
//             || Ok(None),
//             |v| Ok(v.to_dvec2().map_or(None, |v| Some(v))),
//         )?;
//         Ok(Self {
//             theme,
//             color,
//             margin,
//             cursor,
//             height,
//             width,
//             abs_pos,
//         })
//     }
// }

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

component_part! {
    SvgPart {
        Container => container => CONTAINER,
        Svg => svg => SVG
    } , SvgState
}

inherits_view_basic_prop! {
    SvgContainerProp {
        border_width: 0.0,
        border_radius: Radius::new(4.0),
        spread_radius: 0.0,
        blur_radius: 0.0,
        shadow_offset: vec2(0.0, 0.0),
        background_visible: false,
        rotation: 0.0,
        scale: 1.0,
        padding: Padding::from_f64(0.0),
        margin: Margin::from_f64(0.0),
        clip_x: false,
        clip_y: false,
        align: Align::from_f64(0.5),
        cursor: MouseCursor::default(),
        flow: Flow::Down,
        spacing: 0.0,
        height: Size::Fit,
        width: Size::Fit,
        abs_pos: None,
    }, SvgState, "svg.container",
    {
        SvgState::Basic => (500, 500, 400),
        SvgState::Hover => (400, 400, 300),
        SvgState::Pressed => (600, 600, 500),
        SvgState::Disabled => (300, 300, 200)
    }
}

from_inherit_to_view_basic_prop!(SvgContainerProp);
