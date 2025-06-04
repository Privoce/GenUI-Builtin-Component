use makepad_widgets::{Align, Margin, Padding};

pub trait NewFrom {
    fn from_f64(uni: f64) -> Self;
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
}

impl NewFrom for Align {
    fn from_f64(uni: f64) -> Self {
        Align {
            x: uni,
            y: uni,
        }
    }
}