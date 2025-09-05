use makepad_widgets::*;
use toml_edit::Item;

use crate::{
    component_state, components::{
        live_props::LiveProps,
        traits::{BasicProp, ComponentState, Prop},
        view::ViewBasicProp,
    }, error::Error, get_get_mut, prop::{
        manuel::{
            ABS_POS, BACKGROUND_COLOR, BACKGROUND_VISIBLE, BASIC, BLUR_RADIUS, BORDER_RADIUS,
            CURSOR, HEIGHT, MARGIN, SHADOW_COLOR, SHADOW_OFFSET, SPREAD_RADIUS, THEME, WIDTH,
        },
        traits::{FromLiveColor, FromLiveValue, NewFrom},
        ApplyStateMapImpl, Radius,
    }, state_colors, themes::{Color, Theme, TomlValueTo}, interconvert_prop_toml, utils::get_from_itable
};

#[derive(Debug, Clone, Live, LiveHook, LiveRegister)]
#[live_ignore]
pub struct DividerProp {
    #[live(DividerBasicProp::default())]
    pub basic: DividerBasicProp,
}

impl Prop for DividerProp {
    type State = DividerState;

    type Basic = DividerBasicProp;

    get_get_mut! {
        DividerState::Basic => basic
    }

    fn len() -> usize {
        1 * DividerBasicProp::len()
    }

    fn sync(&mut self, map: &crate::prop::ApplyStateMap<Self::State>) -> ()
    where
        Self::State: Eq + std::hash::Hash + Copy,
    {
        map.sync(&mut self.basic, DividerState::Basic, []);
    }
}

impl Default for DividerProp {
    fn default() -> Self {
        Self {
            basic: Default::default(),
        }
    }
}

interconvert_prop_toml! {
    DividerProp {
        basic => BASIC, DividerBasicProp::default(), |v| (v, DividerState::Basic).try_into()
    }, "[component.divider] should be a table"
}

#[derive(Debug, Clone, Live, LiveHook, LiveRegister)]
#[live_ignore]
pub struct DividerBasicProp {
    #[live]
    pub theme: Theme,
    #[live]
    pub background_color: Vec4,
    #[live(Radius::new(4.0))]
    pub border_radius: Radius,
    #[live]
    pub shadow_color: Vec4,
    #[live(0.0)]
    pub spread_radius: f32,
    #[live(0.0)]
    pub blur_radius: f32,
    #[live(vec2(0.0, 0.0))]
    pub shadow_offset: Vec2,
    #[live(true)]
    pub background_visible: bool,
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

impl Default for DividerBasicProp {
    fn default() -> Self {
        Self::from_state(Theme::default(), DividerState::default())
    }
}

impl BasicProp for DividerBasicProp {
    type State = DividerState;
    /// (background_color, shadow_color)
    type Colors = (Color, Color);

    fn from_state(theme: crate::themes::Theme, state: Self::State) -> Self {
        let (background_color, shadow_color) = Self::state_colors(theme, state);
        Self {
            theme,
            background_color: background_color.into(),
            border_radius: Radius::new(1.0),
            shadow_color: shadow_color.into(),
            spread_radius: 0.0,
            blur_radius: 0.0,
            shadow_offset: vec2(0.0, 0.0),
            background_visible: true,
            margin: Margin::from_f64(0.0),
            cursor: MouseCursor::default(),
            height: Size::Fixed(1.2),
            width: Size::Fill,
            abs_pos: None,
        }
    }
    
