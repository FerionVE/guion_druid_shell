use druid_shell::piet::Color as DSColor;

use super::*;
#[derive(Clone,PartialEq)]
pub struct Color(pub DSColor);

impl guion::style::color::Color for Color {
    #[inline]
    fn from_rgba8(c: [u8;4]) -> Self {
        Self(DSColor::rgba8(c[0], c[1], c[2], c[3]))
    }
    #[inline]
    fn into_rgba8(&self) -> [u8;4] {
        let (r,g,b,a) = self.0.as_rgba8();
        [r,g,b,a]
    }
}
