use makepad_widgets::{image_cache::ImageFit, *};
use toml_edit::Item;

use crate::{
    component_state, components::{
        live_props::LiveProps,
        traits::{BasicProp, ComponentState, Prop},
    }, error::Error, get_get_mut, prop::{
        manuel::{
            ABS_POS, BASIC, CURSOR, FIT, HEIGHT, LOADING, MARGIN, MIN_HEIGHT, MIN_WIDTH, WIDTH,
            WIDTH_SCALE,
        },
        traits::{FromLiveValue, NewFrom},
        ApplyStateMapImpl,
    }, themes::{Theme, TomlValueTo}, try_from_toml_item, utils::get_from_itable
};

#[derive(Debug, Clone, Live, LiveHook, LiveRegister)]
#[live_ignore]
pub struct ImageProp {
    #[live(ImageBasicProp::default())]
    pub basic: ImageBasicProp,
    #[live(ImageBasicProp::from_state(Theme::default(), ImageState::Loading))]
    pub loading: ImageBasicProp,
}

impl Default for ImageProp {
    fn default() -> Self {
        Self {
            basic: Default::default(),
            loading: ImageBasicProp::from_state(Theme::default(), ImageState::Loading),
        }
    }
}

try_from_toml_item! {
    ImageProp {
        basic => BASIC, ImageBasicProp::default(), |v| (v, ImageState::Basic).try_into(),
        loading => LOADING, ImageBasicProp::default(), |v| (v, ImageState::Loading).try_into()
    }, "[component.image] should be a table"
}

impl Prop for ImageProp {
    type State = ImageState;

    type Basic = ImageBasicProp;

    get_get_mut! {
        ImageState::Basic => basic,
        ImageState::Loading => loading
    }

    fn len() -> usize {
        1 * ImageBasicProp::len()
    }

    fn sync(&mut self, map: &crate::prop::ApplyStateMap<Self::State>) -> ()
    where
        Self::State: Eq + std::hash::Hash + Copy,
    {
        map.sync(
            &mut self.basic,
            ImageState::Basic,
            [(ImageState::Loading, &mut self.loading)],
        );
    }
}

#[derive(Debug, Clone, Live, LiveHook, LiveRegister)]
#[live_ignore]
pub struct ImageBasicProp {
    #[live(ImageFit::default())]
    pub fit: ImageFit,
    #[live(Size::Fixed(64.0))]
    pub height: Size,
    #[live(Size::Fixed(128.0))]
    pub width: Size,
    #[live(Margin::from_f64(6.0))]
    pub margin: Margin,
    #[live(MouseCursor::default())]
    pub cursor: MouseCursor,
    #[live(None)]
    pub abs_pos: Option<DVec2>,
    #[live(128.0)]
    pub min_width: f64,
    #[live(64.0)]
    pub min_height: f64,
    #[live(1.0)]
    pub width_scale: f64,
}

impl Default for ImageBasicProp {
    fn default() -> Self {
        Self {
            fit: ImageFit::default(),
            height: Size::Fixed(64.0),
            width: Size::Fixed(128.0),
            margin: Margin::from_f64(6.0),
            cursor: MouseCursor::default(),
            abs_pos: None,
            min_width: 128.0,
            min_height: 64.0,
            width_scale: 1.0,
        }
    }
}

impl TryFrom<(&Item, ImageState)> for ImageBasicProp {
    type Error = Error;

