use druid_shell::kurbo;
use druid_shell::piet::{CairoText, CairoTextLayout, Color, FontFamily, Text, TextAttribute};
use druid_shell::piet::TextLayoutBuilder;
use std::path::Path;
use guion::util::bounds::Offset;
use guion::util::{bounds::Dims};
use guion::style::font::Glyphs as GGlyphs;
use guion::{widget::cast::Statize, style::font::{GlyphInfo,CrazyWorkaroundPPIter}};
use druid_shell::piet::TextLayout;
use super::*;

#[derive(Clone,PartialEq)]
pub struct Font {
    /*pub source: FontSource,
    pub index: u32,
    pub size: u16,
    //pub style: FontStyle, //TODO fix
    pub render: FontRender,*/
}
/*#[derive(Clone,PartialEq)]
pub enum FontSource {
    File(&'static Path),
    Memory(&'static [u8]),
}
#[derive(Clone,PartialEq)]
pub enum FontRender {
    Solid(),
    Shaded(),
    Blended(),
    BlendedWrapped(Color,u32),
}*/

pub struct Glyphs {
    text: CairoTextLayout,
}

impl<E> GGlyphs<E> for Glyphs where
    E: Env + Sync,
    E::Context: AsRefMut<DSState>
{
    type Glyph = GlyphInfo;

    #[inline]
    fn size(&self) -> Dims { 
        ksize2dims(self.text.size())
    }
    #[inline]
    fn line_ascent(&self) -> u32 {
        todo!()
    }
    #[inline]
    fn line_height(&self) -> u32 {
        todo!()
    }
    #[inline]
    fn line_distance(&self) -> u32 {
        todo!()
    }
    fn lines<'s>(&'s self) -> CrazyWorkaroundPPIter<'s,GlyphInfo> {
        todo!()
    }
    fn generate(s: &str, b: (f32,f32), ctx: &mut E::Context) -> Self {
        //TODO W H A T colorz are stuck inside here ?!?!?!?!? ok we already wanted to use style here
        Glyphs{
            text: CairoText::new()
                .new_text_layout(s.to_owned())
                .font(FontFamily::SANS_SERIF,16.0) //dead ass font fn use font props
                .default_attribute(TextAttribute::TextColor(Color::rgba8(255, 255, 255, 255))) //TODO take style ins Glyphs::generate
                .build().unwrap()
        }
    }
}

unsafe impl<E> Statize<E> for Glyphs {
    type Statur = Glyphs;
}

impl AsRefMut<Self> for Glyphs {
    #[inline]
    fn as_ref(&self) -> &Self {
        self
    }
    #[inline]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

impl AsRefMut<CairoTextLayout> for Glyphs {
    #[inline]
    fn as_ref(&self) -> &CairoTextLayout {
        &self.text
    }
    #[inline]
    fn as_mut(&mut self) -> &mut CairoTextLayout {
        &mut self.text
    }
}

pub(crate) fn ksize2dims(s: kurbo::Size) -> Dims {
    Dims {
        w: s.width as u32,
        h: s.height as u32,
    }
}