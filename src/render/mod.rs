use druid_shell::{WindowHandle, piet};
use druid_shell::kurbo::{Rect, Point};
use druid_shell::piet::{Piet, RenderContext};
use guion::aliases::{EStyle, ESSelector, ESCursor, ERenderer};
use guion::backend::Backend;
use guion::env::Env;
use guion::util::AsRefMut;
use guion::util::bounds::{Bounds, Offset};

pub mod imp;

pub struct Render<'s,E> where E: Env {
    piet: &'s mut Piet<'static>,
    window_handle: &'s WindowHandle,
    pub live_bounds: Bounds,
    pub live_viewport: Bounds,
    pub live_style: EStyle<E>,
    pub live_selector: ESSelector<E>,
    pub next_cursor: Option<ESCursor<E>>,
}
pub struct PietRef<'s> {
    piet: &'s mut Piet<'static>,
}

impl<'s> PietRef<'s> {
    pub fn new<'l>(piet: &'s mut Piet<'l>) -> Self where 'l: 's {
        //Self{piet: unsafe{&mut *(piet as &mut Piet<'l> as *mut Piet<'l> as *mut Piet<'static>)}}
        todo!()
    }

    pub unsafe fn get(&mut self) -> &mut Piet<'static> {
        //self.piet
        todo!()
    }
}

pub(crate) fn bounds2rect(b: Bounds) -> Rect {
    Rect{
        x0: b.x() as f64,
        y0: b.y() as f64,
        x1: b.x1() as f64,
        y1: b.y1() as f64,
    }
}

pub(crate) fn rect2bounds(r: Rect) -> Bounds {
    Bounds::from_xyxy(r.x0 as i32, r.y0 as i32, r.x1 as i32, r.y1 as i32)
}

pub(crate) fn offset2point(o: Offset) -> Point {
    Point{
        x: o.x as f64,
        y: o.y as f64,
    }
}

pub(crate) fn color2color(c: impl guion::style::color::Color) -> piet::Color {
    let [r,g,b,a] = c.into_rgba8();
    piet::Color::rgba8(r,g,b,a)
}

impl<'s,E> AsRefMut<Render<'s,E>> for Render<'s,E> where E: Env {
    fn as_ref(&self) -> &Render<'s,E> {
        todo!()
    }

    fn as_mut(&mut self) -> &mut Render<'s,E> {
        todo!()
    }
}
