use makepad_widgets::*;
use toml_edit::Item;

use crate::{
    component_part, component_state,
    components::{
        live_props::LiveProps,
        traits::{BasicStyle, ComponentState, Part, Style, SlotBasicStyle, SlotStyle},
        view::{ViewBasicStyle, ViewState},
        ViewColors,
    },
    error::Error,
    from_inherit_to_view_basic_prop, from_prop_to_toml, get_get_mut, inherits_view_basic_prop,
    prop::{
        manuel::{
            ABS_POS, ALIGN, BACKGROUND_COLOR, BACKGROUND_VISIBLE, BASIC, BLUR_RADIUS, BODY,
            BORDER_COLOR, BORDER_RADIUS, BORDER_WIDTH, CLIP_X, CLIP_Y, CONTAINER, CURSOR, FLOW,
            FOOTER, HEADER, HEIGHT, HOVER, MARGIN, PADDING, ROTATION, SCALE, SHADOW_COLOR,
            SHADOW_OFFSET, SPACING, SPREAD_RADIUS, THEME, WIDTH,
        },
        traits::{AbsPos, FromLiveColor, FromLiveValue, NewFrom, ToColor, ToTomlValue},
        ApplySlotMapImpl, ApplyStateMapImpl, Applys, Radius,
    },
    prop_interconvert, state_colors,
    themes::{Theme, TomlValueTo},
    utils::get_from_itable,
};

prop_interconvert! {
    CardProp {
        basic_prop = CardBasicStyle;
        basic => BASIC, CardBasicStyle::default(), |v| (v, CardState::Basic).try_into(),
        hover => HOVER, CardBasicStyle::from_state(Theme::default(), CardState::Hover), |v| (v, CardState::Hover).try_into()
    }, "[component.card] should be a table"
}

impl Style for CardProp {
    type State = CardState;

    type Basic = CardBasicStyle;

    get_get_mut! {
        CardState::Basic => basic,
        CardState::Hover => hover
    }

    fn len() -> usize {
        2 * CardBasicStyle::len()
    }

    fn sync(&mut self, map: &crate::prop::ApplyStateMap<Self::State>) -> ()
    where
        Self::State: Eq + std::hash::Hash + Copy,
    {
        map.sync(
            &mut self.basic,
            CardState::Basic,
            [(CardState::Hover, &mut self.hover)],
        );
    }
}

impl SlotStyle for CardProp {
    type Part = CardPart;

    fn sync_slot(&mut self, map: &crate::prop::ApplySlotMap<Self::State, Self::Part>) -> () {
        map.sync(
            &mut self.basic,
            CardState::Basic,
            [(CardState::Hover, &mut self.hover)],
            [
                CardPart::Container,
                CardPart::Header,
                CardPart::Body,
                CardPart::Footer,
            ],
        );
    }
}

#[derive(Debug, Clone, Live, LiveHook, LiveRegister, Copy)]
#[live_ignore]
pub struct CardBasicStyle {
    #[live(CardBasicStyle::default_container(Theme::default(), CardState::Basic))]
    pub container: ViewBasicStyle,
    #[live(CardBasicStyle::default_header(Theme::default(), CardState::Basic))]
    pub header: CardHeaderProp,
    #[live(CardBasicStyle::default_body(Theme::default(), CardState::Basic))]
    pub body: CardBodyProp,
    #[live(CardBasicStyle::default_footer(Theme::default(), CardState::Basic))]
    pub footer: CardFooterProp,
}

impl BasicStyle for CardBasicStyle {
    type State = CardState;

    type Colors = ViewColors;

    fn from_state(theme: crate::themes::Theme, state: Self::State) -> Self {
        Self {
            container: Self::default_container(theme, state),
            header: Self::default_header(theme, state),
            body: Self::default_body(theme, state),
            footer: Self::default_footer(theme, state),
        }
    }

    fn state_colors(theme: crate::themes::Theme, state: Self::State) -> Self::Colors {
        ViewBasicStyle::state_colors(theme, state.into())
    }

    fn len() -> usize {
        3 * ViewBasicStyle::len()
    }

    fn set_from_str(&mut self, _key: &str, _value: &LiveValue, _state: Self::State) -> () {
        ()
    }

    fn sync(&mut self, state: Self::State) -> () {
        self.header.sync(state.into());
        self.body.sync(state.into());
        self.footer.sync(state.into());
    }

    fn live_props() -> LiveProps {
        vec![
            (live_id!(container), ViewBasicStyle::live_props().into()),
            (live_id!(header), ViewBasicStyle::live_props().into()),
            (live_id!(body), ViewBasicStyle::live_props().into()),
            (live_id!(footer), ViewBasicStyle::live_props().into()),
        ]
    }

    fn walk(&self) -> Walk {
        self.container.walk()
    }
    fn layout(&self) -> Layout {
        self.container.layout()
    }
}

impl SlotBasicStyle for CardBasicStyle {
    type Part = CardPart;