    fn try_from((value, state): (&Item, ImageState)) -> Result<Self, Self::Error> {
        let inline_table = value.as_inline_table().ok_or(Error::ThemeStyleParse(
            "[component.image.$state] should be an inline table".to_string(),
        ))?;

        let fit = get_from_itable(
            inline_table,
            FIT,
            || Ok(ImageFit::default()),
            |v| v.to_image_fit(),
        )?;

        let cursor = if state.is_disabled() {
            MouseCursor::NotAllowed
        } else {
            MouseCursor::Hand
        };

        let cursor = get_from_itable(inline_table, CURSOR, || Ok(cursor), |v| v.to_cursor())?;
        let margin = Margin::from_f64(6.0);
        let margin = get_from_itable(inline_table, MARGIN, || Ok(margin), |v| v.to_margin(margin))?;
        let height = get_from_itable(
            inline_table,
            HEIGHT,
            || Ok(Size::Fixed(64.0)),
            |v| v.to_size(),
        )?;
        let width = get_from_itable(
            inline_table,
            WIDTH,
            || Ok(Size::Fixed(128.0)),
            |v| v.to_size(),
        )?;
        let abs_pos = get_from_itable(
            inline_table,
            ABS_POS,
            || Ok(None),
            |v| v.to_dvec2().map(Some),
        )?;

        let min_width = get_from_itable(inline_table, MIN_WIDTH, || Ok(128.0), |v| v.to_f64())?;
        let min_height = get_from_itable(inline_table, MIN_HEIGHT, || Ok(64.0), |v| v.to_f64())?;
        let width_scale = get_from_itable(inline_table, WIDTH_SCALE, || Ok(1.0), |v| v.to_f64())?;

        Ok(Self {
            fit,
            height,
            width,
            margin,
            cursor,
            abs_pos,
            min_width,
            min_height,
            width_scale,
        })
    }
}

impl BasicProp for ImageBasicProp {
    type State = ImageState;

    type Colors = ();

    fn from_state(_theme: crate::themes::Theme, state: Self::State) -> Self {
        let cursor = if state.is_disabled() {
            MouseCursor::NotAllowed
        } else {
            MouseCursor::Hand
        };
        Self {
            fit: ImageFit::default(),
            height: Size::Fixed(64.0),
            width: Size::Fixed(128.0),
            margin: Margin::from_f64(6.0),
            cursor,
            abs_pos: None,
            min_width: 128.0,
            min_height: 64.0,
            width_scale: 1.0,
        }
    }

    fn state_colors(_theme: crate::themes::Theme, _state: Self::State) -> Self::Colors {
        ()
    }

    fn len() -> usize {
        9
    }

    fn set_from_str(&mut self, key: &str, value: &LiveValue, state: Self::State) -> () {
        match key {
            FIT => {
                self.fit = ImageFit::from_live_value(value).unwrap_or(ImageFit::default());
            }
            HEIGHT => {
                self.height = Size::from_live_value(value).unwrap_or(Size::Fit);
            }
            WIDTH => {
                self.width = Size::from_live_value(value).unwrap_or(Size::Fit);
            }
            MARGIN => {
                self.margin = Margin::from_live_value(value).unwrap_or(Margin::from_f64(6.0));
            }
            CURSOR => {
                let cursor = if state.is_disabled() {
                    MouseCursor::NotAllowed
                } else {
                    MouseCursor::Hand
                };
                self.cursor = MouseCursor::from_live_value(value).unwrap_or(cursor);
            }
            ABS_POS => {
                self.abs_pos = DVec2::from_live_value(value);
            }
            MIN_WIDTH => {
                self.min_width = f64::from_live_value(value).unwrap_or(128.0);
            }
            MIN_HEIGHT => {
                self.min_height = f64::from_live_value(value).unwrap_or(64.0);
            }
            WIDTH_SCALE => {
                self.width_scale = f64::from_live_value(value).unwrap_or(1.0);
            }
            _ => {}
        }
    }

    fn sync(&mut self, _state: Self::State) -> () {
        ()
    }

    fn live_props() -> LiveProps {
        vec![
            (live_id!(fit), None.into()),
            (live_id!(height), None.into()),
            (live_id!(width), None.into()),
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
        Layout::default()
    }
}

component_state! {
    ImageState {
        Basic => BASIC,
        Loading => LOADING
    }, _ => ImageState::Basic
}

impl ComponentState for ImageState {
    fn is_disabled(&self) -> bool {
        // matches!(self, ImageState::Disabled)
        false
    }
}
