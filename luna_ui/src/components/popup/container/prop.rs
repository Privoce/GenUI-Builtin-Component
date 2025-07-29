use makepad_widgets::*;
use toml_edit::{InlineTable, Item, Value};

use crate::{
    component_state,
    components::{
        popup::PopupState,
        traits::{BasicProp, ComponentState, Prop},
        view::ViewBasicProp,
    },
    error::Error,
    prop::{
        manuel::{
            ABS_POS, ALIGN, BACKGROUND_COLOR, BACKGROUND_VISIBLE, BASIC, CLIP_X, CLIP_Y, CURSOR,
            FLOW, HEIGHT, MARGIN, PADDING, SPACING, THEME, WIDTH,
        },
        traits::{FromLiveColor, FromLiveValue, NewFrom},
        ApplyStateMapImpl,
    },
    themes::{Color, Theme, TomlValueTo},
    try_from_toml_item,
    utils::get_from_itable,
};

#[derive(Debug, Clone, Live, LiveHook, LiveRegister)]
#[live_ignore]
pub struct PopupContainerProp {
    #[live(PopupContainerBasicProp::default())]
    pub basic: PopupContainerBasicProp,
}

impl Default for PopupContainerProp {
    fn default() -> Self {
        Self {
            basic: PopupContainerBasicProp::default(),
        }
    }
}

impl Prop for PopupContainerProp {
    type State = PopupState;

    type Basic = PopupContainerBasicProp;

    fn get(&self, state: Self::State) -> &Self::Basic {
        match state {
            PopupState::Basic => &self.basic,
        }
    }

    fn get_mut(&mut self, state: Self::State) -> &mut Self::Basic {
        match state {
            PopupState::Basic => &mut self.basic,
        }
    }

    fn len() -> usize {
        PopupContainerBasicProp::len()
    }

    fn sync(&mut self, map: &crate::prop::ApplyStateMap<Self::State>) -> ()
    where
        Self::State: Eq + std::hash::Hash + Copy,
    {
        map.sync(&mut self.basic, PopupState::Basic, []);
    }
}

try_from_toml_item! {
    PopupContainerProp {
        basic => BASIC, PopupContainerBasicProp::default(),|v| (v, PopupState::Basic).try_into()
    }, "[component.popup] should be a table"
}

#[derive(Debug, Clone, Live, LiveHook, LiveRegister, Copy)]
#[live_ignore]
pub struct PopupContainerBasicProp {
    #[live]
    pub theme: Theme,
    #[live]
    pub background_color: Vec4,
    #[live(true)]
    pub background_visible: bool,
    #[live(Padding::from_f64(6.0))]
    pub padding: Padding,
    #[live(Margin::from_f64(0.0))]
    pub margin: Margin,
    #[live(false)]
    pub clip_x: bool,
    #[live(false)]
    pub clip_y: bool,
    #[live(Align{
        x: 0.5,
        y: 0.5,
    })]
    pub align: Align,
    #[live(MouseCursor::default())]
    pub cursor: MouseCursor,
    #[live(Flow::Down)]
    pub flow: Flow,
    #[live(6.0)]
    pub spacing: f64,
    #[live(Size::Fill)]
    pub height: Size,
    #[live(Size::Fill)]
    pub width: Size,
    #[live(None)]
    pub abs_pos: Option<DVec2>,
}

impl BasicProp for PopupContainerBasicProp {
    type State = PopupState;

    type Colors = Color;

    fn len() -> usize {
        14
    }

