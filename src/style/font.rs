use druid_shell::kurbo::{self, Point};
use druid_shell::piet::{CairoText, CairoTextLayout, Color, FontFamily, Text, TextAttribute, LineMetric};
use druid_shell::piet::TextLayoutBuilder;
use guion::text::layout::{Direction, TxtLayout, TxtLayoutFromStor};
use guion::text::stor::TextStor;
use guion::validation::validated::Validated;
use render::rect2bounds;
use guion::util::bounds::{Bounds, Offset};
use guion::util::{bounds::Dims};
use guion::widget::cast::Statize;
use druid_shell::piet::TextLayout;
use crate::render::offset2point;

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

impl<E> TxtLayout<E> for Glyphs where E: Env {
    fn remove_chars(&mut self, range: std::ops::Range<usize>) {
        todo!()
    }

    fn push_chars(&mut self, off: usize, chars: &str) {
        todo!()
    }

    fn size(&self) -> Dims {
        ksize2dims(self.text.size())
    }

    fn char_at_display(&self, p: Offset) -> usize {
        off_char(self.text.hit_test_point(offset2point(p)).idx)
    }

    fn display_of_char(&self, c: usize) -> guion::util::bounds::Bounds {
        let p = self.text.hit_test_text_position(char_off(self.text.text(),c));
        let lm = self.text.line_metric(p.line).unwrap();
        Bounds::from_xywh(p.point.x as i32, lm.y_offset as i32, 0, lm.height as u32)
    }

    fn selection_bounds(&self, s: std::ops::Range<usize>) -> Vec<guion::util::bounds::Bounds> {
        let s = char_off(self.text.text(),s.start) .. char_off(self.text.text(),s.end);
        assert!(s.end >= s.start);
        self.text.rects_for_range(s).into_iter()
            .map(rect2bounds)
            .collect()
    }

    fn coord_of(&self, i: u32) -> Option<(u32,u32)> {
        let i = char_off(self.text.text(),i as usize);
        for test_line in 0..self.text.line_count() {
            let lm = self.text.line_metric(test_line).unwrap();
            if i >= lm.start_offset && i < lm.end_offset {
                let h = off_char(i - lm.start_offset);
                return Some((h as u32,test_line as u32));
            }
            if test_line+1 == self.text.line_count() && i >= lm.start_offset && i <= lm.end_offset {
                let h = off_char(i - lm.start_offset);
                return Some((h as u32,test_line as u32));
            }
        }
        None
    }

    fn at_coord(&self, (x,y): (u32,u32)) -> Option<u32> {
        if let Some(lm) = self.text.line_metric(y as usize) {
            let y = char_off(self.text.line_text(y as usize).unwrap(), x as usize);
            let i = lm.start_offset + y;
            Some(off_char(i) as u32)
        } else {
            None
        }
    }

    fn cursor_pos_reverse_line_centric(&self, line: u32, x: i32) -> Option<u32> {
        if let Some(lm) = self.text.line_metric(line as usize) {
            let p = Point {
                x: x as f64,
                y: lm.y_offset + lm.baseline,
            };
            let p = self.text.hit_test_point(p);
            let pos = p.idx - lm.start_offset;
            Some(off_char(pos) as u32)
        } else {
            None
        }
    }

    fn line_count(&self) -> u32 {
        self.text.line_count() as u32
    }

    fn chars(&self) -> usize {
        self.text.text().chars().count()
    }

    fn len(&self) -> usize {
        self.text.text().len()
    }

