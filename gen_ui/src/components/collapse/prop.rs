use makepad_widgets::*;
use toml_edit::Item;

use crate::{
    component_part, component_state,
    components::{
        label::LabelState,
        live_props::LiveProps,
        svg::SvgState,
        traits::{BasicProp, ComponentState, Part, Prop, SlotBasicProp, SlotProp},
        view::{ViewBasicProp, ViewState},
    },
    error::Error,
    get_get_mut,
    prop::{
        manuel::{BASIC, BODY, CONTAINER, DISABLED, HEADER, HOVER, PRESSED},
        traits::NewFrom,
        ApplySlotMapImpl, Applys,
    },
    themes::{Color, Theme},
    prop_interconvert,
    utils::get_from_itable,
};

#[derive(Debug, Clone, Live, LiveHook, LiveRegister)]
#[live_ignore]
pub struct CollapseProp {
    #[live(CollapseBasicProp::default())]
    pub basic: CollapseBasicProp,
    #[live(CollapseBasicProp::from_state(Theme::default(), CollapseState::Hover))]
    pub hover: CollapseBasicProp,
    #[live(CollapseBasicProp::from_state(Theme::default(), CollapseState::Active))]
    pub active: CollapseBasicProp,
    #[live(CollapseBasicProp::from_state(Theme::default(), CollapseState::Disabled))]
    pub disabled: CollapseBasicProp,
}

impl Prop for CollapseProp {
    type State = CollapseState;

    type Basic = CollapseBasicProp;

    get_get_mut! {
        CollapseState::Basic => basic,
        CollapseState::Hover => hover,
        CollapseState::Active => active,
        CollapseState::Disabled => disabled
    }

    fn len() -> usize {
        4 * CollapseBasicProp::len()
    }

    fn sync(&mut self, _map: &crate::prop::ApplyStateMap<Self::State>) -> ()
    where
        Self::State: Eq + std::hash::Hash + Copy,
    {
        ()
    }
}

impl SlotProp for CollapseProp {
    type Part = CollapsePart;

    fn sync_slot(&mut self, map: &crate::prop::ApplySlotMap<Self::State, Self::Part>) -> () {
        map.sync(
            &mut self.basic,
            CollapseState::Basic,
            [
                (CollapseState::Hover, &mut self.hover),
                (CollapseState::Active, &mut self.active),
                (CollapseState::Disabled, &mut self.disabled),
            ],
            [
                CollapsePart::Container,
                CollapsePart::Header,
                CollapsePart::Body,
            ],
        );
    }
}

prop_interconvert! {
    CollapseProp {
        basic => BASIC, CollapseBasicProp::default(), |v| (v, CollapseState::Basic).try_into(),
        hover => HOVER, CollapseBasicProp::from_state(Theme::default(), CollapseState::Hover), |v| (v, CollapseState::Hover).try_into(),
        active => PRESSED, CollapseBasicProp::from_state(Theme::default(), CollapseState::Active), |v| (v, CollapseState::Active).try_into(),
        disabled => DISABLED, CollapseBasicProp::from_state(Theme::default(), CollapseState::Disabled), |v| (v, CollapseState::Disabled).try_into()
    }, "[component.menu_item] should be a table"
}

impl Default for CollapseProp {
    fn default() -> Self {
        Self {
            basic: CollapseBasicProp::default(),
            hover: CollapseBasicProp::from_state(Theme::default(), CollapseState::Hover),
            active: CollapseBasicProp::from_state(Theme::default(), CollapseState::Active),
            disabled: CollapseBasicProp::from_state(Theme::default(), CollapseState::Disabled),
        }
    }
}

#[derive(Debug, Clone, Live, LiveHook, LiveRegister, Copy)]
#[live_ignore]
pub struct CollapseBasicProp {
    #[live(CollapseBasicProp::default_container(Theme::default(), CollapseState::Basic))]
    pub container: ViewBasicProp,
    #[live(CollapseBasicProp::default_header(Theme::default(), CollapseState::Basic))]
    pub header: ViewBasicProp,
    #[live(CollapseBasicProp::default_body(Theme::default(), CollapseState::Basic))]
    pub body: ViewBasicProp,
}

impl BasicProp for CollapseBasicProp {
    type State = CollapseState;

    type Colors = (Color, Color, Color);

    fn from_state(theme: crate::themes::Theme, state: Self::State) -> Self {
        Self {
            container: Self::default_container(theme, state),
            header: Self::default_header(theme, state),
            body: Self::default_body(theme, state),
        }
    }

    fn state_colors(theme: crate::themes::Theme, state: Self::State) -> Self::Colors {
        ViewBasicProp::state_colors(theme, state.into())
    }

    fn len() -> usize {
        3 * ViewBasicProp::len()
    }

    fn set_from_str(&mut self, _key: &str, _value: &LiveValue, _state: Self::State) -> () {
        ()
    }

    fn sync(&mut self, state: Self::State) -> () {
        self.header.sync(state.into());
        self.body.sync(state.into());
    }

    fn live_props() -> LiveProps {
        vec![
            (live_id!(container), ViewBasicProp::live_props().into()),
            (live_id!(header), ViewBasicProp::live_props().into()),
            (live_id!(body), ViewBasicProp::live_props().into()),
        ]
    }