    fn set_from_str_slot(
        &mut self,
        key: &str,
        value: &Applys,
        state: Self::State,
        part: Self::Part,
    ) -> () {
        match part {
            CardPart::Container => self
                .container
                .set_from_str(key, &value.into(), state.into()),
            CardPart::Header => self.header.set_from_str(key, &value.into(), state.into()),
            CardPart::Body => self.body.set_from_str(key, &value.into(), state.into()),
            CardPart::Footer => self.footer.set_from_str(key, &value.into(), state.into()),
        }
    }

    fn sync_slot(&mut self, state: Self::State, part: Self::Part) -> () {
        match part {
            CardPart::Container => self.container.sync(state.into()),
            CardPart::Header => self.header.sync(state.into()),
            CardPart::Body => self.body.sync(state.into()),
            CardPart::Footer => self.footer.sync(state.into()),
        }
    }
}

impl Default for CardBasicStyle {
    fn default() -> Self {
        Self::from_state(Theme::default(), CardState::Basic)
    }
}

from_prop_to_toml! {
    CardBasicStyle {
        container => CONTAINER,
        header => HEADER,
        body => BODY,
        footer => FOOTER
    }
}

impl TryFrom<(&Item, CardState)> for CardBasicStyle {
    type Error = Error;

    fn try_from((value, state): (&Item, CardState)) -> Result<Self, Self::Error> {
        let inline_table = value.as_inline_table().ok_or(Error::ThemeStyleParse(
            "[component.card.$slot] should be an inline table".to_string(),
        ))?;

        let container = get_from_itable(
            inline_table,
            CONTAINER,
            || Ok(CardBasicStyle::default_container(Theme::default(), state)),
            |v| (v, ViewState::from(state)).try_into(),
        )?;

        let header = get_from_itable(
            inline_table,
            HEADER,
            || Ok(CardBasicStyle::default_header(Theme::default(), state)),
            |v| (v, state).try_into(),
        )?;

        let body = get_from_itable(
            inline_table,
            BODY,
            || Ok(CardBasicStyle::default_body(Theme::default(), state)),
            |v| (v, state).try_into(),
        )?;

        let footer = get_from_itable(
            inline_table,
            FOOTER,
            || Ok(CardBasicStyle::default_footer(Theme::default(), state)),
            |v| (v, state).try_into(),
        )?;

        Ok(Self {
            container,
            header,
            body,
            footer,
        })
    }
}

impl CardBasicStyle {
    pub fn default_header(theme: Theme, state: CardState) -> CardHeaderProp {
        CardHeaderProp::from_state(theme, state.into())
    }
    pub fn default_footer(theme: Theme, state: CardState) -> CardFooterProp {
        CardFooterProp::from_state(theme, state.into())
    }
    pub fn default_container(theme: Theme, state: CardState) -> ViewBasicStyle {
        let mut container = ViewBasicStyle::from_state(theme, state.into());
        container.set_cursor(Default::default());
        container.set_background_visible(true);
        container
    }
    pub fn default_body(theme: Theme, state: CardState) -> CardBodyProp {
        CardBodyProp::from_state(theme, state.into())
    }
}

component_state! {
    CardState {
        Basic => BASIC,
        Hover => HOVER
    }, _ => CardState::Basic
}

impl ComponentState for CardState {
    fn is_disabled(&self) -> bool {
        false
    }
}

impl From<CardState> for ViewState {
    fn from(value: CardState) -> Self {
        match value {
            CardState::Basic => ViewState::Basic,
            CardState::Hover => ViewState::Hover,
        }
    }
}

impl From<ViewState> for CardState {
    fn from(value: ViewState) -> Self {
        match value {
            ViewState::Basic => CardState::Basic,
            ViewState::Hover => CardState::Hover,
            _ => panic!("CardState can only be Basic or Hover"),
        }
    }
}

inherits_view_basic_prop! {
    CardHeaderProp {
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
        align: Align::default(),
        cursor: MouseCursor::default(),
        flow: Flow::Right,
        spacing: 0.0,
        height: Size::Fit,
        width: Size::Fill,
        abs_pos: None,
    }, CardState, "card.header",
    {
        CardState::Basic => (500, 500, 400),
        CardState::Hover => (400, 400, 300)
    }
}

inherits_view_basic_prop! {
    CardBodyProp {
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
        align: Align::default(),
        cursor: MouseCursor::default(),
        flow: Flow::Right,
        spacing: 0.0,
        height: Size::Fill,
        width: Size::Fill,
        abs_pos: None,
    }, CardState, "card.body", {
        CardState::Basic => (500, 500, 400),
        CardState::Hover => (400, 400, 300)
    }
}

inherits_view_basic_prop! {
    CardFooterProp {
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
        align: Align::default(),
        cursor: MouseCursor::default(),
        flow: Flow::Right,
        spacing: 0.0,
        height: Size::Fit,
        width: Size::Fill,
        abs_pos: None,
    }, CardState, "card.footer",
    {
        CardState::Basic => (500, 500, 400),
        CardState::Hover => (400, 400, 300)
    }
}

from_inherit_to_view_basic_prop!(CardHeaderProp);
from_inherit_to_view_basic_prop!(CardBodyProp);
from_inherit_to_view_basic_prop!(CardFooterProp);

component_part! {
    CardPart {
        Container => container => CONTAINER,
        Header => header => HEADER,
        Body => body => BODY,
        Footer => footer => FOOTER
    }, CardState
}
