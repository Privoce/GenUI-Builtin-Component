use makepad_widgets::{Align, Margin, Padding};

pub trait NewFrom {
    fn from_f64(uni: f64) -> Self;
    fn from_xy(x: f64, y: f64) -> Self;
    fn from_all(x: f64, y: f64, z: f64, w: f64) -> Self;
}

impl NewFrom for Margin {
    fn from_f64(uni: f64) -> Self {
        Margin {
            top: uni,
            right: uni,
            bottom: uni,
            left: uni,
        }
    }

    fn from_xy(x: f64, y: f64) -> Self {
        Margin {
            top: x,
            right: y,
            bottom: x,
            left: y,
        }
    }

    fn from_all(x: f64, y: f64, z: f64, w: f64) -> Self {
        Margin {
            top: x,
            right: y,
            bottom: z,
            left: w,
        }
    }
}

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
