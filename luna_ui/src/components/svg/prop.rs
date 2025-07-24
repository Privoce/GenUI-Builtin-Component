use makepad_widgets::*;
use toml_edit::Item;

use crate::{
    component_state,
    components::traits::{BasicProp, ComponentState, Prop},
    error::Error,
    prop::{
        manuel::{ABS_POS, BACKGROUND_COLOR, BASIC, COLOR, CURSOR, HEIGHT, MARGIN, THEME, WIDTH},
        traits::{FromLiveColor, FromLiveValue, NewFrom},
        ApplyStateMapImpl,
    },
    themes::{Color, Theme, TomlValueTo},
    utils::{get_from_itable, get_from_table},
};

#[derive(Debug, Clone, Live, LiveHook, LiveRegister)]
#[live_ignore]
pub struct SvgProp {
    #[live(SvgBasicProp::default())]
    pub basic: SvgBasicProp,
}

impl Prop for SvgProp {
    type State = SvgState;

    type Basic = SvgBasicProp;

    fn get(&self, state: Self::State) -> &Self::Basic {
        match state {
            SvgState::Basic => &self.basic,
        }
    }

    fn get_mut(&mut self, state: Self::State) -> &mut Self::Basic {
        match state {
            SvgState::Basic => &mut self.basic,
        }
    }

    fn len() -> usize {
        1 * SvgBasicProp::len()
    }

    fn sync(&mut self, map: &crate::prop::ApplyStateMap<Self::State>) -> ()
    where
        Self::State: Eq + std::hash::Hash + Copy,
    {
        map.sync(&mut self.basic, SvgState::Basic, []);
    }
}

impl Default for SvgProp {
    fn default() -> Self {
        Self {
            basic: Default::default(),
        }
    }
}

impl TryFrom<&Item> for SvgProp {
    type Error = Error;

    fn try_from(value: &Item) -> Result<Self, Self::Error> {
        let table = value.as_table().ok_or(Error::ThemeStyleParse(
            "[components.svg] should be a table".to_string(),
        ))?;

        let basic = get_from_table(
            table,
            BASIC,
            || Ok(SvgBasicProp::default()),
            |item| (item, SvgState::Basic).try_into(),
        )?;

        Ok(Self { basic })
    }
}

#[derive(Debug, Clone, Live, LiveHook, LiveRegister)]
#[live_ignore]
pub struct SvgBasicProp {
    #[live]
    pub theme: Theme,
    #[live]
    pub color: Vec4,
    #[live(Margin::from_f64(0.0))]
    pub margin: Margin,
    #[live(MouseCursor::default())]
    pub cursor: MouseCursor,
    #[live(Size::Fixed(1.2))]
    pub height: Size,
    #[live(Size::Fill)]
    pub width: Size,
    #[live(None)]
    pub abs_pos: Option<DVec2>,
}

impl Default for SvgBasicProp {
    fn default() -> Self {
        Self::from_state(Theme::default(), SvgState::default())
    }
}

impl BasicProp for SvgBasicProp {
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
            height: Size::Fixed(1.2),
            width: Size::Fill,
            abs_pos: None,
        }
    }

    fn state_colors(theme: crate::themes::Theme, state: Self::State) -> Self::Colors {
        let color_level = match state {
            SvgState::Basic => 500,
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
        12
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
                self.height = Size::from_live_value(value).unwrap_or(Size::Fixed(1.2));
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
        let color = Self::state_colors(self.theme, state);
        self.color = color.into();
    }

    fn live_props() -> Vec<(LiveId, Option<Vec<LiveId>>)> {
        vec![
            (live_id!(theme), None),
            (live_id!(color), None),
            (
                live_id!(margin),
                Some(vec![
                    live_id!(top),
                    live_id!(bottom),
                    live_id!(left),
                    live_id!(right),
                ]),
            ),
            (live_id!(cursor), None),
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
            clip_x: false,
            clip_y: false,
            ..Default::default()
        }
    }
}

impl TryFrom<(&Item, SvgState)> for SvgBasicProp {
    type Error = Error;

    fn try_from((value, state): (&Item, SvgState)) -> Result<Self, Self::Error> {
        let inline_table = value.as_inline_table().ok_or(Error::ThemeStyleParse(
            "[components.divider.$state] should be an inline table".to_string(),
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
            || Ok(Size::Fixed(1.2)),
            |v| v.to_size(),
        )?;
        let width = get_from_itable(inline_table, WIDTH, || Ok(Size::Fill), |v| v.to_size())?;
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
        Basic => BASIC
    }, _ => SvgState::Basic
}

impl ComponentState for SvgState {
    fn is_disabled(&self) -> bool {
        false
    }
}
