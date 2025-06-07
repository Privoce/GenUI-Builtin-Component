use makepad_widgets::Align;
use super::NewFrom;

impl NewFrom for Align {
    fn from_f64(uni: f64) -> Self {
        Align { x: uni, y: uni }
    }

    fn from_xy(x: f64, y: f64) -> Self {
        Align { x, y }
    }
    fn from_all(x: f64, y: f64, _z: f64, _w: f64) -> Self {
        Align { x, y }
    }
}
