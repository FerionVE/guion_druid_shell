use crate::ctx::state::DSState;
use crate::style::color::Color;
use crate::style::font::Font;
use crate::style::font::Glyphs;
use guion::aliases::ESColor;
use guion::env::Env;
use guion::render::TestStyle;
use guion::render::TestStyleV1;
use guion::render::TestStyleVariant;
use guion::render::WithTestStyle;
use guion::style::color::Color as GColor;
use guion::style::standard::cursor::StdCursor;
use guion::util::AsRefMut;
use guion::util::border::Border;
use self::selector::*;

use super::*;

pub mod font;
pub mod cursor;
pub mod default;
pub mod color;
pub mod selector;

#[derive(Clone)]
pub struct Style {
    font: Option<Font>,
    cursor: StdCursor,
}

impl<E> guion::style::Style<E> for Style where
    E: Env + Default + Sync,
    //E::Backend: GBackend<E,Style=Self>,
    //E::StyleSelector: Into<Selector<E>>,
    for<'a> E::Context<'a>: AsRefMut<DSState>,
{
    type Font = Font;
    type Cursor = StdCursor;
    type Color = Color;
    type Selector = Selector<E>;

    #[inline]
    fn font(&self, v: &Self::Selector, _: &mut E::Context<'_>) -> Option<&Self::Font> {
        todo!()
    }
    #[inline]
    fn cursor(&self, v: &Self::Selector, _: &mut E::Context<'_>) -> Self::Cursor {
        self.cursor.clone()
    }
    #[inline]
    fn color(&self, v: &Self::Selector, _: &mut E::Context<'_>) -> Self::Color {
        Color::from_rgba8(stupid_colors(v.clone().filled()))
    }
    #[inline]
    fn border(&self, v: &Self::Selector, _: &mut E::Context<'_>) -> Border {
        stupid_border(v.clone().filled())
    }

    fn and(&self, s: &Self) -> Self {
        self.clone() //TODO
    }

    // fn choose_font(css_font_desc: &str) -> Self {
    //     todo!()
    // }
}

impl AsRefMut<Self> for Style {
    #[inline]
    fn as_ref(&self) -> &Self {
        self
    }
    #[inline]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

pub fn stupid_border<E>(v: SelectorFilled<E>) -> Border where E: Env {
    match v {
        SelectorFilled{border: BorderPtr::Outer,design: Design::Flat,..} => Border::empty(),
        SelectorFilled{border: BorderPtr::Outer,..} => Border::uniform(2),
        SelectorFilled{border: BorderPtr::Visual,..} => Border::uniform(1),
        _ => Border::uniform(2),
    }
}

pub fn stupid_colors<E>(v: SelectorFilled<E>) -> [u8;4] where E: Env {
    match v {
        SelectorFilled{obj: Obj::Text,..} => [255,255,255,255],
        SelectorFilled{obj: Obj::Foreground,pressed: true,..} => [0,192,0,255],
        SelectorFilled{obj: Obj::Foreground,hovered: true,..} => [64,128,64,255],
        SelectorFilled{obj: Obj::Foreground,..} => [64,64,64,255],
        SelectorFilled{obj: Obj::Active,..} => [0,128,0,255],
        SelectorFilled{obj: Obj::Border,pressed: true,..} => [0,0,0,255],
        SelectorFilled{obj: Obj::Border,focused: true,..} => [255,127,0,255],
        SelectorFilled{obj: Obj::Border,..} => [0,255,0,255],
        SelectorFilled{obj: Obj::Background,..} => [32,32,32,255],
        SelectorFilled{obj: Obj::Default,..} => [32,32,32,255],
        _ => [127,0,0,255],
    }
}

pub fn stupid_test_style<E>() -> TestStyle<E> where E: Env {
    TestStyleV1 {
        default_border_color: ESColor::<E>::from_rgba8([0,255,0,255]),
        default_fg_color: ESColor::<E>::from_rgba8([64,64,64,255]),
        disabled_border_color: ESColor::<E>::from_rgba8([128,128,128,255]),
        disabled_fg_color: ESColor::<E>::from_rgba8([64,64,64,255]),
        hovered_border_color: None,
        hovered_fg_color: Some(ESColor::<E>::from_rgba8([64,128,64,255])),
        selected_border_color: Some(ESColor::<E>::from_rgba8([255,127,0,255])),
        selected_fg_color: None,
        activated_border_color: Some(ESColor::<E>::from_rgba8([0,0,0,255])),
        activated_fg_color: Some(ESColor::<E>::from_rgba8([0,192,0,255])),
        current_variant: Default::default(),
        bg_color: ESColor::<E>::from_rgba8([32,32,32,255]),
        text_color: ESColor::<E>::from_rgba8([255,255,255,255]),
        component_border: Border::uniform(1),
        spacing: Border::uniform(2),
        cursor: StdCursor::Default.into(),
        color_type: guion::render::TestStyleColorType::Bg,
        border_type: guion::render::TestStyleBorderType::Spacing,
    }.into()
}