    fn set_from_str(&mut self, key: &str, value: &LiveValue, state: Self::State) -> () {
        match key {
            THEME => {
                self.theme = Theme::from_live_value(value).unwrap_or(Theme::default());
                self.sync(state);
            }
            BACKGROUND_COLOR => {
                let background_color = Self::state_colors(self.theme, state);
                self.background_color =
                    Vec4::from_live_color(value).unwrap_or(background_color.into());
            }
            BACKGROUND_VISIBLE => {
                self.background_visible = bool::from_live_value(value).unwrap_or(true);
            }
            PADDING => {
                self.padding = Padding::from_live_value(value).unwrap_or(Padding::from_f64(6.0));
            }
            MARGIN => {
                self.margin = Margin::from_live_value(value).unwrap_or(Margin::from_f64(0.0));
            }
            CLIP_X => {
                self.clip_x = bool::from_live_value(value).unwrap_or(false);
            }
            CLIP_Y => {
                self.clip_y = bool::from_live_value(value).unwrap_or(false);
            }
            ALIGN => {
                self.align = Align::from_live_value(value).unwrap_or(Align { x: 0.5, y: 0.5 });
            }
            CURSOR => {
                let cursor = if state.is_disabled() {
                    MouseCursor::NotAllowed
                } else {
                    MouseCursor::Default
                };
                self.cursor = MouseCursor::from_live_value(value).unwrap_or(cursor);
            }
            FLOW => {
                self.flow = Flow::from_live_value(value).unwrap_or(Flow::Down);
            }
            SPACING => {
                self.spacing = f64::from_live_value(value).unwrap_or(6.0);
            }
            HEIGHT => {
                self.height = Size::from_live_value(value).unwrap_or(Size::Fill);
            }
            WIDTH => {
                self.width = Size::from_live_value(value).unwrap_or(Size::Fill);
            }
            ABS_POS => {
                self.abs_pos = DVec2::from_live_value(value);
            }
            _ => {}
        }
    }

    fn sync(&mut self, state: Self::State) -> () {
        let background_color = Self::state_colors(self.theme, state);
        self.background_color = background_color.into();
    }

    fn from_state(theme: Theme, state: Self::State) -> Self {
        let background_color = Self::state_colors(theme, state);

        let cursor = if state.is_disabled() {
            MouseCursor::NotAllowed
        } else {
            MouseCursor::default()
        };

        Self {
            theme,
            background_color: background_color.into(),
            background_visible: true,
            padding: Padding::from_f64(6.0),
            margin: Margin::from_f64(0.0),
            clip_x: false,
            clip_y: false,
            align: Align { x: 0.5, y: 0.5 },
            cursor,
            flow: Flow::Down,
            spacing: 6.0,
            height: Size::Fill,
            width: Size::Fill,
            abs_pos: None,
        }
    }

    fn state_colors(theme: Theme, state: Self::State) -> Self::Colors {
        let bg_level = match state {
            PopupState::Basic => 300,
        };

        match theme {
            Theme::Dark => Theme::Dark.color(bg_level),
            Theme::Primary => Theme::Primary.color(bg_level),
            Theme::Error => Theme::Error.color(bg_level),
            Theme::Warning => Theme::Warning.color(bg_level),
            Theme::Success => Theme::Success.color(bg_level),
            Theme::Info => Theme::Info.color(bg_level),
        }
    }
    fn live_props() -> Vec<(LiveId, Option<Vec<LiveId>>)> {
        vec![
            (live_id!(theme), None),
            (live_id!(background_color), None),
            (live_id!(border_color), None),
            (live_id!(border_width), None),
            (
                live_id!(border_radius),
                Some(vec![
                    live_id!(top),
                    live_id!(bottom),
                    live_id!(left),
                    live_id!(right),
                ]),
            ),
            (live_id!(shadow_color), None),
            (live_id!(spread_radius), None),
            (live_id!(blur_radius), None),
            (live_id!(shadow_offset), None),
            (live_id!(background_visible), None),
            (live_id!(rotation), None),
            (live_id!(scale), None),
            (
                live_id!(padding),
                Some(vec![
                    live_id!(top),
                    live_id!(bottom),
                    live_id!(left),
                    live_id!(right),
                ]),
            ),
            (
                live_id!(margin),
                Some(vec![
                    live_id!(top),
                    live_id!(bottom),
                    live_id!(left),
                    live_id!(right),
                ]),
            ),
            (live_id!(clip_x), None),
            (live_id!(clip_y), None),
            (live_id!(align), None),
            (live_id!(cursor), None),
            (live_id!(flow), None),
            (live_id!(spacing), None),
            (live_id!(height), None),
            (live_id!(width), None),
            (live_id!(abs_pos), None),
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
            clip_x: self.clip_x,
            clip_y: self.clip_y,
            padding: self.padding,
            align: self.align,
            flow: self.flow,
            spacing: self.spacing,
            ..Default::default()
        }
    }
}

impl Default for PopupContainerBasicProp {
    fn default() -> Self {
        PopupContainerBasicProp::from_state(Theme::default(), PopupState::Basic)
    }
}

