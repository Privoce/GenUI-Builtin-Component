use makepad_widgets::{image_cache::ImageFit, *};

use crate::{
    basic_prop_interconvert, component_state,
    components::{
        live_props::LiveProps,
        traits::{BasicStyle, ComponentState, Style},
    },
    error::Error,
    get_get_mut,
    prop::{
        manuel::{
            ABS_POS, BASIC, CURSOR, FIT, HEIGHT, LOADING, MARGIN, MIN_HEIGHT, MIN_WIDTH, THEME,
            WIDTH, WIDTH_SCALE,
        },
        traits::{AbsPos, FromLiveValue, NewFrom, ToTomlValue},
        ApplyStateMapImpl,
    },
    prop_interconvert,
    themes::{Theme, TomlValueTo},
};

prop_interconvert! {
    ImageProp {
        basic_prop = ImageBasicStyle;
        basic => BASIC, ImageBasicStyle::default(), |v| (v, ImageState::Basic).try_into(),
        loading => LOADING, ImageBasicStyle::default(), |v| (v, ImageState::Loading).try_into()
    }, "[component.image] should be a table"
}

impl Style for ImageProp {
    type State = ImageState;

    type Basic = ImageBasicStyle;

    get_get_mut! {
        ImageState::Basic => basic,
        ImageState::Loading => loading
    }

    fn len() -> usize {
        1 * ImageBasicStyle::len()
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


basic_prop_interconvert! {
    ImageBasicStyle {
        state = ImageState;
        {};
        {
            fit: ImageFit => FIT, ImageFit::default(), |v| v.to_image_fit(),
            height: Size => HEIGHT, Size::Fixed(64.0), |v| v.to_size(),
            width: Size => WIDTH, Size::Fixed(128.0), |v| v.to_size(),
            margin: Margin => MARGIN, Margin::from_f64(0.0), |v| v.to_margin(margin),
            cursor: MouseCursor => CURSOR, MouseCursor::Default, |v| v.to_cursor(),
            abs_pos: AbsPos => ABS_POS, None, |v| Ok(v.to_dvec2().map_or(None, |v| Some(v))),
            min_width: f64 => MIN_WIDTH, 128.0, |v| v.to_f64(),
            min_height: f64 => MIN_HEIGHT, 64.0, |v| v.to_f64(),
            width_scale: f64 => WIDTH_SCALE, 1.0, |v| v.to_f64()
        }
    }, "[component.image.$state] should be an inline table"
}

impl BasicStyle for ImageBasicStyle {
    type State = ImageState;

    type Colors = ();

    fn from_state(theme: crate::themes::Theme, state: Self::State) -> Self {
        let cursor = if state.is_disabled() {
            MouseCursor::NotAllowed
        } else {
            MouseCursor::Hand
        };
        Self {
            theme,
            fit: ImageFit::default(),
            height: Size::Fixed(64.0),
            width: Size::Fixed(128.0),
            margin: Margin::from_f64(0.0),
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
            THEME => {
                self.theme = Theme::from_live_value(value).unwrap_or(Theme::default());
            }
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
                self.margin = Margin::from_live_value(value).unwrap_or(Margin::from_f64(0.0));
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