    fn walk(&self) -> Walk {
        self.container.walk()
    }
    fn layout(&self) -> Layout {
        self.container.layout()
    }
}

impl SlotBasicProp for CollapseBasicProp {
    type Part = CollapsePart;

    fn set_from_str_slot(
        &mut self,
        key: &str,
        value: &Applys,
        state: Self::State,
        part: Self::Part,
    ) -> () {
        match part {
            CollapsePart::Container => {
                self.container
                    .set_from_str(key, &value.into(), state.into())
            }
            CollapsePart::Header => self.header.set_from_str(key, &value.into(), state.into()),
            CollapsePart::Body => {
                self.body.set_from_str(key, &value.into(), state.into());
            }
        }
    }

    fn sync_slot(&mut self, state: Self::State, part: Self::Part) -> () {
        match part {
            CollapsePart::Container => self.container.sync(state.into()),
            CollapsePart::Header => self.header.sync(state.into()),
            CollapsePart::Body => self.body.sync(state.into()),
        }
    }
}

impl Default for CollapseBasicProp {
    fn default() -> Self {
        Self::from_state(Theme::default(), CollapseState::Basic)
    }
}

impl TryFrom<(&Item, CollapseState)> for CollapseBasicProp {
    type Error = Error;

    fn try_from((value, state): (&Item, CollapseState)) -> Result<Self, Self::Error> {
        let inline_table = value.as_inline_table().ok_or(Error::ThemeStyleParse(
            "[component.card.$slot] should be an inline table".to_string(),
        ))?;

        let container = get_from_itable(
            inline_table,
            CONTAINER,
            || {
                Ok(CollapseBasicProp::default_container(
                    Theme::default(),
                    state,
                ))
            },
            |v| (v, ViewState::from(state)).try_into(),
        )?;

        let header = get_from_itable(
            inline_table,
            HEADER,
            || Ok(CollapseBasicProp::default_header(Theme::default(), state)),
            |v| (v, ViewState::from(state)).try_into(),
        )?;

        let body = get_from_itable(
            inline_table,
            BODY,
            || Ok(CollapseBasicProp::default_body(Theme::default(), state)),
            |v| (v, ViewState::from(state)).try_into(),
        )?;

        Ok(Self {
            container,
            header,
            body,
        })
    }
}

impl CollapseBasicProp {
    pub fn default_container(theme: Theme, state: CollapseState) -> ViewBasicProp {
        let mut container = ViewBasicProp::from_state(theme, state.into());
        container.set_height(Size::Fit);
        container.set_width(Size::Fill);
        container.set_background_visible(true);
        container.set_flow(Flow::Down);
        container.set_margin(Margin::from_f64(0.0));
        container.set_padding(Padding::from_f64(0.0));
        container.set_spacing(0.0);
        container
    }
    pub fn default_header(theme: Theme, state: CollapseState) -> ViewBasicProp {
        let mut header = ViewBasicProp::from_state(theme, state.into());
        header.set_height(Size::Fit);
        header.set_width(Size::Fill);
        header.set_background_visible(true);
        header.set_cursor(MouseCursor::Hand);
        header.set_flow(Flow::Right);
        header
    }
    pub fn default_body(theme: Theme, state: CollapseState) -> ViewBasicProp {
        let mut body = ViewBasicProp::from_state(theme, state.into());
        body.set_height(Size::Fit);
        body.set_width(Size::Fill);
        body.set_background_visible(true);
        body.set_margin(Margin::from_f64(0.0));
        body.set_flow(Flow::Down);
        body
    }
}

component_state! {
    CollapseState {
        Basic => BASIC,
        Hover => HOVER,
        Active => PRESSED,
        Disabled => DISABLED
    }, _ => CollapseState::Basic
}

impl ComponentState for CollapseState {
    fn is_disabled(&self) -> bool {
        false
    }
}

impl From<CollapseState> for ViewState {
    fn from(value: CollapseState) -> Self {
        match value {
            CollapseState::Basic => ViewState::Basic,
            CollapseState::Hover => ViewState::Hover,
            CollapseState::Active => ViewState::Pressed,
            CollapseState::Disabled => ViewState::Disabled,
        }
    }
}

impl From<ViewState> for CollapseState {
    fn from(value: ViewState) -> Self {
        match value {
            ViewState::Basic => CollapseState::Basic,
            ViewState::Hover => CollapseState::Hover,
            ViewState::Pressed => CollapseState::Active,
            ViewState::Disabled => CollapseState::Disabled,
        }
    }
}

impl From<CollapseState> for SvgState {
    fn from(value: CollapseState) -> Self {
        match value {
            CollapseState::Basic => SvgState::Basic,
            CollapseState::Hover => SvgState::Hover,
            CollapseState::Active => SvgState::Pressed,
            CollapseState::Disabled => SvgState::Disabled,
        }
    }
}

impl From<CollapseState> for LabelState {
    fn from(value: CollapseState) -> Self {
        match value {
            CollapseState::Basic | CollapseState::Hover | CollapseState::Active => {
                LabelState::Basic
            }
            CollapseState::Disabled => LabelState::Disabled,
        }
    }
}

component_part! {
    CollapsePart {
        Container => container => CONTAINER,
        Header => header => HEADER,
        Body => body => BODY
    }, CollapseState
}