    state_colors! {
        (bg_level, shadow_level),
        DividerState::Basic => (300, 400)
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
            BACKGROUND_COLOR => {
                let (background_color, _) = Self::state_colors(self.theme, state);
                self.background_color =
                    Vec4::from_live_color(value).unwrap_or(background_color.into());
            }
            BORDER_RADIUS => {
                self.border_radius = Radius::from_live_value(value).unwrap_or(Radius::new(1.0));
            }
            SHADOW_COLOR => {
                let (_, shadow_color) = Self::state_colors(self.theme, state);
                self.shadow_color = Vec4::from_live_color(value).unwrap_or(shadow_color.into());
            }
            SPREAD_RADIUS => {
                self.spread_radius = f32::from_live_value(value).unwrap_or(0.0);
            }
            BLUR_RADIUS => {
                self.blur_radius = f32::from_live_value(value).unwrap_or(0.0);
            }
            SHADOW_OFFSET => {
                self.shadow_offset = Vec2::from_live_value(value).unwrap_or(vec2(0.0, 0.0));
            }
            BACKGROUND_VISIBLE => {
                self.background_visible = bool::from_live_value(value).unwrap_or(true);
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
        let (background_color, shadow_color) = Self::state_colors(self.theme, state);
        self.background_color = background_color.into();
        self.shadow_color = shadow_color.into();
    }

    fn live_props() -> LiveProps {
        vec![
            (live_id!(theme), None.into()),
            (live_id!(background_color), None.into()),
            (live_id!(border_radius), None.into()),
            (live_id!(shadow_color), None.into()),
            (live_id!(spread_radius), None.into()),
            (live_id!(blur_radius), None.into()),
            (live_id!(shadow_offset), None.into()),
            (live_id!(background_visible), None.into()),
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

impl TryFrom<(&Item, DividerState)> for DividerBasicProp {
    type Error = Error;

    fn try_from((value, state): (&Item, DividerState)) -> Result<Self, Self::Error> {
        let inline_table = value.as_inline_table().ok_or(Error::ThemeStyleParse(
            "[components.divider.$state] should be an inline table".to_string(),
        ))?;

        let theme = Theme::default();
        let theme = get_from_itable(inline_table, THEME, || Ok(theme), |v| v.try_into())?;
        let (background_color, shadow_color) = Self::state_colors(theme, state);
        let background_color = get_from_itable(
            inline_table,
            BACKGROUND_COLOR,
            || Ok(background_color),
            |v| v.try_into(),
        )?
        .into();
        let border_radius = get_from_itable(
            inline_table,
            BORDER_RADIUS,
            || Ok(Radius::new(1.0)),
            |v| v.try_into(),
        )?;

        let shadow_color = get_from_itable(
            inline_table,
            SHADOW_COLOR,
            || Ok(shadow_color),
            |v| v.try_into(),
        )?
        .into();
        let spread_radius =
            get_from_itable(inline_table, SPREAD_RADIUS, || Ok(0.0), |v| v.to_f32())?;

        let blur_radius = get_from_itable(inline_table, BLUR_RADIUS, || Ok(0.0), |v| v.to_f32())?;
        let shadow_offset = vec2(0.0, 0.0);
        let shadow_offset = get_from_itable(
            inline_table,
            SHADOW_OFFSET,
            || Ok(shadow_offset),
            |v| v.to_vec2(shadow_offset),
        )?;
        let background_visible = get_from_itable(
            inline_table,
            BACKGROUND_VISIBLE,
            || Ok(true),
            |v| v.to_bool(),
        )?;
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
            background_color,
            border_radius,
            shadow_color,
            spread_radius,
            blur_radius,
            shadow_offset,
            background_visible,
            margin,
            cursor,
            height,
            width,
            abs_pos,
        })
    }
}

impl From<&DividerBasicProp> for ViewBasicProp {
    fn from(value: &DividerBasicProp) -> Self {
        let DividerBasicProp {
            theme,
            background_color,
            border_radius,
            shadow_color,
            spread_radius,
            blur_radius,
            shadow_offset,
            background_visible,
            margin,
            cursor,
            height,
            width,
            abs_pos,
        } = value;

        ViewBasicProp {
            theme: *theme,
            background_color: *background_color,
            border_radius: *border_radius,
            shadow_color: *shadow_color,
            spread_radius: *spread_radius,
            blur_radius: *blur_radius,
            shadow_offset: *shadow_offset,
            background_visible: *background_visible,
            margin: *margin,
            cursor: *cursor,
            height: *height,
            width: *width,
            abs_pos: *abs_pos,
            border_color: Default::default(),
            border_width: 0.0,
            rotation: 0.0,
            scale: 1.0,
            padding: Padding::default(),
            clip_x: false,
            clip_y: false,
            align: Align::default(),
            flow: Flow::default(),
            spacing: 0.0,
        }
    }
}

component_state! {
    DividerState {
        Basic => BASIC
    }, _ => DividerState::Basic
}

impl ComponentState for DividerState {
    fn is_disabled(&self) -> bool {
        false
    }
}
