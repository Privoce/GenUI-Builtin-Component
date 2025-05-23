mod hex;
mod rgb;
mod rgba;

pub use hex::Hex;
pub use rgb::RGB;
pub use rgba::RGBA;

pub enum Color {
    Hex(Hex),
    RGB(RGB),
    RGBA(RGBA),
}