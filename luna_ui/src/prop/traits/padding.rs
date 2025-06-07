use makepad_widgets::Padding;
use crate::prop::traits::NewFrom;

impl NewFrom for Padding {
    fn from_f64(uni: f64) -> Self {
        Padding {
            top: uni,
            right: uni,
            bottom: uni,
            left: uni,
        }
    }

    fn from_xy(x: f64, y: f64) -> Self {
        Padding {
            top: x,
            right: y,
            bottom: x,
            left: y,
        }
    }

    fn from_all(x: f64, y: f64, z: f64, w: f64) -> Self {
        Padding {
            top: x,
            right: y,
            bottom: z,
            left: w,
        }
    }
}