impl TryFrom<(&Value, PopupState)> for PopupContainerBasicProp {
    type Error = Error;

    fn try_from((value, state): (&Value, PopupState)) -> Result<Self, Self::Error> {
        let inline_table = value.as_inline_table().ok_or(Error::ThemeStyleParse(
            "[components.view.$state] should be an inline table".to_string(),
        ))?;
        (inline_table, state).try_into()
    }
}

impl TryFrom<(&Item, PopupState)> for PopupContainerBasicProp {
    type Error = Error;

    fn try_from((value, state): (&Item, PopupState)) -> Result<Self, Self::Error> {
        let inline_table = value.as_inline_table().ok_or(Error::ThemeStyleParse(
            "[components.view.$state] should be an inline table".to_string(),
        ))?;
        (inline_table, state).try_into()
    }
}

impl TryFrom<(&InlineTable, PopupState)> for PopupContainerBasicProp {
    type Error = Error;

    fn try_from((inline_table, state): (&InlineTable, PopupState)) -> Result<Self, Self::Error> {
        let theme = Theme::default();
        let theme = get_from_itable(inline_table, THEME, || Ok(theme), |v| v.try_into())?;

        let background_color = Self::state_colors(theme, state);

        let background_color = get_from_itable(
            inline_table,
            BACKGROUND_COLOR,
            || Ok(background_color),
            |v| v.try_into(),
        )?
        .into();

        let background_visible = get_from_itable(
            inline_table,
            BACKGROUND_VISIBLE,
            || Ok(true),
            |v| v.to_bool(),
        )?;

        let padding = Padding::from_f64(6.0);
        let padding = get_from_itable(
            inline_table,
            PADDING,
            || Ok(padding),
            |v| v.to_padding(padding),
        )?;
        let margin = Margin::from_f64(0.0);
        let margin = get_from_itable(inline_table, MARGIN, || Ok(margin), |v| v.to_margin(margin))?;
        let clip_x = get_from_itable(inline_table, CLIP_X, || Ok(false), |v| v.to_bool())?;
        let clip_y = get_from_itable(inline_table, CLIP_Y, || Ok(false), |v| v.to_bool())?;
        let align = Align { x: 0.5, y: 0.5 };
        let align = get_from_itable(inline_table, ALIGN, || Ok(align), |v| v.to_align(align))?;
        let cursor = if state.is_disabled() {
            MouseCursor::NotAllowed
        } else {
            MouseCursor::default()
        };
        let cursor = get_from_itable(inline_table, CURSOR, || Ok(cursor), |v| v.to_cursor())?;
        let flow = get_from_itable(inline_table, FLOW, || Ok(Flow::Down), |v| v.to_flow())?;
        let spacing = get_from_itable(inline_table, SPACING, || Ok(6.0), |v| v.to_f64())?;
        let height = get_from_itable(inline_table, HEIGHT, || Ok(Size::Fill), |v| v.to_size())?;
        let width = get_from_itable(inline_table, WIDTH, || Ok(Size::Fill), |v| v.to_size())?;
        let abs_pos = get_from_itable(
            inline_table,
            ABS_POS,
            || Ok(None),
            |v| v.to_dvec2().map(Some),
        )?;

        Ok(Self {
            theme,
            background_color,
            background_visible,
            padding,
            margin,
            clip_x,
            clip_y,
            align,
            cursor,
            flow,
            spacing,
            height,
            width,
            abs_pos,
        })
    }
}

impl From<&PopupContainerBasicProp> for ViewBasicProp {
    fn from(value: &PopupContainerBasicProp) -> Self {
        let PopupContainerBasicProp {
            theme,
            background_color,
            background_visible,
            padding,
            margin,
            clip_x,
            clip_y,
            align,
            cursor,
            flow,
            spacing,
            height,
            width,
            abs_pos,
        } = *value;

        ViewBasicProp {
            theme,
            background_color,
            background_visible,
            padding,
            margin,
            clip_x,
            clip_y,
            align,
            cursor,
            flow,
            spacing,
            height,
            width,
            abs_pos,
            ..Default::default()
        }
    }
}

// component_state! {
//     PopupState {
//         Basic => BASIC
//     }, _ => PopupState::Basic
// }

// impl ComponentState for PopupState {
//     fn is_disabled(&self) -> bool {
//         false
//     }
// }
