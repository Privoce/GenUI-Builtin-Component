use makepad_widgets::*;
use toml_edit::Item;

use crate::{
    components::{
        traits::{BasicProp, Part, Prop, SlotBasicProp, SlotProp},
        view::{ViewBasicProp, ViewState},
    },
    error::Error,
    prop::{
        manuel::{BASIC, BODY, FOOTER, HEADER, OUTER},
        ApplySlotMapImpl, ApplyStateMapImpl,
    },
    themes::{Color, Theme},
    utils::{get_from_itable, get_from_table},
};

#[derive(Debug, Clone, Live, LiveHook, LiveRegister)]
#[live_ignore]
pub struct CardProp {
    #[live(CardBasicProp::default())]
    pub basic: CardBasicProp,
    #[live(CardBasicProp::from_state(Theme::default(), CardState::Hover))]
    pub hover: CardBasicProp,
}

impl Prop for CardProp {
    type State = CardState;

    type Basic = CardBasicProp;

    fn get(&self, state: Self::State) -> &Self::Basic {
        match state {
            CardState::Basic => &self.basic,
            CardState::Hover => &self.hover,
        }
    }

    fn get_mut(&mut self, state: Self::State) -> &mut Self::Basic {
        match state {
            CardState::Basic => &mut self.basic,
            CardState::Hover => &mut self.hover,
        }
    }

