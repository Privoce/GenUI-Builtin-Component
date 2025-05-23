use std::str::FromStr;

use makepad_widgets::Vec4;

use crate::error::Error;


pub struct Hex(pub Vec4);

impl FromStr for Hex {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}