    fn move_cursor(&self, dir: Direction, off: usize) -> usize {
        fn line_of_char(s: &Glyphs, off: usize) -> (usize,usize,LineMetric) {
            for test_line in 0..s.text.line_count() {
                let lm = s.text.line_metric(test_line).unwrap();
                if off >= lm.start_offset && off < lm.end_offset {
                    return (test_line, off - lm.start_offset,lm);
                }
                if test_line+1 == s.text.line_count() && off >= lm.start_offset && off <= lm.end_offset {
                    return (test_line, off - lm.start_offset,lm);
                }
            }
            panic!()
        }
        fn last_char_before_in_str(s: &str, off: usize) -> usize {
            s.char_indices()
                .filter(|(o,_)| *o < off )
                .last()
                .map(|i| i.0 )
                .unwrap_or(0)
        }
        fn next_char_in_str(s: &str, off: usize) -> usize {
            s.char_indices()
                .filter(|(o,_)| *o > off )
                .next()
                .map(|i| i.0 )
                .unwrap_or(s.len())
        }

        match dir {
            Direction::Right => {
                let (line,_,lm) = line_of_char(self, off);
                // if it's >= lm.end_offset, then it's on the next line
                let nc = next_char_in_str(self.text.line_text(line).unwrap(), off - lm.start_offset) + lm.start_offset;
                if nc > lm.end_offset && line+1 >= self.text.line_count() {
                    off
                } else {
                    nc
                }
            },
            Direction::Left => {
                let (line,_,lm) = line_of_char(self, off);
                if off > lm.start_offset {
                    last_char_before_in_str(self.text.line_text(line).unwrap(), off - lm.start_offset) + lm.start_offset
                } else {
                    assert_eq!(off, lm.start_offset);
                    if line > 0 {
                        let lm = self.text.line_metric(line-1).unwrap();
                        let lt = self.text.line_text(line-1).unwrap();
                        last_char_before_in_str(lt, off - lm.start_offset) + lm.start_offset
                    } else {
                        0
                    }
                }
            },
            Direction::Down => {
                let (line,_,lm) = line_of_char(self, off);
                if line+1 >= self.text.line_count() {
                    return lm.end_offset;
                }
                let dlm = self.text.line_metric(line+1).unwrap();
                let tp = self.text.hit_test_text_position(off);
                let rh = self.text.hit_test_point(Point{
                    x: tp.point.x,
                    y: dlm.y_offset + dlm.baseline, //TODO intra-line hit
                });
                assert_eq!(line_of_char(self,rh.idx).0, line+1);
                rh.idx
            },
            Direction::Up => {
                let (line,_,_) = line_of_char(self, off);
                if line == 0 {
                    return 0;
                }
                let dlm = self.text.line_metric(line-1).unwrap();
                let tp = self.text.hit_test_text_position(off);
                let rh = self.text.hit_test_point(Point{
                    x: tp.point.x,
                    y: dlm.y_offset + dlm.baseline, //TODO intra-line hit
                });
                assert_eq!(line_of_char(self,rh.idx).0, line-1);
                rh.idx
            },
        }
    }

    fn char_len_l(&self, off: usize, chars: usize) -> usize {
        let mut oof = off;
        for _ in 0..chars {
            oof = TxtLayout::<E>::move_cursor(self,Direction::Left,oof);
        }
        off - oof
    }

    fn fix_boundary(&self, mut off: usize) -> usize {
        while !self.text.text().is_char_boundary(off) && off!=0 {
            off = off.saturating_sub(1); //TODO efficient algorithm
        }
        off
    }
}

impl<E,S> TxtLayoutFromStor<E,S> for Glyphs where E: Env, S: TextStor<E>+?Sized {
    fn from(s: &S, c: &mut E::Context) -> Self {
        Self {
            text: CairoText::new()
                .new_text_layout(s.caption().into_owned())
                .font(FontFamily::SANS_SERIF,16.0) //dead ass font fn use font props
                .default_attribute(TextAttribute::TextColor(Color::rgba8(255, 255, 255, 255))) //TODO take style ins Glyphs::generate
                .build().unwrap()
        }
    }

    fn update(&mut self, s: &S, c: &mut E::Context) {
        *self = TxtLayoutFromStor::<E,S>::from(s,c);
    }
}
/*impl<E> TxtLayoutFromStor<E,&str> for Glyphs where E: Env {
    fn from(s: &&str, c: &mut E::Context) -> Self {
        TxtLayoutFromStor::<E,str>::from(*s,c)
    }

    fn update(&mut self, s: &&str, c: &mut E::Context) {
        TxtLayoutFromStor::<E,str>::update(self,*s,c)
    }
}
impl<E> TxtLayoutFromStor<E,String> for Glyphs where E: Env {
    fn from(s: &String, c: &mut E::Context) -> Self {
        TxtLayoutFromStor::<E,str>::from(&*s,c)
    }

    fn update(&mut self, s: &String, c: &mut E::Context) {
        TxtLayoutFromStor::<E,str>::update(self,&*s,c)
    }
}
impl<T,E> TxtLayoutFromStor<E,Validated<E,T>> for Glyphs where T: TextStor<E>, Self: TxtLayoutFromStor<E,T>, E: Env {
    fn from(s: &Validated<E,T>, c: &mut E::Context) -> Self {
        TxtLayoutFromStor::<E,T>::from(&*s,c)
    }

    fn update(&mut self, s: &Validated<E,T>, c: &mut E::Context) {
        TxtLayoutFromStor::<E,T>::update(self,&*s,c)
    }
}*/

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

fn char_off(s: impl AsRef<str>, o: usize) -> usize {
    /*let s = s.as_ref();
    match s.char_indices().skip(o).next() {
        Some((i,_)) => i,
        None => s.len(),
    }*/
    o
}
fn off_char(bo: usize) -> usize {
    bo //TODO
}