    fn len() -> usize {
        2 * CardBasicProp::len()
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

impl SlotProp for CardProp {
    type Part = CardPart;

    fn sync_slot(&mut self, map: &crate::prop::ApplySlotMap<Self::State, Self::Part>) -> () {
        map.sync(
            &mut self.basic,
            CardState::Basic,
            [(CardState::Hover, &mut self.hover)],
            [
                CardPart::Outer,
                CardPart::Header,
                CardPart::Body,
                CardPart::Footer,
            ],
        );
    }
}

impl TryFrom<&Item> for CardProp {
    type Error = Error;

    fn try_from(value: &Item) -> Result<Self, Self::Error> {
        let table = value.as_table().ok_or(Error::ThemeStyleParse(
            "[component.card] should be a table".to_string(),
        ))?;

        let basic = get_from_table(
            table,
            BASIC,
            || Ok(CardBasicProp::default()),
            |v| (v, CardState::Basic).try_into(),
        )?;
        let hover = get_from_table(
            table,
            "hover",
            || {
                Ok(CardBasicProp::from_state(
                    Theme::default(),
                    CardState::Hover,
                ))
            },
            |v| (v, CardState::Hover).try_into(),
        )?;
        Ok(Self { basic, hover })
    }
}

impl Default for CardProp {
    fn default() -> Self {
        Self {
            basic: CardBasicProp::default(),
            hover: CardBasicProp::from_state(Theme::default(), CardState::Hover),
        }
    }
}

#[derive(Debug, Clone, Live, LiveHook, LiveRegister, Copy)]
#[live_ignore]
pub struct CardBasicProp {
    #[live(CardBasicProp::default_outer(CardState::Basic))]
    pub outer: ViewBasicProp,
    #[live(CardBasicProp::default_header(CardState::Basic))]
    pub header: ViewBasicProp,
    #[live(CardBasicProp::default_body(CardState::Basic))]
    pub body: ViewBasicProp,
    #[live(CardBasicProp::default_footer(CardState::Basic))]
    pub footer: ViewBasicProp,
}

impl BasicProp for CardBasicProp {
    type State = CardState;

    type Colors = (Color, Color, Color);

    fn from_state(theme: crate::themes::Theme, state: Self::State) -> Self {
        let mut basic = ViewBasicProp::from_state(theme, state.into());
        basic.set_cursor(Default::default());
        let mut header_footer = basic.clone();
        header_footer.set_height(Size::Fixed(32.0));
        let mut body = basic.clone();
        body.set_height(Size::Fill);

        Self {
            outer: basic.clone(),
            header: header_footer.clone(),
            body,
            footer: header_footer,
        }
    }

    fn state_colors(theme: crate::themes::Theme, state: Self::State) -> Self::Colors {
        ViewBasicProp::state_colors(theme, state.into())
    }

    fn len() -> usize {
        3 * ViewBasicProp::len()
    }

    fn set_from_str(&mut self, _key: &str, _value: &LiveValue, _state: Self::State) -> () {
        // self.header.set_from_str(key, value, state.into());
        // self.body.set_from_str(key, value, state.into());
        // self.footer.set_from_str(key, value, state.into());
        ()
    }

    fn sync(&mut self, state: Self::State) -> () {
        self.header.sync(state.into());
        self.body.sync(state.into());
        self.footer.sync(state.into());
    }

    fn live_props() -> Vec<(LiveId, Option<Vec<LiveId>>)> {
        ViewBasicProp::live_props()
    }

    fn walk(&self) -> Walk {
        self.outer.walk()
    }
}

impl SlotBasicProp for CardBasicProp {
    type Part = CardPart;

    fn set_from_str_slot(
        &mut self,
        key: &str,
        value: &LiveValue,
        state: Self::State,
        part: Self::Part,
    ) -> () {
        match part {
            CardPart::Outer => self.outer.set_from_str(key, value, state.into()),
            CardPart::Header => self.header.set_from_str(key, value, state.into()),
            CardPart::Body => self.body.set_from_str(key, value, state.into()),
            CardPart::Footer => self.footer.set_from_str(key, value, state.into()),
        }
    }

    fn sync_slot(&mut self, state: Self::State, part: Self::Part) -> () {
        match part {
            CardPart::Outer => self.outer.sync(state.into()),
            CardPart::Header => self.header.sync(state.into()),
            CardPart::Body => self.body.sync(state.into()),
            CardPart::Footer => self.footer.sync(state.into()),
        }
    }
}

impl Default for CardBasicProp {
    fn default() -> Self {
        Self::from_state(Theme::default(), CardState::Basic)
    }
}

impl TryFrom<(&Item, CardState)> for CardBasicProp {
    type Error = Error;

    fn try_from((value, state): (&Item, CardState)) -> Result<Self, Self::Error> {
        let inline_table = value.as_inline_table().ok_or(Error::ThemeStyleParse(
            "[component.card.$slot] should be an inline table".to_string(),
        ))?;

        let outer = get_from_itable(
            inline_table,
            OUTER,
            || Ok(CardBasicProp::default_outer(state)),
            |v| (v, ViewState::from(state)).try_into(),
        )?;

        let header = get_from_itable(
            inline_table,
            HEADER,
            || Ok(CardBasicProp::default_header(state)),
            |v| (v, ViewState::from(state)).try_into(),
        )?;

        let body = get_from_itable(
            inline_table,
            BODY,
            || Ok(CardBasicProp::default_body(state)),
            |v| (v, ViewState::from(state)).try_into(),
        )?;

        let footer = get_from_itable(
            inline_table,
            FOOTER,
            || Ok(CardBasicProp::default_footer(state)),
            |v| (v, ViewState::from(state)).try_into(),
        )?;

        Ok(Self {
            outer,
            header,
            body,
            footer,
        })
    }
}

impl CardBasicProp {
    pub fn default_header(state: CardState) -> ViewBasicProp {
        let mut header = Self::default_outer(state);
        header.set_height(Size::Fixed(32.0));
        header
    }
    pub fn default_footer(state: CardState) -> ViewBasicProp {
        Self::default_header(state)
    }
    pub fn default_outer(state: CardState) -> ViewBasicProp {
        let mut outer = ViewBasicProp::from_state(Theme::default(), state.into());
        outer.set_cursor(Default::default());
        outer
    }
    pub fn default_body(state: CardState) -> ViewBasicProp {
        let mut body = Self::default_outer(state);
        body.set_height(Size::Fill);
        body
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum CardState {
    #[default]
    Basic,
    Hover,
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

/// Represents the different parts of a card component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CardPart {
    Outer,
    Header,
    Body,
    Footer,
}

impl Part for CardPart {
    type State = ViewState;
    fn to_live_id(&self) -> LiveId {
        match self {
            CardPart::Outer => live_id!(outer),
            CardPart::Header => live_id!(header),
            CardPart::Body => live_id!(body),
            CardPart::Footer => live_id!(footer),
        }
    }
}
